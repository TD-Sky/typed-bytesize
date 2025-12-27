use core::str::FromStr;

use crate::{ByteSizeIec, ByteSizeSi, Error, GB, KB, KIB, MIB};

macro_rules! assert_si_eq {
    ($s:literal, $bs:expr) => {
        assert_eq!(ByteSizeSi::from_str($s), Ok($bs));
    };
}

macro_rules! assert_iec_eq {
    ($s:literal, $bs:expr) => {
        assert_eq!(ByteSizeIec::from_str($s), Ok($bs));
    };
}

macro_rules! assert_si_error {
    ($s:literal, $e:expr) => {
        assert_eq!(ByteSizeSi::from_str($s), Err($e));
    };
}

macro_rules! assert_iec_error {
    ($s:literal, $e:expr) => {
        assert_eq!(ByteSizeIec::from_str($s), Err($e));
    };
}

macro_rules! assert_display {
    ($s:literal, $v:expr) => {
        assert_eq!($s, &$v.to_string());
    };
}

#[test]
fn test_parse_int_si() {
    assert_si_eq!("999", ByteSizeSi(999));
    assert_si_eq!("999B", ByteSizeSi::b(999u64));
    assert_si_eq!("999kB", ByteSizeSi::kb(999));
    assert_si_eq!("999MB", ByteSizeSi::mb(999));
    assert_si_eq!("999GB", ByteSizeSi::gb(999));
    assert_si_eq!("999TB", ByteSizeSi::tb(999));
    assert_si_eq!("999PB", ByteSizeSi::pb(999));
    assert_si_eq!("18EB", ByteSizeSi::eb(18));
}

#[test]
fn test_parse_float_si() {
    let expected = 114.514 * KB as f64;
    assert_si_eq!("114.514KB", ByteSizeSi::b(expected as u64));

    let expected = 0.1919810 * GB as f64;
    assert_si_eq!("0.1919810GB", ByteSizeSi::b(expected as u64));
}

#[test]
fn test_parse_max_si() {
    let size = ByteSizeSi::from_str("18.4EB").unwrap();
    assert!(size < ByteSizeSi::MAX);

    // NOTE: Oversized float will round to MAX
    let size = ByteSizeSi::from_str("18.5EB").unwrap();
    assert_eq!(size, ByteSizeSi::MAX);

    let size = ByteSizeSi::from_str("114514.0EB").unwrap();
    assert_eq!(size, ByteSizeSi::MAX);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn test_parse_oversize_si() {
    let _ = ByteSizeSi::from_str("19EB");
}

#[test]
fn test_display_si() {
    assert_display!("725B", ByteSizeSi::b(725u64));
    assert_display!("310.0kB", ByteSizeSi::kb(310));
    assert_display!("18.7MB", ByteSizeSi::kb(18666));
    assert_display!("806.2GB", ByteSizeSi::mb(806233));
    assert_display!("25.3TB", ByteSizeSi::gb(25270));
    assert_display!("12.7PB", ByteSizeSi::tb(12722));
    assert_display!("18.0EB", ByteSizeSi::eb(18));
}

#[test]
fn test_parse_int_iec() {
    assert_iec_eq!("1023", ByteSizeIec(1023));
    assert_iec_eq!("1023B", ByteSizeIec::b(1023u64));
    assert_iec_eq!("1023KiB", ByteSizeIec::kib(1023));
    assert_iec_eq!("1023MiB", ByteSizeIec::mib(1023));
    assert_iec_eq!("1023GiB", ByteSizeIec::gib(1023));
    assert_iec_eq!("1023TiB", ByteSizeIec::tib(1023));
    assert_iec_eq!("1023PiB", ByteSizeIec::pib(1023));
    assert_iec_eq!("15EiB", ByteSizeIec::eib(15));
}

#[test]
fn test_parse_float_iec() {
    let expected = 114.514 * KIB as f64;
    assert_iec_eq!("114.514KiB", ByteSizeIec::b(expected as u64));

    let expected = 0.1919810 * MIB as f64;
    assert_iec_eq!("0.1919810MiB", ByteSizeIec::b(expected as u64));
}

#[test]
fn test_parse_max_iec() {
    let size = ByteSizeIec::from_str("15.9EiB").unwrap();
    assert!(size < ByteSizeIec::MAX);

    // NOTE: Oversized float will round to MAX
    let size = ByteSizeIec::from_str("16.0EiB").unwrap();
    assert_eq!(size, ByteSizeIec::MAX);

    let size = ByteSizeIec::from_str("114514.0EiB").unwrap();
    assert_eq!(size, ByteSizeIec::MAX);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn test_parse_oversize_iec() {
    let _ = ByteSizeIec::from_str("17EiB");
}

#[test]
fn test_display_iec() {
    assert_display!("523B", ByteSizeIec(523));
    assert_display!("1.2KiB", ByteSizeIec(1228));
    assert_display!("7.8MiB", ByteSizeIec::kib(7987));
    assert_display!("91.5GiB", ByteSizeIec::mib(93696));
    assert_display!("290.8TiB", ByteSizeIec::gib(297779));
    assert_display!("477.9PiB", ByteSizeIec::tib(489369));
    assert_display!("15.0EiB", ByteSizeIec::eib(15));
}

#[test]
fn test_parse_min() {
    assert_si_eq!("0B", ByteSizeSi::MIN);
    assert_si_eq!("0.9B", ByteSizeSi::MIN);

    assert_iec_eq!("0B", ByteSizeIec::MIN);
    assert_iec_eq!("0.9B", ByteSizeIec::MIN);
}

#[test]
fn test_parse_with_mid_spaces() {
    assert_si_eq!("114.514 kB", ByteSizeSi(114514));
    assert_iec_eq!("114.514    KiB", ByteSizeIec(117262));
    assert_si_error!("114.514\tKB", Error::Unit);
}

#[test]
fn test_cross_parse() {
    assert_iec_eq!("114.514 KB", ByteSizeIec(114514));
    assert_si_eq!("5.5 GiB", ByteSizeSi::b(5632 * MIB));
}

#[test]
fn test_parse_error() {
    assert_iec_error!("114.514", Error::Invalid);
    assert_si_error!("114.514 ", Error::Unit);
    assert_si_error!("", Error::Empty);
    assert_si_error!(".123GB", Error::Invalid);
    assert_si_error!("2.5E10 B", Error::Unit);
    assert_iec_error!("11. TiB", Error::Invalid);
    assert_iec_error!("-9MiB", Error::Invalid);
    assert_si_error!("inf B", Error::Invalid);
}
