use std::io::{Read, Write};
use std::num::TryFromIntError;

use crate::Variable;

/// An unsigned integer value
///
/// This type encodes values in the range `0..2.pow(124)` by using the first 4
/// bits to denote an unsigned byte `length`. This length ranges from `0..=15`.
/// The remaining 4 bits of the first byte and any additional bytes are then
/// used to store the integer in big-endian encoding.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Unsigned(pub(crate) u128);

impl Unsigned {
    pub(crate) fn encode_be_bytes<W: Write, const N: usize>(
        mut value: [u8; N],
        mut output: W,
    ) -> std::io::Result<usize> {
        // Because we encode "extra bytes" in 4 bits, we must keep the extra
        // bytes to 15 or less. This only affects 128-bit encoding
        if N == 16 && value[0] >> 4 != 0 {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }

        let (total_length, extra_bytes) = value
            .iter()
            .enumerate()
            .find_map(|(index, &byte)| {
                if byte > 0 {
                    let extra_bytes = N - 1 - index;
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

        let extra_bytes_encoded = (extra_bytes as u8) << 4;
        if total_length > N {
            // We need an extra byte to store the length information
            output.write_all(&[extra_bytes_encoded])?;
            output.write_all(&value)?;
        } else {
            value[N - total_length] |= extra_bytes_encoded;
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
        let length = (first_byte >> 4) as usize;
        if length > N {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        input.read_exact(&mut buffer[N - length..])?;
        match N - length {
            0 => {
                // We overwrite the first byte with the read operation, so we need
                // to fill back in the bits from the first byte.
                buffer[0] |= first_byte & 0b1111;
            }
            1 => {
                // Clear the top 4 bits of the first byte. The lower 4 bits may
                // still have data in them.
                buffer[0] &= 0b1111;
            }
            _ => {
                // Move the first byte's data into the last byte read, then
                // clear our initial byte.
                buffer[N - 1 - length] |= first_byte & 0b1111;
                buffer[0] = 0;
            }
        }
        Ok(buffer)
    }
}

impl Variable for Unsigned {
    fn encode_variable<W: Write>(&self, output: W) -> std::io::Result<usize> {
        Self::encode_be_bytes(self.0.to_be_bytes(), output)
    }

    fn decode_variable<R: Read>(input: R) -> std::io::Result<Self> {
        let buffer = Self::decode_variable_bytes(input)?;

        Ok(Self(u128::from_be_bytes(buffer)))
    }
}

macro_rules! impl_primitive_from_varint {
    ($ty:ty) => {
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

impl_primitive_from_varint!(u8);
impl_primitive_from_varint!(u16);
impl_primitive_from_varint!(u32);
impl_primitive_from_varint!(u64);
impl_primitive_from_varint!(usize);

impl From<Unsigned> for u128 {
    fn from(value: Unsigned) -> Self {
        value.0
    }
}

impl From<usize> for Unsigned {
    fn from(value: usize) -> Self {
        Self(value as u128)
    }
}
