#![doc= include_str!("../.rustme/docs.md")]
#![forbid(unsafe_code)]
#![warn(
    clippy::cargo,
    missing_docs,
    // clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    future_incompatible,
    rust_2018_idioms,
)]
#![cfg_attr(doc, allow(unknown_lints), warn(rustdoc::all))] // https://github.com/rust-lang/rust/pull/106316
#![allow(
    clippy::missing_errors_doc, // TODO clippy::missing_errors_doc
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
)]

mod signed;
mod unsigned;

use std::io::{Read, Write};

pub use self::signed::*;
pub use self::unsigned::*;

/// Encodes and decodes a type using a variable-length format.
pub trait Variable: Sized {
    /// Encodes `self` into `destination`, returning the number of bytes written upon success.
    fn encode_variable<W: Write>(&self, destination: W) -> std::io::Result<usize>;
    /// Decodes a variable length value from `source`.
    fn decode_variable<R: Read>(source: R) -> std::io::Result<Self>;

    /// Encodes `self` into a new `Vec<u8>`.
    fn to_variable_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut output = Vec::with_capacity(16);
        self.encode_variable(&mut output)?;
        Ok(output)
    }
}

macro_rules! impl_primitive_variable {
    ($ty:ty,  $dest:ty) => {
        impl Variable for $ty {
            fn encode_variable<W: Write>(&self, destination: W) -> std::io::Result<usize> {
                <$dest>::encode_be_bytes(self.to_be_bytes(), destination)
            }

            fn decode_variable<R: Read>(source: R) -> std::io::Result<Self> {
                <$dest>::decode_variable_bytes(source).map(<Self>::from_be_bytes)
            }
        }
    };
}

impl_primitive_variable!(u8, Unsigned);
impl_primitive_variable!(u16, Unsigned);
impl_primitive_variable!(u32, Unsigned);
impl_primitive_variable!(u64, Unsigned);
impl_primitive_variable!(u128, Unsigned);
impl_primitive_variable!(usize, Unsigned);

