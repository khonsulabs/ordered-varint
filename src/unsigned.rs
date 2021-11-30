use std::{
    io::{Read, Write},
    num::TryFromIntError,
};

use crate::Variable;

/// An unsigned integer value
///
/// This type encodes values in the range `0..2.pow(124)` by using the first 4
/// bits to denote an unsigned byte `length`. This length ranges from `0..=15`.
/// The remaining 4 bits of the first byte and any additional bytes are then
/// used to store the integer in big-endian encoding.
pub struct Unsigned(pub(crate) u128);

impl Variable for Unsigned {
    fn encode<W: Write>(&self, output: &mut W) -> std::io::Result<usize> {
        // We reserve the top 4 bits for the length information.
        if self.0 >> 124 != 0 {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }

        let mut value = self.0.to_be_bytes();
        let (total_length, extra_bytes) = value
            .iter()
            .enumerate()
            .find_map(|(index, &byte)| {
                if byte > 0 {
                    let extra_bytes = 15 - index;
                    if byte < 16 {
                        Some((extra_bytes + 1, extra_bytes))
                    } else {
                        Some((extra_bytes + 2, extra_bytes + 1))
                    }
                } else {
                    None
                }
            })
            .unwrap_or((0, 0));
        let total_length = total_length.max(1);

        value[16 - total_length] |= (extra_bytes as u8) << 4;

        output.write_all(&value[16 - total_length..])?;
        Ok(total_length)
    }

    fn decode<R: Read>(mut input: R) -> std::io::Result<Self> {
        let mut buffer = [0_u8; 16];
        input.read_exact(&mut buffer[0..1])?;
        let length = (buffer[0] >> 4) as usize;
        input.read_exact(&mut buffer[16 - length as usize..])?;
        if length < 15 {
            buffer[15 - length] |= buffer[0] & 0b1111;
            buffer[0] = 0;
        } else {
            buffer[0] &= 0b1111;
        }

        Ok(Self(u128::from_be_bytes(buffer)))
    }
}

macro_rules! impl_primitive_from_varint {
    ($ty:ty,  $dest:ty) => {
        impl TryFrom<Unsigned> for $ty {
            type Error = TryFromIntError;

            fn try_from(value: Unsigned) -> Result<Self, Self::Error> {
                value.0.try_into()
            }
        }
    };
}

macro_rules! impl_varint_from_primitive {
    ($ty:ty, $dest:ty) => {
        impl From<$ty> for Unsigned {
            fn from(value: $ty) -> Self {
                Self(<$dest>::from(value))
            }
        }
    };
}

impl_varint_from_primitive!(u8, u128);
impl_varint_from_primitive!(u16, u128);
impl_varint_from_primitive!(u32, u128);
impl_varint_from_primitive!(u64, u128);
impl_varint_from_primitive!(u128, u128);

impl_primitive_from_varint!(u8, u128);
impl_primitive_from_varint!(u16, u128);
impl_primitive_from_varint!(u32, u128);
impl_primitive_from_varint!(u64, u128);

impl TryFrom<Unsigned> for u128 {
    type Error = TryFromIntError;

    fn try_from(value: Unsigned) -> Result<Self, Self::Error> {
        Ok(value.0)
    }
}
