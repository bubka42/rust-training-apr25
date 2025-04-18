use std::ops::{Add, Mul, Sub};
use std::ops::{AddAssign, MulAssign, SubAssign};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BigUInt<const N: usize> {
    pub data: [u64; N],
}

impl<const N: usize> Default for BigUInt<N> {
    fn default() -> Self {
        BigUInt { data: [0; N] }
    }
}

impl<const N: usize> BigUInt<N> {
    pub fn new() -> Self {
        BigUInt { data: [0; N] }
    }

    pub fn carrying_add(&self, other: &Self, mut carry: bool) -> (Self, bool) {
        let mut result = BigUInt::<N>::new();

        for i in 0..N {
            (result.data[i], carry) = self.data[i].carrying_add(other.data[i], carry);
        }

        (result, carry)
    }

    pub fn overflowing_add(&self, other: &Self) -> (Self, bool) {
        self.carrying_add(other, false)
    }

    pub fn wrapping_add(&self, other: &Self) -> Self {
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

    pub fn strict_add(&self, other: &Self) -> Self {
        let (result, overflow) = self.overflowing_add(other);
        if overflow {
            panic!("Overflow in addition");
        }
        result
    }

    pub fn borrowing_sub(&self, other: &Self, mut borrow: bool) -> (Self, bool) {
        let mut result = BigUInt::<N>::new();

        for i in 0..N {
            (result.data[i], borrow) = self.data[i].borrowing_sub(other.data[i], borrow);
        }

        (result, borrow)
    }

    pub fn overflowing_sub(&self, other: &Self) -> (Self, bool) {
        self.borrowing_sub(other, false)
    }

    pub fn wrapping_sub(&self, other: &Self) -> Self {
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

    pub fn carrying_mul_by_u64(&self, other: u64, mut carry: u64) -> (Self, u64) {
        let mut result = BigUInt::<N>::new();

        for i in 0..N {
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
        let mut result = BigUInt::<N>::new();
        let mut overflow = BigUInt::<N>::new();

        for i in shift_in_blocks..N {
            result.data[i] = self.data[i - shift_in_blocks];
        }

        for i in N..N + shift_in_blocks {
            if i < 2 * N {
                overflow.data[i - N] = self.data[i - shift_in_blocks];
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

        for i in 0..N {
            temp = carry;
            carry = result.data[i] >> (64 - shift_in_bits);
            result.data[i] <<= shift_in_bits;
            result.data[i] |= temp;
        }
        carry = 0;
        for i in 0..N {
            temp = carry;
            carry = overflow.data[i] >> (64 - shift_in_bits);
            overflow.data[i] <<= shift_in_bits;
            overflow.data[i] |= temp;
        }

        (result, overflow)
    }

    fn from_u64_shifted(block: u64, shift_in_blocks: usize) -> Self {
        let mut result = BigUInt::<N>::new();
        result.data[shift_in_blocks] = block;
        result
    }

    pub fn carrying_mul(&self, other: &Self, carry: &mut Self) -> (Self, Self) {
        let mut result = BigUInt::<N>::new();
        let mut overflow;
        let mut temp_r: BigUInt<N>;
        let mut temp_o: BigUInt<N>;
        let mut temp_carry = BigUInt::<N>::new();
        let mut temp_bool: bool;

        for i in 0..N {
            (temp_r, overflow) = self.overflowing_mul_by_u64(other.data[i]);
            (temp_r, temp_o) = temp_r.overflowing_shift_left(i);
            temp_o = temp_o.strict_add(&BigUInt::<N>::from_u64_shifted(overflow, i));
            (result, temp_bool) = result.overflowing_add(&temp_r);
            temp_o = temp_o.strict_add(&BigUInt::<N>::from_u64_shifted(temp_bool as u64, 0));
            temp_carry = temp_carry.strict_add(&temp_o);
        }

        (result, temp_bool) = result.overflowing_add(carry);
        temp_carry = temp_carry.strict_add(&BigUInt::<N>::from_u64_shifted(temp_bool as u64, 0));
        (result, temp_carry)
    }

    pub fn overflowing_mul(&self, other: &Self) -> (Self, Self) {
        let mut carry = BigUInt::<N>::new();
        self.carrying_mul(other, &mut carry)
    }

    pub fn wrapping_mul(&self, other: &Self) -> Self {
        let (result, _) = self.overflowing_mul(other);
        result
    }

    pub fn checked_mul(&self, other: &Self) -> Option<Self> {
        let (result, overflow) = self.overflowing_mul(other);
        if overflow != BigUInt::<N>::default() {
            None
        } else {
            Some(result)
        }
    }

    pub fn strict_mul(&self, other: &Self) -> Self {
        let (result, overflow) = self.overflowing_mul(other);
        if overflow != BigUInt::<N>::default() {
            panic!("Overflow in multiplication");
        }
        result
    }
}

impl<const N: usize> FromStr for BigUInt<N> {
    type Err = std::num::ParseIntError;
    fn from_str(hex_str: &str) -> Result<Self, Self::Err> {
        if hex_str.is_empty() {
            return Ok(BigUInt::<N>::default());
        }
        let mut result = BigUInt::<N>::new();
        let hex_str = hex_str.trim_start_matches("0x");
        let hex_str = format!("{:0>64}", hex_str);
        for i in 0..N {
            let start = hex_str.len() - (i + 1) * 16;
            let end = hex_str.len() - i * 16;
            let block_str = &hex_str[start..end];
            result.data[i] = u64::from_str_radix(block_str, 16)?;
        }
        Ok(result)
    }
}

impl<const N: usize> Add for BigUInt<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.wrapping_add(&other)
    }
}

impl<const N: usize> Add<&Self> for BigUInt<N> {
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        self.wrapping_add(other)
    }
}

impl<const N: usize> Add for &BigUInt<N> {
    type Output = BigUInt<N>;

    fn add(self, other: Self) -> Self::Output {
        self.wrapping_add(other)
    }
}

impl<const N: usize> Add<BigUInt<N>> for &BigUInt<N> {
    type Output = BigUInt<N>;

    fn add(self, other: BigUInt<N>) -> Self::Output {
        self.wrapping_add(&other)
    }
}

impl<const N: usize> AddAssign for BigUInt<N> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<const N: usize> AddAssign<&Self> for BigUInt<N> {
    fn add_assign(&mut self, other: &Self) {
        *self = *self + *other;
    }
}

impl<const N: usize> Sub for BigUInt<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.wrapping_sub(&other)
    }
}

