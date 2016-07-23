extern crate test;

use super::int::{Digits, Parity, Half};
use std::ops;
use std::cmp;
use std::fmt;
use std::char;
use std::convert;

/// A naïve and incomplete implementation of big integer
/// This is an experiment in order to learn Rust traits system
/// Storing digits in decimal in a vector is obviously not a good idea.
#[derive(Clone, PartialEq, Eq)]
pub struct BigUint(Vec<u8>);

impl BigUint {
    pub fn new() -> Self {
        BigUint::zero()
    }

    pub fn zero() -> Self() {
        BigUint(vec![0u8])
    }

    pub fn one() -> Self() {
        BigUint(vec![1u8])
    }
}

impl Default for BigUint {
    fn default() -> Self {
        BigUint::zero()
    }
}

impl fmt::Display for BigUint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.0.iter().rev()
            .map(|c| char::from_digit(*c as u32, 10).unwrap()).collect();
        write!(f, "{}", &s)
    }
}

impl fmt::Debug for BigUint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0.iter().rev().collect::<Vec<_>>())
    }
}

impl Digits for BigUint {
    fn to_digits(&self) -> Vec<u8> {
        self.0.clone()
    }

    fn from_digits(digits: &[u8]) -> Self {
        BigUint(digits.to_vec())
    }

}

impl Parity for BigUint {
    fn is_even(&self) -> bool {
        self.0[0].is_even()
    }
}

impl Half for BigUint {
    fn half(&self) -> Self {
        self / 2u32
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TryFromBigUintError(());

impl TryFromBigUintError {
    pub fn description(&self) -> &str {
        "out of range Big Integer type conversion attempted"
    }
}

impl fmt::Display for TryFromBigUintError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(fmt)
    }
}

macro_rules! common_biguint_ops_for {
    ( $( $t:ty ),* ) => {
        $(
            impl convert::From<$t> for BigUint {
                fn from(n: $t) -> Self {
                    BigUint(n.to_digits())
                }
            }

            impl<'a> convert::From<&'a $t> for BigUint {
                fn from(n: &'a $t) -> Self {
                    BigUint(n.to_digits())
                }
            }

            impl<'a> convert::TryFrom<&'a BigUint> for $t {
                type Err = TryFromBigUintError;
                fn try_from(u: &BigUint) -> Result<Self, Self::Err> {
                    let max = BigUint::from(<$t>::max_value());
                    if u > &max {
                        Err(TryFromBigUintError(()))
                    }
                    else {
                        Ok(<$t>::from_digits(&u.0))
                    }
                }
            }

            impl<'a> ops::Div<$t> for &'a BigUint {
                type Output = BigUint;
                fn div(self, rhs: $t) -> Self::Output {
                    let rhs = rhs as u64;
                    let mut v = Vec::new();
                    let mut carry = 0u64;

                    for d in self.0.iter().rev() {
                        let n = 10 * carry + *d as u64;
                        if n < rhs {
                            carry = n;
                            if !v.is_empty() {
                                v.insert(0, 0u8);
                            }
                            continue;
                        }

                        let r = n / rhs;
                        carry = n - rhs * r;
                        v.insert(0, r as u8);
                    }

                    BigUint(v)
                }
            }

            impl<'a> ops::Mul<$t> for &'a BigUint {
                type Output = BigUint;
                fn mul(self, rhs: $t) -> Self::Output {
                   &BigUint::from(rhs) * self
                }
            }

            impl ops::Mul<$t> for BigUint {
                type Output = BigUint;
                fn mul(self, rhs: $t) -> Self::Output {
                    &self * &BigUint::from(rhs)
                }
            }

            impl<'a> ops::Mul<&'a BigUint> for $t {
                type Output = BigUint;
                fn mul(self, rhs: &'a BigUint) -> Self::Output {
                    &BigUint::from(self) * rhs
                }
            }

            impl ops::Mul<BigUint> for $t {
                type Output = BigUint;
                fn mul(self, rhs: BigUint) -> Self::Output {
                    &rhs * &BigUint::from(self)
                }
            }

            impl<'a> ops::Add<$t> for &'a BigUint {
                type Output = BigUint;
                fn add(self, rhs: $t) -> Self::Output {
                    &BigUint::from(rhs) + self
                }
            }

            impl ops::Add<$t> for BigUint {
                type Output = BigUint;
                fn add(self, rhs: $t) -> Self::Output {
                    &self + &BigUint::from(rhs)
                }
            }

            impl<'a> ops::Add<&'a BigUint> for $t {
                type Output = BigUint;
                fn add(self, rhs: &'a BigUint) -> Self::Output {
                    &BigUint::from(self) + rhs
                }
            }

            impl ops::Add<BigUint> for $t {
                type Output = BigUint;
                fn add(self, rhs: BigUint) -> Self::Output {
                    &rhs + &BigUint::from(self)
                }
            }
        )*
    }
}
common_biguint_ops_for!(usize,u8,u16,u32,u64);

