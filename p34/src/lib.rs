#![feature(bigint_helper_methods)]
use std::ops::{Add, Mul, Sub};
use std::ops::{AddAssign, MulAssign, SubAssign};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BigUInt4096 {
    data: [u64; 64],
}

impl Default for BigUInt4096 {
    fn default() -> Self {
        BigUInt4096 { data: [0; 64] }
    }
}

impl BigUInt4096 {
    pub fn new() -> Self {
        BigUInt4096 { data: [0; 64] }
    }

    fn carrying_add(&self, other: &Self, mut carry: bool) -> (Self, bool) {
        let mut result = BigUInt4096::new();

        for i in 0..64 {
            (result.data[i], carry) = self.data[i].carrying_add(other.data[i], carry);
        }

        (result, carry)
    }

    fn overflowing_add(&self, other: &Self) -> (Self, bool) {
        self.carrying_add(other, false)
    }

    fn wrapping_add(&self, other: &Self) -> Self {
        let (result, _) = self.overflowing_add(other);
        result
    }

    pub fn checked_add(&self, other: &Self) -> Option<Self> {
        let (result, overflow) = self.overflowing_add(other);
        if overflow {
            None
        } else {
            Some(result)
        }
    }

    fn strict_add(&self, other: &Self) -> Self {
        let (result, overflow) = self.overflowing_add(other);
        if overflow {
            panic!("Overflow in addition");
        }
        result
    }

    fn borrowing_sub(&self, other: &Self, mut borrow: bool) -> (Self, bool) {
        let mut result = BigUInt4096::new();

        for i in 0..64 {
            (result.data[i], borrow) = self.data[i].borrowing_sub(other.data[i], borrow);
        }

        (result, borrow)
    }

    fn overflowing_sub(&self, other: &Self) -> (Self, bool) {
        self.borrowing_sub(other, false)
    }

    fn wrapping_sub(&self, other: &Self) -> Self {
        let (result, _) = self.overflowing_sub(other);
        result
    }

    pub fn checked_sub(&self, other: &Self) -> Option<Self> {
        let (result, overflow) = self.overflowing_sub(other);
        if overflow {
            None
        } else {
            Some(result)
        }
    }

    pub fn strict_sub(&self, other: &Self) -> Self {
        let (result, overflow) = self.overflowing_sub(other);
        if overflow {
            panic!("Overflow in subtraction");
        }
        result
    }

    fn carrying_mul_by_u64(&self, other: u64, mut carry: u64) -> (Self, u64) {
        let mut result = BigUInt4096::new();

        for i in 0..64 {
            (result.data[i], carry) = self.data[i].carrying_mul(other, carry);
        }

        (result, carry)
    }

    fn overflowing_mul_by_u64(&self, other: u64) -> (Self, u64) {
        self.carrying_mul_by_u64(other, 0)
    }

    pub fn wrapping_mul_by_u64(&self, other: u64) -> Self {
        let (result, _) = self.overflowing_mul_by_u64(other);
        result
    }

    fn overflowing_shift_left(&self, shift_in_blocks: usize) -> (Self, Self) {
        let mut result = BigUInt4096::new();
        let mut overflow = BigUInt4096::new();

        for i in shift_in_blocks..64 {
            result.data[i] = self.data[i - shift_in_blocks];
        }

        for i in 64..64 + shift_in_blocks {
            if i < 128 {
                overflow.data[i - 64] = self.data[i - shift_in_blocks];
            }
        }

        (result, overflow)
    }

    pub fn widening_shl(&self, shift_in_bits: usize) -> (Self, Self) {
        let mut result;
        let mut overflow;
        let mut carry: u64 = 0;
        let mut temp: u64;

        let shift_in_blocks = shift_in_bits / 64;
        let shift_in_bits = shift_in_bits % 64;

        (result, overflow) = self.overflowing_shift_left(shift_in_blocks);

        for i in 0..64 {
            temp = carry;
            carry = result.data[i] >> (64 - shift_in_bits);
            result.data[i] <<= shift_in_bits;
            result.data[i] |= temp;
        }
        carry = 0;
        for i in 0..64 {
            temp = carry;
            carry = overflow.data[i] >> (64 - shift_in_bits);
            overflow.data[i] <<= shift_in_bits;
            overflow.data[i] |= temp;
        }

        (result, overflow)
    }

    fn from_u64_shifted(block: u64, shift_in_blocks: usize) -> Self {
        let mut result = BigUInt4096::new();
        result.data[shift_in_blocks] = block;
        result
    }

