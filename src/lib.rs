

pub trait Capacity {

  fn bytes(&self) -> i64 where Self: Sized;

  fn kilobytes(&self) -> i64 where Self:Sized {
    self.bytes().rotate_left(10)
  }

  fn megabytes(&self) -> i64 where Self:Sized {
    self.kilobytes().rotate_left(10)
  }

  fn gigabytes(&self) -> i64 where Self:Sized{
    self.megabytes().rotate_left(10)
  }

  fn terabytes(&self) -> i64 where Self:Sized {
    self.gigabytes().rotate_left(10)
  }

  fn petabytes(&self) -> i64 where Self:Sized{
    self.terabytes().rotate_left(10)
  }

  fn exabytes(&self) -> i64 where Self:Sized {
    self.petabytes().rotate_left(10)
  }
}

impl Capacity for i64 {
  fn bytes(&self) -> i64 {
    *self
  }
}

//pub struct Unparseable;

//impl FromStr for Capacity {
//  type Err = Unparseable;
//  #[inline]
//  fn from_str(s: &str) -> Result<Capacity, Unparseable> {
//    Err(Unparseable)
//  }
//}

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
