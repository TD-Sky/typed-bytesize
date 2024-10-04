#![doc = include_str!("../README.md")]

#[cfg(test)]
mod tests;

#[cfg(feature = "serde")]
mod serde;

use core::num::{IntErrorKind, ParseFloatError, ParseIntError};
use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use core::str::FromStr;

/// 1 byte
pub const B: u64 = 1;

/// 1 kilobyte
pub const KB: u64 = 10u64.pow(3);
/// 1 megabyte
pub const MB: u64 = 10u64.pow(6);
/// 1 gigabyte
pub const GB: u64 = 10u64.pow(9);
/// 1 terabyte
pub const TB: u64 = 10u64.pow(12);
/// 1 petabyte
pub const PB: u64 = 10u64.pow(15);
/// 1 exabyte
pub const EB: u64 = 10u64.pow(18);

/// Decimal prefix bytesize
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ByteSizeSi(pub u64);

impl ByteSizeSi {
    const PREFIXES: &'static [char] = &['k', 'M', 'G', 'T', 'P', 'E'];
}

impl core::fmt::Display for ByteSizeSi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.0 < KB {
            write!(f, "{}B", self.0)
        } else {
            let size = self.0 as f64;
            let exp = (size.log(KB as f64) as usize).min(Self::PREFIXES.len());
            let number = size / (KB.pow(exp as u32) as f64);
            let prefix = Self::PREFIXES[exp - 1];

            write!(f, "{number:.1}{prefix}B")
        }
    }
}

impl FromStr for ByteSizeSi {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_byte_size(s).map(Self)
    }
}

impl ByteSizeSi {
    /// The smallest value that can be represented.
    pub const MIN: Self = Self(u64::MIN);

    /// The largest value that can be represented.
    pub const MAX: Self = Self(u64::MAX);

    #[inline]
    pub fn b(n: impl Into<u64>) -> Self {
        Self(n.into())
    }

    #[inline(always)]
    pub const fn kb(n: u64) -> Self {
        Self(n * KB)
    }

    #[inline(always)]
    pub const fn mb(n: u64) -> Self {
        Self(n * MB)
    }

    #[inline(always)]
    pub const fn gb(n: u64) -> Self {
        Self(n * GB)
    }

    #[inline(always)]
    pub const fn tb(n: u64) -> Self {
        Self(n * TB)
    }

    #[inline(always)]
    pub const fn pb(n: u64) -> Self {
        Self(n * PB)
    }

    #[inline(always)]
    pub const fn eb(n: u64) -> Self {
        Self(n * EB)
    }

    /// Convert into binary prefix unit
    #[inline(always)]
    pub const fn iec(self) -> ByteSizeIec {
        ByteSizeIec(self.0)
    }
}

impl From<u64> for ByteSizeSi {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl From<ByteSizeSi> for u64 {
    fn from(bs: ByteSizeSi) -> Self {
        bs.0
    }
}

impl Add for ByteSizeSi {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for ByteSizeSi {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for ByteSizeSi {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for ByteSizeSi {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<T> Mul<T> for ByteSizeSi
where
    T: Into<u64>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs.into())
    }
}

impl<T> MulAssign<T> for ByteSizeSi
where
    T: Into<u64>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs.into();
    }
}

/// 1 kibibyte
pub const KIB: u64 = 2u64.pow(10);
/// 1 mebibyte
pub const MIB: u64 = 2u64.pow(20);
/// 1 gibibyte
pub const GIB: u64 = 2u64.pow(30);
/// 1 tebibyte
pub const TIB: u64 = 2u64.pow(40);
/// 1 pebibyte
pub const PIB: u64 = 2u64.pow(50);
/// 1 exbibyte
pub const EIB: u64 = 2u64.pow(60);

/// Binary prefix bytesize
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ByteSizeIec(pub u64);

impl ByteSizeIec {
    const PREFIXES: &'static [char] = &['K', 'M', 'G', 'T', 'P', 'E'];
}

impl core::fmt::Display for ByteSizeIec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.0 < KIB {
            write!(f, "{}B", self.0)
        } else {
            let size = self.0 as f64;
            let exp = (size.log(KIB as f64) as usize).min(Self::PREFIXES.len());
            let number = size / (KIB.pow(exp as u32) as f64);
            let prefix = Self::PREFIXES[exp - 1];

            write!(f, "{number:.1}{prefix}iB")
        }
    }
}

