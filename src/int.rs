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

/// A few methods that ease digits manipulation
pub trait Digits {
    /// get the list of digits of a number
    fn to_digits(&self) -> Vec<u8>;

    /// create a number from a list of digits
    fn from_digits(digits: &[u8]) -> Self;
}

/// A trait to get half of a value
pub trait Half {
    fn half(&self) -> Self;
}

macro_rules! int_digits_half_and_sqrt_for {
    ( $( $t:ty ),* ) => {
        $(
            impl Sqrt for $t {
                fn sqrt(&self) -> Self {
                    let s = (*self as f64).sqrt() as Self;
                    if (s+1)*(s+1) == *self { s+1 } else { s }
                }
            }

            impl Digits for $t {
                fn to_digits(&self) -> Vec<u8> {
                    let mut n = *self;
                    let mut v = Vec::new();
                    while n != 0 {
                        let t = n / 10;
                        v.push((n - 10*t) as u8);
                        n = t;
                    }
                    v
                }

                fn from_digits(digits: &[u8]) -> Self {
                    digits.iter().rev().fold(0, |a, &d| 10 * a + d as Self)
                }
            }

            impl Half for $t {
                fn half(&self) -> Self {
                    self >> 1
                }
            }
        )*
    }
}
int_digits_half_and_sqrt_for!(usize,u8,u16,u32,u64);

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
    use super::{Sqrt, Parity, Digits};

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

    #[test]
    fn test_to_primes() {
        let n = 1234567890123;
        assert_eq!(&n.to_digits(), &[3,2,1,0,9,8,7,6,5,4,3,2,1]);
        assert_eq!(usize::from_digits(&n.to_digits()), n);
    }
}