    fn carrying_mul(&self, other: &Self, carry: &mut Self) -> (Self, Self) {
        let mut result = BigUInt4096::new();
        let mut overflow;
        let mut temp_r: BigUInt4096;
        let mut temp_o: BigUInt4096;
        let mut temp_carry = BigUInt4096::new();
        let mut temp_bool: bool;

        for i in 0..64 {
            (temp_r, overflow) = self.overflowing_mul_by_u64(other.data[i]);
            (temp_r, temp_o) = temp_r.overflowing_shift_left(i);
            temp_o = temp_o.strict_add(&BigUInt4096::from_u64_shifted(overflow, i));
            (result, temp_bool) = result.overflowing_add(&temp_r);
            temp_o = temp_o.strict_add(&BigUInt4096::from_u64_shifted(temp_bool as u64, 0));
            temp_carry = temp_carry.strict_add(&temp_o);
        }

        (result, temp_bool) = result.overflowing_add(carry);
        temp_carry = temp_carry.strict_add(&BigUInt4096::from_u64_shifted(temp_bool as u64, 0));
        (result, temp_carry)
    }

    fn overflowing_mul(&self, other: &Self) -> (Self, Self) {
        let mut carry = BigUInt4096::new();
        self.carrying_mul(other, &mut carry)
    }

    fn wrapping_mul(&self, other: &Self) -> Self {
        let (result, _) = self.overflowing_mul(other);
        result
    }

    pub fn checked_mul(&self, other: &Self) -> Option<Self> {
        let (result, overflow) = self.overflowing_mul(other);
        if overflow != BigUInt4096::default() {
            None
        } else {
            Some(result)
        }
    }

    pub fn strict_mul(&self, other: &Self) -> Self {
        let (result, overflow) = self.overflowing_mul(other);
        if overflow != BigUInt4096::default() {
            panic!("Overflow in multiplication");
        }
        result
    }
}

impl FromStr for BigUInt4096 {
    type Err = std::num::ParseIntError;
    fn from_str(hex_str: &str) -> Result<Self, Self::Err> {
        if hex_str.is_empty() {
            return Ok(BigUInt4096::default());
        }
        let mut result = BigUInt4096::new();
        let hex_str = hex_str.trim_start_matches("0x");
        let hex_str = format!("{:0>64}", hex_str);
        for i in 0..64 {
            let start = hex_str.len() - (i + 1) * 16;
            let end = hex_str.len() - i * 16;
            let block_str = &hex_str[start..end];
            result.data[i] = u64::from_str_radix(block_str, 16)?;
        }
        Ok(result)
    }
}

impl Add for BigUInt4096 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.wrapping_add(&other)
    }
}

impl Add<&Self> for BigUInt4096 {
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        self.wrapping_add(other)
    }
}

impl Add for &BigUInt4096 {
    type Output = BigUInt4096;

    fn add(self, other: Self) -> Self::Output {
        self.wrapping_add(other)
    }
}

impl Add<BigUInt4096> for &BigUInt4096 {
    type Output = BigUInt4096;

    fn add(self, other: BigUInt4096) -> Self::Output {
        self.wrapping_add(&other)
    }
}

impl AddAssign for BigUInt4096 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl AddAssign<&Self> for BigUInt4096 {
    fn add_assign(&mut self, other: &Self) {
        *self = *self + *other;
    }
}

impl Sub for BigUInt4096 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.wrapping_sub(&other)
    }
}

impl Sub<&Self> for BigUInt4096 {
    type Output = Self;

    fn sub(self, other: &Self) -> Self::Output {
        self.wrapping_sub(other)
    }
}

impl Sub for &BigUInt4096 {
    type Output = BigUInt4096;

    fn sub(self, other: Self) -> Self::Output {
        self.wrapping_sub(other)
    }
}

impl Sub<BigUInt4096> for &BigUInt4096 {
    type Output = BigUInt4096;

    fn sub(self, other: BigUInt4096) -> Self::Output {
        self.wrapping_sub(&other)
    }
}

impl SubAssign for BigUInt4096 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl SubAssign<&Self> for BigUInt4096 {
    fn sub_assign(&mut self, other: &Self) {
        *self = *self - *other;
    }
}

impl Mul for BigUInt4096 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.wrapping_mul(&other)
    }
}

impl Mul<&Self> for BigUInt4096 {
    type Output = Self;

    fn mul(self, other: &Self) -> Self::Output {
        self.wrapping_mul(other)
    }
}

impl Mul for &BigUInt4096 {
    type Output = BigUInt4096;

    fn mul(self, other: Self) -> Self::Output {
        self.wrapping_mul(other)
    }
}

impl Mul<BigUInt4096> for &BigUInt4096 {
    type Output = BigUInt4096;

    fn mul(self, other: BigUInt4096) -> Self::Output {
        self.wrapping_mul(&other)
    }
}

impl MulAssign for BigUInt4096 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl MulAssign<&Self> for BigUInt4096 {
    fn mul_assign(&mut self, other: &Self) {
        *self = *self * *other;
    }
}

impl std::fmt::Display for BigUInt4096 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hex_str = String::new();
        for i in (0..64).rev() {
            hex_str.push_str(&format!("{:016x}", self.data[i]));
        }
        write!(f, "0x{}", hex_str)
    }
}
