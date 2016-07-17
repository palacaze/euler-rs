extern crate test;

use std::ops::Mul;
use std::cmp::Eq;

/// This module exposes a few tools related to integer manipulation
/// that are frequently encountered in euler problems

/// A trait for square root application for non floating-point numbers
pub trait Sqrt : Sized + Copy + Mul<Output=Self> + Eq {
    /// sqrt calculates the nearest square root of a number
    fn sqrt(&self) -> Self;

    /// indicates whether the number is a square
    fn is_square(&self) -> bool {
        let s = self.sqrt();
        s * s == *self
    }
}

macro_rules! int_sqrt_for {
    ( $( $t:ty ),* ) => {
        $(
            impl Sqrt for $t {
                fn sqrt(&self) -> Self {
                    let s = (*self as f64).sqrt() as Self;
                    if (s+1)*(s+1) == *self { s+1 } else { s }
                }
            }
        )*
    }
}
int_sqrt_for!(usize,u8,u16,u32,u64);

/// Parity trait
pub trait Parity {
    /// determines if a value is even
    fn is_even(&self) -> bool;

    /// determines if a value is odd
    fn is_odd(&self) -> bool {
        !self.is_even()
    }

}

macro_rules! parity_for {
    ( $( $t:ty ),* ) => {
        $(
            impl Parity for $t {
                fn is_even(&self) -> bool {
                    *self & 0x1 == 0
                }
            }
        )*
    }
}
parity_for!(usize,u8,u16,u32,u64,isize,i8,i16,i32,i64);


#[cfg(test)]
mod tests {
    use super::{Sqrt, Parity};

    #[test]
    fn test_square_root() {
        assert_eq!(1u64.sqrt(), 1);
        assert_eq!(2u64.sqrt(), 1);
        assert_eq!(5u64.sqrt(), 2);
        assert_eq!(144u32.sqrt(), 12);
    }

    #[test]
    fn test_is_square() {
        assert_eq!(1u64.is_square(), true);
        assert_eq!(49u32.is_square(), true);
        assert_eq!(48u16.is_square(), false);
    }

    #[test]
    fn test_parity() {
        assert!(1u64.is_odd());
        assert!(2u64.is_even());
        assert!(3i64.is_odd());
        assert!(4i64.is_even());
        assert!(0u8.is_even());
    }
}

