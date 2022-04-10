use std::{
    io::{Read, Write},
    num::TryFromIntError,
};

use crate::Variable;

/// A signed integer value.
///
/// This type encodes values in the range `-2.pow(123)..2.pow(123)` by using the
/// first 5 bits to denote a signed byte `length`. This length ranges from
/// `-15..=15`. The number of bytes read is always absolute, but the sign of the
/// length is used to determine the overall sign of the encoded value. The
/// remaining 3 bits of the first byte and any additional bytes are then
/// used to store the integer in big-endian encoding.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Signed(i128);

impl Variable for Signed {
    fn encode_variable<W: Write>(&self, output: &mut W) -> std::io::Result<usize> {
        // We reserve 5 bits for a signed 4 bit number, ranging from -16..=15.
        let reserved = (self.0 as u128) >> 123;
        let check_bits = match reserved {
            0 => 0,
            0b11111 => 0xFF,
            _ => return Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        };

        let mut value = self.0.to_be_bytes();
        let (total_length, extra_bytes) = value
            .iter()
            .enumerate()
            .find_map(|(index, &byte)| {
                if byte == check_bits {
                    None
                } else {
                    let extra_bytes = 15 - index;
                    if byte >> 3 == check_bits >> 3 {
                        Some((extra_bytes + 1, extra_bytes))
                    } else {
                        Some((extra_bytes + 2, extra_bytes + 1))
                    }
                }
            })
            .unwrap_or((0, 0));
        let total_length = total_length.max(1);

        let length_header = if check_bits == 0 {
            extra_bytes + 2_usize.pow(4)
        } else {
            2_usize.pow(4) - extra_bytes - 1
        };

        // Clear the top bits to prepare for the header
        value[16 - total_length] &= 0b111;
        // Set the length bits
        value[16 - total_length] |= (length_header << 3) as u8;

        output.write_all(&value[16 - total_length..])?;
        Ok(total_length)
    }

    fn decode_variable<R: Read>(mut input: R) -> std::io::Result<Self> {
        let mut buffer = [0_u8; 16];
        input.read_exact(&mut buffer[0..1])?;

        let encoded_length = buffer[0] as usize >> 3;
        let (negative, length) = if encoded_length >= 2_usize.pow(4) {
            (false, encoded_length - 2_usize.pow(4))
        } else {
            (true, 2_usize.pow(4) - (encoded_length + 1))
        };

        input.read_exact(&mut buffer[16 - length..])?;

        if length < 15 {
            buffer[15 - length] |= buffer[0] & 0b111;
            if negative {
                buffer[15 - length] ^= 0b1111_1000;
            }
            buffer[0] = 0;
        } else {
            buffer[0] &= 0b111;
            if negative {
                buffer[0] ^= 0b1111_1000;
            }
        }

        if negative {
            for byte in &mut buffer[0..15 - length] {
                *byte ^= 0xFF;
            }
        }

        Ok(Self(i128::from_be_bytes(buffer)))
    }
}

macro_rules! impl_primitive_from_varint {
    ($ty:ty) => {
        impl TryFrom<Signed> for $ty {
            type Error = TryFromIntError;

            fn try_from(value: Signed) -> Result<Self, Self::Error> {
                value.0.try_into()
            }
        }
    };
}

macro_rules! impl_varint_from_primitive {
    ($ty:ty, $dest:ty) => {
        impl From<$ty> for Signed {
            fn from(value: $ty) -> Self {
                Self(<$dest>::from(value))
            }
        }
    };
}

impl_varint_from_primitive!(i8, i128);
impl_varint_from_primitive!(i16, i128);
impl_varint_from_primitive!(i32, i128);
impl_varint_from_primitive!(i64, i128);
impl_varint_from_primitive!(i128, i128);

impl_primitive_from_varint!(i8);
impl_primitive_from_varint!(i16);
impl_primitive_from_varint!(i32);
impl_primitive_from_varint!(i64);
impl_primitive_from_varint!(isize);

impl TryFrom<Signed> for i128 {
    type Error = TryFromIntError;

    fn try_from(value: Signed) -> Result<Self, Self::Error> {
        Ok(value.0)
    }
}

impl TryFrom<isize> for Signed {
    type Error = TryFromIntError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        Ok(Self(i128::try_from(value)?))
    }
}
