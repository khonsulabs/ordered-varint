use std::io::{Read, Write};
use std::num::TryFromIntError;

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

impl Signed {
    pub(crate) fn encode_be_bytes<W: Write, const N: usize>(
        mut value: [u8; N],
        mut output: W,
    ) -> std::io::Result<usize> {
        let check_bits = if N == 16 {
            // We reserve 5 bits for a signed 4 bit number, ranging from -16..=15.
            let reserved = value[0] >> 3;
            match reserved {
                0 => 0,
                0b11111 => 0xFF,
                _ => return Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
            }
        } else if value[0] >> 7 == 0 {
            // positive
            0
        } else {
            // negative
            0xff
        };

        let (total_length, extra_bytes) = value
            .iter()
            .enumerate()
            .find_map(|(index, &byte)| {
                if byte == check_bits {
                    None
                } else {
                    let extra_bytes = N - 1 - index;
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

        let encoded_length_header = (length_header as u8) << 3;
        if total_length > N {
            // We can't fit the length in the buffer.
            output.write_all(&[encoded_length_header | (check_bits >> 5)])?;
            output.write_all(&value)?;
        } else {
            // Clear the top bits to prepare for the header
            value[N - total_length] &= 0b111;
            // Set the length bits
            value[N - total_length] |= encoded_length_header;

            output.write_all(&value[N - total_length..])?;
        }

        Ok(total_length)
    }

    pub(crate) fn decode_variable_bytes<R: Read, const N: usize>(
        mut input: R,
    ) -> std::io::Result<[u8; N]> {
        let mut buffer = [0_u8; N];
        input.read_exact(&mut buffer[0..1])?;
        let first_byte = buffer[0];
        let encoded_length = first_byte as usize >> 3;
        let (negative, length) = if encoded_length >= 2_usize.pow(4) {
            (false, encoded_length - 2_usize.pow(4))
        } else {
            (true, 2_usize.pow(4) - (encoded_length + 1))
        };
        if length > N {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }

        input.read_exact(&mut buffer[N - length..])?;

        match N - length {
            0 => {
                // We overwrote our first byte, but the first byte has some 3
                // bits of data we need to preserve.
                let mut first_bits = first_byte & 0b111;
                if negative {
                    first_bits ^= 0b111;
                }
                buffer[0] |= first_bits << 5;
            }
            1 => {
                // Clear the top 3 bits of the top byte, and negate if needed.
                buffer[0] &= 0b111;
                if negative {
                    buffer[0] ^= 0b1111_1000;
                }
            }
            _ => {
                buffer[N - 1 - length] |= first_byte & 0b111;
                if negative {
                    buffer[N - 1 - length] ^= 0b1111_1000;
                }
                buffer[0] = 0;
            }
        }

        if negative && N > 1 {
            let bytes_to_negate = N - length;
            // We know we can skip updating the last byte that contained data.
            if bytes_to_negate > 1 {
                for byte in &mut buffer[0..bytes_to_negate - 1] {
                    *byte ^= 0xFF;
                }
            }
        }

        Ok(buffer)
    }
}

impl Variable for Signed {
    fn encode_variable<W: Write>(&self, output: W) -> std::io::Result<usize> {
        Self::encode_be_bytes(self.0.to_be_bytes(), output)
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

impl From<Signed> for i128 {
    fn from(value: Signed) -> Self {
        value.0
    }
}

impl From<isize> for Signed {
    fn from(value: isize) -> Self {
        Self(value as i128)
    }
}