impl cmp::PartialOrd for BigUint {
    fn partial_cmp(&self, other: &BigUint) -> Option<cmp::Ordering> {
        let (ra, rb) = if self.0.len() > other.0.len() { (self,other) }
                       else { (other,self) };

        for (da, db) in ra.0.iter().zip(rb.0.iter().chain([0u8].iter().cycle()))  {
            if da < db {
                return Some(cmp::Ordering::Less);
            }
            if da > db {
                return Some(cmp::Ordering::Greater);
            }
        }

        Some(cmp::Ordering::Equal)
    }
}

impl<'a> ops::Add for &'a BigUint {
    type Output = BigUint;
    fn add(self, rhs: &BigUint) -> Self::Output {
        let (ra, rb) = if self.0.len() > rhs.0.len() { (self,rhs) } else { (rhs,self) };

        let len = cmp::max(ra.0.len(), rb.0.len()) + 1;
        let mut v = Vec::with_capacity(len);

        let mut carry = 0;

        for (da, db) in ra.0.iter().zip(rb.0.iter().chain([0u8].iter().cycle()))  {
            let d = da + db + carry;
            carry = d / 10;
            v.push((d % 10) as u8);
        }

        if carry > 0 {
            v.push(carry);
        }

        BigUint(v)
    }
}

impl<'a> ops::Add<&'a BigUint> for BigUint {
    type Output = BigUint;
    fn add(self, rhs: &'a BigUint) -> Self::Output {
        &self + rhs
    }
}
impl<'a> ops::Add<BigUint> for &'a BigUint {
    type Output = BigUint;
    fn add(self, rhs: BigUint) -> Self::Output {
        &rhs + self
    }
}
impl ops::Add for BigUint {
    type Output = BigUint;
    fn add(self, rhs: BigUint) -> Self::Output {
        &self + &rhs
    }
}

// ridiculously inefficient multiplication by additions
impl<'a> ops::Mul for &'a BigUint {
    type Output = BigUint;
    fn mul(self, rhs: &BigUint) -> Self::Output {
        // try to minimize work by setting b as the smaller number
        let (ra, rb) = if self.0.len() > rhs.0.len() { (self,rhs) } else { (rhs,self) };
        let mut b = rb.clone();
        let mut n = ra.clone();
        let mut r = BigUint::new();

        loop {
            if b.is_odd() {
                r = &r + &n;
                if &b == &BigUint::one() {
                    return r;
                }
            }

            b = b.half();
            n = &n + &n;
        }
    }
}

impl<'a> ops::Mul<&'a BigUint> for BigUint {
    type Output = BigUint;
    fn mul(self, rhs: &'a BigUint) -> Self::Output {
        &self * rhs
    }
}
impl<'a> ops::Mul<BigUint> for &'a BigUint {
    type Output = BigUint;
    fn mul(self, rhs: BigUint) -> Self::Output {
        &rhs * self
    }
}
impl ops::Mul for BigUint {
    type Output = BigUint;
    fn mul(self, rhs: BigUint) -> Self::Output {
        &self * &rhs
    }
}

#[cfg(test)]
mod tests {
    use super::{BigUint};

    #[test]
    fn test_from() {
        assert_eq!(BigUint::from(1u64).to_string(), "1");
        assert_eq!(BigUint::from(123456789u64).to_string(), "123456789");
    }
}