impl<const N: usize> Sub<&Self> for BigUInt<N> {
    type Output = Self;

    fn sub(self, other: &Self) -> Self::Output {
        self.wrapping_sub(other)
    }
}

impl<const N: usize> Sub for &BigUInt<N> {
    type Output = BigUInt<N>;

    fn sub(self, other: Self) -> Self::Output {
        self.wrapping_sub(other)
    }
}

impl<const N: usize> Sub<BigUInt<N>> for &BigUInt<N> {
    type Output = BigUInt<N>;

    fn sub(self, other: BigUInt<N>) -> Self::Output {
        self.wrapping_sub(&other)
    }
}

impl<const N: usize> SubAssign for BigUInt<N> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<const N: usize> SubAssign<&Self> for BigUInt<N> {
    fn sub_assign(&mut self, other: &Self) {
        *self = *self - *other;
    }
}

impl<const N: usize> Mul for BigUInt<N> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.wrapping_mul(&other)
    }
}

impl<const N: usize> Mul<&Self> for BigUInt<N> {
    type Output = Self;

    fn mul(self, other: &Self) -> Self::Output {
        self.wrapping_mul(other)
    }
}

impl<const N: usize> Mul for &BigUInt<N> {
    type Output = BigUInt<N>;

    fn mul(self, other: Self) -> Self::Output {
        self.wrapping_mul(other)
    }
}

impl<const N: usize> Mul<BigUInt<N>> for &BigUInt<N> {
    type Output = BigUInt<N>;

    fn mul(self, other: BigUInt<N>) -> Self::Output {
        self.wrapping_mul(&other)
    }
}

impl<const N: usize> MulAssign for BigUInt<N> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<const N: usize> MulAssign<&Self> for BigUInt<N> {
    fn mul_assign(&mut self, other: &Self) {
        *self = *self * *other;
    }
}

impl<const N: usize> std::fmt::Display for BigUInt<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hex_str = String::new();
        for i in (0..N).rev() {
            hex_str.push_str(&format!("{:016x}", self.data[i]));
        }
        write!(f, "0x{}", hex_str)
    }
}

/// Macro to create BigUInt<N> types from list of (type_name, N) pairs.
macro_rules! new_biguints {
    ($($type_name:ident, $N:expr);*) => {
        $(
            pub type $type_name = BigUInt<$N>;
        )*
    };
    () => {

    };
}

new_biguints! (
    BigUInt1024, 16;
    BigUInt2048, 32;
    BigUInt4096, 64;
    BigUInt8192, 128
);
