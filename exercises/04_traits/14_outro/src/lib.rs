// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SaturatingU16 {
    value: u16,
}

impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.value.saturating_add(rhs.value).into()
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: u16) -> Self {
        self.value.saturating_add(rhs).into()
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {
        self.value.saturating_add((*rhs).value).into()
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: &u16) -> Self {
        self.value.saturating_add(*rhs).into()
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

macro_rules! conversion {
    ($from:ty) => {
        impl From<$from> for SaturatingU16 {
            fn from(value: $from) -> Self {
                Self {
                    value: value as u16,
                }
            }
        }
    };
}

macro_rules! conversionRef {
    ($from:ty) => {
        impl From<$from> for SaturatingU16 {
            fn from(value: $from) -> Self {
                Self {
                    value: *value as u16,
                }
            }
        }
    };
}

conversion!(u16);
conversion!(u8);
conversionRef!(&u16);
conversionRef!(&u8);