impl_primitive_variable!(i8, Signed);
impl_primitive_variable!(i16, Signed);
impl_primitive_variable!(i32, Signed);
impl_primitive_variable!(i64, Signed);
impl_primitive_variable!(i128, Signed);
impl_primitive_variable!(isize, Signed);

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    trait TestType: Variable {
        type Variable: From<Self> + TryInto<Self> + Variable + Eq + Debug;
    }

    macro_rules! impl_test_type {
        ($name:ident, $kind:ident) => {
            impl TestType for $name {
                type Variable = $kind;
            }
        };
    }

    impl_test_type!(u8, Unsigned);
    impl_test_type!(u16, Unsigned);
    impl_test_type!(u32, Unsigned);
    impl_test_type!(u64, Unsigned);
    impl_test_type!(u128, Unsigned);
    impl_test_type!(usize, Unsigned);

    impl_test_type!(i8, Signed);
    impl_test_type!(i16, Signed);
    impl_test_type!(i32, Signed);
    impl_test_type!(i64, Signed);
    impl_test_type!(i128, Signed);
    impl_test_type!(isize, Signed);

    fn roundtrip<T: TestType + Eq + Debug + Copy>(value: T, expected_bytes: usize) {
        let mut bytes = Vec::new();
        let encoded_length = value.encode_variable(&mut bytes).unwrap();
        println!("Encoded {value:?} to {bytes:02x?}");
        assert_eq!(
            encoded_length, expected_bytes,
            "expected {expected_bytes} encoded bytes, got {encoded_length}"
        );
        assert_eq!(
            encoded_length,
            bytes.len(),
            "vec has more bytes than returned value"
        );
        let decoded = T::decode_variable(&bytes[..]).unwrap();
        assert_eq!(
            decoded, value,
            "decoded value did not match: {value:?} vs {decoded:?}",
        );

        // Because we now decode and encode directly, we also need to test using
        // Signed/Unsigned
        let variable = <T::Variable as From<T>>::from(value);
        let mut bytes = Vec::new();
        let encoded_length = variable.encode_variable(&mut bytes).unwrap();
        assert_eq!(
            encoded_length, expected_bytes,
            "expected {expected_bytes} encoded bytes, got {encoded_length}"
        );
        assert_eq!(
            encoded_length,
            bytes.len(),
            "vec has more bytes than returned value"
        );
        let decoded = <T::Variable as Variable>::decode_variable(&bytes[..]).unwrap();
        assert_eq!(
            decoded, variable,
            "decoded value did not match: {variable:?} vs {decoded:?}",
        );
    }

    #[test]
    fn roundtrip_u8() {
        roundtrip(u8::MIN, 1);
        roundtrip(2_u8.pow(4) - 1, 1);
        roundtrip(2_u8.pow(4), 2);
    }

    #[test]
    fn roundtrip_i8() {
        roundtrip(0_i8, 1);
        roundtrip(2_i8.pow(3) - 1, 1);
        roundtrip(-(2_i8.pow(3)), 1);

        roundtrip(2_i8.pow(3), 2);
        roundtrip(-(2_i8.pow(3) + 1), 2);

        roundtrip(-1_i8, 1);
    }

    #[test]
    fn roundtrip_u16() {
        roundtrip(u16::from(u8::MAX), 2);
        roundtrip(2_u16.pow(12) - 1, 2);
        roundtrip(2_u16.pow(12), 3);
    }

    #[test]
    fn roundtrip_i16() {
        roundtrip(i16::from(i8::MAX), 2);
        roundtrip(i16::from(i8::MIN), 2);
        roundtrip(2_i16.pow(11) - 1, 2);
        roundtrip(-(2_i16.pow(11)), 2);

        roundtrip(2_i16.pow(11), 3);
        roundtrip(-(2_i16.pow(11) + 1), 3);

        roundtrip(-1_i16, 1);
    }

    #[test]
    fn roundtrip_u32() {
        roundtrip(u32::from(u16::MAX), 3);
        roundtrip(2_u32.pow(20) - 1, 3);
        roundtrip(2_u32.pow(20), 4);
        roundtrip(2_u32.pow(28) - 1, 4);
        roundtrip(2_u32.pow(28), 5);
    }

    #[test]
    fn roundtrip_i32() {
        roundtrip(i32::from(i16::MAX), 3);
        roundtrip(i32::from(i16::MIN), 3);
        roundtrip(2_i32.pow(19) - 1, 3);
        roundtrip(-(2_i32.pow(19)), 3);
        roundtrip(2_i32.pow(19), 4);
        roundtrip(-(2_i32.pow(19) + 1), 4);

        roundtrip(2_i32.pow(27), 5);
        roundtrip(-(2_i32.pow(27) + 1), 5);

        roundtrip(-1_i32, 1);
    }

    #[test]
    fn roundtrip_u64() {
        roundtrip(u64::from(u32::MAX), 5);
        roundtrip(2_u64.pow(36) - 1, 5);
        roundtrip(2_u64.pow(36), 6);
        roundtrip(2_u64.pow(44) - 1, 6);
        roundtrip(2_u64.pow(44), 7);
        roundtrip(2_u64.pow(52) - 1, 7);
        roundtrip(2_u64.pow(52), 8);
        roundtrip(2_u64.pow(60) - 1, 8);
        roundtrip(2_u64.pow(60), 9);
    }

    #[test]
    fn roundtrip_i64() {
        roundtrip(i64::from(i32::MAX), 5);
        roundtrip(i64::from(i32::MIN), 5);
        roundtrip(2_i64.pow(35) - 1, 5);
        roundtrip(-(2_i64.pow(35)), 5);
        roundtrip(2_i64.pow(35), 6);
        roundtrip(-(2_i64.pow(35) + 1), 6);

        roundtrip(2_i64.pow(43), 7);
        roundtrip(-(2_i64.pow(43) + 1), 7);

        roundtrip(2_i64.pow(51), 8);
        roundtrip(-(2_i64.pow(51) + 1), 8);

        roundtrip(2_i64.pow(59), 9);
        roundtrip(-(2_i64.pow(59) + 1), 9);

        roundtrip(-1_i64, 1);
    }

    #[test]
    fn roundtrip_u128() {
        roundtrip(u128::from(u64::MAX), 9);
        roundtrip(2_u128.pow(68) - 1, 9);
        roundtrip(2_u128.pow(68), 10);
        roundtrip(2_u128.pow(76) - 1, 10);
        roundtrip(2_u128.pow(76), 11);
        roundtrip(2_u128.pow(84) - 1, 11);
        roundtrip(2_u128.pow(84), 12);
        roundtrip(2_u128.pow(92) - 1, 12);
        roundtrip(2_u128.pow(92), 13);
        roundtrip(2_u128.pow(100) - 1, 13);
        roundtrip(2_u128.pow(100), 14);
        roundtrip(2_u128.pow(108) - 1, 14);
        roundtrip(2_u128.pow(108), 15);
        roundtrip(2_u128.pow(116) - 1, 15);
        roundtrip(2_u128.pow(116), 16);

        // Maximum value
        roundtrip(2_u128.pow(124) - 1, 16);

        // Above maximum value
        assert!(2_u128.pow(124).encode_variable(&mut Vec::new()).is_err());
    }

    #[test]
    fn roundtrip_i128() {
        roundtrip(i128::from(i64::MAX), 9);
        roundtrip(i128::from(i64::MIN), 9);
        roundtrip(2_i128.pow(67) - 1, 9);
        roundtrip(-(2_i128.pow(67)), 9);
        roundtrip(2_i128.pow(67), 10);
        roundtrip(-(2_i128.pow(67) + 1), 10);

        roundtrip(2_i128.pow(75), 11);
        roundtrip(-(2_i128.pow(75) + 1), 11);

        roundtrip(2_i128.pow(83), 12);
        roundtrip(-(2_i128.pow(83) + 1), 12);

        roundtrip(2_i128.pow(91), 13);
        roundtrip(-(2_i128.pow(91) + 1), 13);

        roundtrip(2_i128.pow(99), 14);
        roundtrip(-(2_i128.pow(99) + 1), 14);

        roundtrip(2_i128.pow(107), 15);
        roundtrip(-(2_i128.pow(107) + 1), 15);

        roundtrip(2_i128.pow(115), 16);
        roundtrip(-(2_i128.pow(115) + 1), 16);

        // Maximum value
        roundtrip(2_i128.pow(123) - 1, 16);
        // Minimum value
        roundtrip(-(2_i128.pow(123)), 16);

        // Above maximum
        assert!(2_i128.pow(123).encode_variable(&mut Vec::new()).is_err());
        // Below minimum
        assert!((-(2_i128.pow(123)) - 1)
            .encode_variable(&mut Vec::new())
            .is_err());
    }

    #[test]
    fn roundtrip_sizes() {
        // This test is minimal due to *size types being dependent on the
        // architecture limits.
        roundtrip(usize::MAX, std::mem::size_of::<usize>() + 1);
        roundtrip(usize::MIN, 1);
        roundtrip(isize::MAX, std::mem::size_of::<isize>() + 1);
        roundtrip(isize::MIN, std::mem::size_of::<isize>() + 1);
        roundtrip(0_isize, 1);
    }

    #[test]
    fn conversions() {
        assert_eq!(
            isize::try_from(Signed::from(isize::MAX)).unwrap(),
            isize::MAX
        );
        assert_eq!(
            usize::try_from(Unsigned::from(usize::MAX)).unwrap(),
            usize::MAX
        );
        assert_eq!(i128::try_from(Signed::from(i128::MAX)).unwrap(), i128::MAX);
        assert_eq!(
            usize::try_from(Unsigned::from(usize::MAX)).unwrap(),
            usize::MAX
        );
    }

    #[test]
    fn test_signed_ordering() {
        let mut entries = Vec::new();
        for i in i16::MIN..=i16::MAX {
            println!("{} => {:02X?}", i, i.to_variable_vec().unwrap());
            entries.push(i.to_variable_vec().unwrap());
        }
        let originals = entries.clone();
        entries.sort();
        assert_eq!(originals, entries);
    }

    #[test]
    fn test_unsigned_ordering() {
        let mut entries = Vec::new();
        for i in u16::MIN..=u16::MAX {
            println!("{} => {:02X?}", i, i.to_variable_vec().unwrap());
            entries.push(i.to_variable_vec().unwrap());
        }
        let originals = entries.clone();
        entries.sort();
        assert_eq!(originals, entries);
    }

    #[test]
    fn overflow_decode() {
        let unsigned_max = u64::MAX.to_variable_vec().unwrap();
        u32::decode_variable(&unsigned_max[..]).expect_err("u32 should overflow");
        let signed_min = i64::MIN.to_variable_vec().unwrap();
        i32::decode_variable(&signed_min[..]).expect_err("i32 should overflow");
        let signed_max = i64::MAX.to_variable_vec().unwrap();
        i32::decode_variable(&signed_max[..]).expect_err("i32 should overflow");
    }
}
