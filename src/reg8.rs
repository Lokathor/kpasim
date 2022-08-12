use core::fmt::{
  Binary, Debug, Display, LowerHex, Octal, Pointer, UpperHex, Write,
};

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Reg8(u8);
impl Reg8 {
  pub const fn new(u: u8) -> Self {
    Self(u)
  }
  pub const fn get(self) -> u8 {
    self.0
  }
  pub fn set(&mut self, u: u8) {
    self.0 = u;
  }
}

impl Debug for Reg8 {
  /// Prints the value as an unsigned decimal, or signed if you use alternate
  /// formatting.
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if f.alternate() {
      <i8 as Debug>::fmt(&(self.0 as i8), f)
    } else {
      <u8 as Debug>::fmt(&self.0, f)
    }
  }
}

impl Display for Reg8 {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    <Self as Debug>::fmt(self, f)
  }
}

impl Binary for Reg8 {
  /// Prints in binary with a `%` prefix, or `0b` if you use the alternate
  /// formatting.
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if f.alternate() {
      // this will have a 0b prefix
      <u8 as Binary>::fmt(&self.0, f)
    } else {
      f.write_char('%')?;
      <u8 as Binary>::fmt(&self.0, f)
    }
  }
}

impl Octal for Reg8 {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    <u8 as Octal>::fmt(&self.0, f)
  }
}

impl LowerHex for Reg8 {
  /// Prefix `$`, or `0x` with alternate
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if f.alternate() {
      // this will have a 0x prefix
      <u8 as LowerHex>::fmt(&self.0, f)
    } else {
      f.write_char('$')?;
      <u8 as LowerHex>::fmt(&self.0, f)
    }
  }
}

impl UpperHex for Reg8 {
  /// Prefix `$`, or `0x` with alternate
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if f.alternate() {
      // this will have a 0x prefix
      <u8 as UpperHex>::fmt(&self.0, f)
    } else {
      f.write_char('$')?;
      <u8 as UpperHex>::fmt(&self.0, f)
    }
  }
}

impl Pointer for Reg8 {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    <Self as UpperHex>::fmt(self, f)
  }
}