impl FromStr for ByteSizeIec {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_byte_size(s).map(Self)
    }
}

impl ByteSizeIec {
    /// The smallest value that can be represented.
    pub const MIN: Self = Self(u64::MIN);

    /// The largest value that can be represented.
    pub const MAX: Self = Self(u64::MAX);

    #[inline]
    pub fn b(n: impl Into<u64>) -> Self {
        Self(n.into())
    }

    #[inline(always)]
    pub const fn kib(n: u64) -> Self {
        Self(n * KIB)
    }

    #[inline(always)]
    pub const fn mib(n: u64) -> Self {
        Self(n * MIB)
    }

    #[inline(always)]
    pub const fn gib(n: u64) -> Self {
        Self(n * GIB)
    }

    #[inline(always)]
    pub const fn tib(n: u64) -> Self {
        Self(n * TIB)
    }

    #[inline(always)]
    pub const fn pib(n: u64) -> Self {
        Self(n * PIB)
    }

    #[inline(always)]
    pub const fn eib(n: u64) -> Self {
        Self(n * EIB)
    }

    /// Convert into decimal prefix unit
    #[inline(always)]
    pub const fn si(self) -> ByteSizeSi {
        ByteSizeSi(self.0)
    }
}

impl From<u64> for ByteSizeIec {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl From<ByteSizeIec> for u64 {
    fn from(bs: ByteSizeIec) -> Self {
        bs.0
    }
}

impl Add for ByteSizeIec {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for ByteSizeIec {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for ByteSizeIec {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for ByteSizeIec {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<T> Mul<T> for ByteSizeIec
where
    T: Into<u64>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs.into())
    }
}

impl<T> MulAssign<T> for ByteSizeIec
where
    T: Into<u64>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs.into();
    }
}

fn parse_byte_size(input: &str) -> Result<u64, Error> {
    let Some(i) = input.find(|c: char| !c.is_ascii_digit()) else {
        return input.parse::<u64>().map_err(Into::into);
    };

    if i == 0 {
        return Err(Error::Invalid);
    }

    let (integer, rest) = input.split_at(i);

    if let Some(rest) = rest.strip_prefix('.') {
        let i = i
            + 1
            + rest
                .find(|c: char| !c.is_ascii_digit())
                .filter(|n| *n > 0)
                .ok_or(Error::Invalid)?;

        let (float, rest) = input.split_at(i);
        let float = float.parse::<f64>()?;
        let unit = parse_unit(rest.trim_start_matches(' '))?;
        Ok((float * unit as f64) as u64)
    } else {
        let integer = integer.parse::<u64>()?;
        let unit = parse_unit(rest.trim_start_matches(' '))?;
        Ok(integer * unit)
    }
}

fn parse_unit(input: &str) -> Result<u64, Error> {
    match input.to_lowercase().as_str() {
        "b" => Ok(B),
        // SI
        "k" | "kb" => Ok(KB),
        "m" | "mb" => Ok(MB),
        "g" | "gb" => Ok(GB),
        "t" | "tb" => Ok(TB),
        "p" | "pb" => Ok(PB),
        "e" | "eb" => Ok(EB),
        // IEC 60027-2
        "ki" | "kib" => Ok(KIB),
        "mi" | "mib" => Ok(MIB),
        "gi" | "gib" => Ok(GIB),
        "ti" | "tib" => Ok(TIB),
        "pi" | "pib" => Ok(PIB),
        "ei" | "eib" => Ok(EIB),
        _ => Err(Error::Unit),
    }
}

impl From<ByteSizeIec> for ByteSizeSi {
    fn from(iec: ByteSizeIec) -> Self {
        iec.si()
    }
}

impl From<ByteSizeSi> for ByteSizeIec {
    fn from(si: ByteSizeSi) -> Self {
        si.iec()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Empty,
    Invalid,
    Unit,
}

impl core::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let msg = match self {
            Self::Empty => "cannot parse bytesize from empty string",
            Self::Invalid => "invalid number found in string",
            Self::Unit => "cannot recognize byte unit in string",
        };
        f.write_str(msg)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        match e.kind() {
            IntErrorKind::Empty => Self::Empty,
            _ => Self::Invalid,
        }
    }
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Self {
        Self::Invalid
    }
}
