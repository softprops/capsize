//#![deny(missing_docs)]

//! # Capsize
//!
//! Capsize provides conversions between units of capacity,
//! similar in nature to [Duration](https://doc.rust-lang.org/std/time/duration/), which provides conversions between units of time.
//!
//! All conversions are represented as an `i64` by default.
//!
//! This crate also provides FromStr implementations that parse values "1k" into
//! their corresponding capacity in `i64` format in bytes.
//!
//! # Examples
//!
//! ```rust
//! use capsize::Capacity;
//!
//! // convert to kilobytes to bytes
//! let bytes = 4.kilobytes();
//! assert_eq!(bytes, 4096);
//!
//! // resolve 4096 to the closest human readable form
//! assert_eq!(bytes.capacity(), "4.0K");
//! ```
use std::convert::Into;
use std::str::FromStr;

const BYTE: i64     = 1;
const KILOBYTE: i64 = BYTE     << 10;
const MEGABYTE: i64 = KILOBYTE << 10;
const GIGABYTE: i64 = MEGABYTE << 10;
const TERABYTE: i64 = GIGABYTE << 10;
const PETABYTE: i64 = TERABYTE << 10;
const EXABYTE: i64  = PETABYTE << 10;

macro_rules! map(
  { $($key:expr => $value:expr),+ } => {
    {
      let mut m = ::std::collections::HashMap::new();
      $(
        m.insert($key, $value);
        )+
        m
    }
  };
);


/// Capacity provides a simple way to convert
/// values to corresponding units of capacity
pub trait Capacity {

  /// size in bytes
  fn bytes(&self) -> i64;

  /// size in kilobytes
  fn kilobytes(&self) -> i64 {
    self.bytes().rotate_left(10)
  }

  /// size in megabytes
  fn megabytes(&self) -> i64 {
    self.kilobytes().rotate_left(10)
  }

  /// size in gigabytes
  fn gigabytes(&self) -> i64 {
    self.megabytes().rotate_left(10)
  }

  /// size in terabytes
  fn terabytes(&self) -> i64 {
    self.gigabytes().rotate_left(10)
  }

  /// size in petabytes
  fn petabytes(&self) -> i64 {
    self.terabytes().rotate_left(10)
  }

  /// size in exabytes
  fn exabytes(&self) -> i64 {
    self.petabytes().rotate_left(10)
  }

  /// size as a human readable string
  fn capacity(&self) -> String {
    match self.bytes() {
      small if small < KILOBYTE =>
        stringify!(small).to_owned(),
      large => {
        let units = vec![
          ('E', EXABYTE),
          ('P', PETABYTE),
          ('T', TERABYTE),
          ('G', GIGABYTE),
          ('M', MEGABYTE),
          ('K', KILOBYTE)
        ];
        for (suffix, size) in units {
          if large == size {
            return format!("1{}", suffix)
          } else if large > size {
            let sized = (large as f64) / (size as f64);
            let round = (sized * 100.0).round() / 100.0;
            return format!("{:.1}{}", round, suffix)
          }
        }
        unreachable!()
      }
    }
  }
}

impl Capacity for i64 {
  fn bytes(&self) -> i64 {
    *self
  }
}

impl Capacity for Bytes {
  fn bytes(&self) -> i64 {
    (*self).size
  }
}

/// Bytes is a simple type
/// to extract a capacity size, in bytes,
/// from a string value
#[derive(Debug)]
pub struct Bytes {
  size: i64
}

impl Into<Bytes> for i64 {
  fn into(self) -> Bytes {
    Bytes { size: self }
  }
}

impl FromStr for Bytes {
  type Err = String;
  fn from_str(s: &str) -> Result<Bytes, String> {
    let units = map!(
      'E' => EXABYTE,
      'P' => PETABYTE,
      'T' => TERABYTE,
      'G' => GIGABYTE,
      'M' => MEGABYTE,
      'K' => KILOBYTE
    );
    if s.len() > 1 {
      let last = s.chars().last().unwrap();
      match units.get(&last) {
        Some(unit) => {
          let init: String = s.chars().take(s.chars().count() - 1).collect();
          init.parse::<i64>().map(|lit| {
            Bytes { size: lit * unit }
          }).or_else(|_| Err(s.to_owned()))
        },
        _ => {
          Err(s.to_owned())
        }
      }
    } else {
      s.parse::<i64>().map(|lit| {
        Bytes { size: lit }
      }).or_else(|_| Err(s.to_owned()))
    }
  }
}

#[test]
fn test_bytes() {
  assert_eq!(1.bytes(), 1);
}

#[test]
fn test_kilobytes() {
  assert_eq!(1.kilobytes(), 1024)
}

#[test]
fn test_megabytes() {
  assert_eq!(1.megabytes(), 1048576)
}

#[test]
fn test_gigabytes() {
  assert_eq!(1.gigabytes(), 1073741824)
}

#[test]
fn test_terabytes() {
  assert_eq!(1.terabytes(), 1099511627776)
}

#[test]
fn test_petabytes() {
  assert_eq!(1.petabytes(), 1125899906842624)
}

#[test]
fn test_exabytes() {
  assert_eq!(1.exabytes(), 1152921504606846976)
}

#[test]
fn test_kilobytes_capactity() {
  let half = 1.kilobytes() / 2;
  assert_eq!((1.kilobytes() + half).capacity(), "1.5K".to_owned())
}

#[test]
fn test_megabytes_capactity() {
  let half = 1.megabytes() / 2;
  assert_eq!((1.megabytes() + half).capacity(), "1.5M".to_owned())
}

#[test]
fn test_gigabytes_capactity() {
  let half = 1.gigabytes() / 2;
  assert_eq!((1.gigabytes() + half).capacity(), "1.5G".to_owned())
}

#[test]
fn test_terabytes_capactity() {
  let half = 1.terabytes() / 2;
  assert_eq!((1.terabytes() + half).capacity(), "1.5T".to_owned())
}

#[test]
fn test_petabytes_capactity() {
  let half = 1.petabytes() / 2;
  assert_eq!((1.petabytes() + half).capacity(), "1.5P".to_owned())
}

#[test]
fn test_exabytes_capactity() {
  let half = 1.exabytes() / 2;
  assert_eq!((1.exabytes() + half).capacity(), "1.5E".to_owned())
}

#[test]
fn test_kilobytes_parse() {
  let cap: String = 1.kilobytes().capacity();
  let bytes = cap.parse::<Bytes>().ok().unwrap();
  assert_eq!(bytes.capacity(), cap)
}
