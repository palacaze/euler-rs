extern crate test;

use std::ops::Mul;
use std::cmp::Eq;
use std::mem;

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

/// a tag that represents a set of digits.
/// All the permutations of these digits will produce the same tag
/// with 6 bits per digit we can handle numbers up to 127 digits long
pub trait PermutTag {
    fn permut_tag(&self) -> u64;
}

/// Determine greatest common divisor
pub trait Gcd {
    fn gcd(self, other: Self) -> Self;
    fn euclidean_gcd(self, other: Self) -> Self;
}

macro_rules! uint_traits_impl {
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

            impl PermutTag for $t {
                fn permut_tag(&self) -> u64 {
                    let mut n = *self as u64;
                    let mut d: u64 = 0;
                    while n != 0 {
                        d += 1 << (6 * (n % 10));
                        n /= 10;
                    }
                    d
                }
            }

            impl Gcd for $t {
                // use Stein's algorithm
                fn gcd(self, other: Self) -> Self {
                    let (mut a, mut b) = if self > other {
                        (self, other)
                    } else {
                        (other, self)
                    };

                    if a == 0 || b == 0 { return a | b; }

                    // find common factors of 2
                    let shift = (a | b).trailing_zeros();

                    // divide n and m by 2 until odd
                    // m inside loop
                    b >>= b.trailing_zeros();

                    while a != 0 {
                        a >>= a.trailing_zeros();
                        if b > a { mem::swap(&mut b, &mut a) }
                        a -= b;
                    }

                    b << shift
                }

                fn euclidean_gcd(self, other: Self) -> Self {
                    let (mut a, mut b) = (self, other);
                    while a != 0 {
                        mem::swap(&mut a, &mut b);
                        a %= b;
                    }
                    b
                }
            }
        )*
    }
}
uint_traits_impl!(usize,u8,u16,u32,u64);

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
    use super::{Sqrt, Parity, Digits, Gcd};

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
    fn test_gcd() {
        assert_eq!(0, 0u8.gcd(0));
        assert_eq!(10, 10u8.gcd(0));
        assert_eq!(10, 0u8.gcd(10));
        assert_eq!(10, 10u8.gcd(20));
        assert_eq!(44, 2024u32.gcd(748));

        for i in 0..1000u32 {
            for j in 0..1000u32 {
                assert_eq!(i.gcd(j), i.euclidean_gcd(j));
            }
        }
    }

    #[test]
    fn test_to_primes() {
        let n = 1234567890123;
        assert_eq!(&n.to_digits(), &[3,2,1,0,9,8,7,6,5,4,3,2,1]);
        assert_eq!(usize::from_digits(&n.to_digits()), n);
    }
}

