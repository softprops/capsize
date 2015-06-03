use std::convert::Into;
use std::str::FromStr;


const BYTE: i64 = 1;
const KILOBYTE: i64 = BYTE << 10;
const MEGABYTE: i64 = KILOBYTE << 10;
const GIGABYTE: i64 = MEGABYTE << 10;
const TERABYTE: i64 = GIGABYTE << 10;
const PETABYTE: i64 = TERABYTE << 10;
const EXABYTE: i64 = PETABYTE << 10;

pub trait Capacity {

  fn bytes(&self) -> i64;

  fn kilobytes(&self) -> i64 {
    self.bytes().rotate_left(10)
  }

  fn megabytes(&self) -> i64 {
    self.kilobytes().rotate_left(10)
  }

  fn gigabytes(&self) -> i64 {
    self.megabytes().rotate_left(10)
  }

  fn terabytes(&self) -> i64 {
    self.gigabytes().rotate_left(10)
  }

  fn petabytes(&self) -> i64 {
    self.terabytes().rotate_left(10)
  }

  fn exabytes(&self) -> i64 {
    self.petabytes().rotate_left(10)
  }

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
    Err(s.to_owned())
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
