use core::fmt::{
  Binary, Debug, Display, LowerHex, Octal, Pointer, UpperHex, Write,
};

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Reg16(u16);
impl Reg16 {
  pub const fn new(u: u16) -> Self {
    Self(u)
  }
  pub const fn get(self) -> u16 {
    self.0
  }
  pub fn set(&mut self, u: u16) {
    self.0 = u;
  }
  pub fn inc(&mut self) {
    self.0 = self.0.wrapping_add(1)
  }
}

impl Debug for Reg16 {
  /// Prints the value as an unsigned decimal, or signed if you use alternate
  /// formatting.
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if f.alternate() {
      <i16 as Debug>::fmt(&(self.0 as i16), f)
    } else {
      <u16 as Debug>::fmt(&self.0, f)
    }
  }
}

impl Display for Reg16 {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    <Self as Debug>::fmt(self, f)
  }
}

impl Binary for Reg16 {
  /// Prints in binary with a `%` prefix, or `0b` if you use the alternate
  /// formatting.
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if f.alternate() {
      // this will have a 0b prefix
      <u16 as Binary>::fmt(&self.0, f)
    } else {
      f.write_char('%')?;
      <u16 as Binary>::fmt(&self.0, f)
    }
  }
}

impl Octal for Reg16 {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    <u16 as Octal>::fmt(&self.0, f)
  }
}

impl LowerHex for Reg16 {
  /// Prefix `$`, or `0x` with alternate
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if f.alternate() {
      // this will have a 0x prefix
      <u16 as LowerHex>::fmt(&self.0, f)
    } else {
      f.write_char('$')?;
      <u16 as LowerHex>::fmt(&self.0, f)
    }
  }
}

impl UpperHex for Reg16 {
  /// Prefix `$`, or `0x` with alternate
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if f.alternate() {
      // this will have a 0x prefix
      <u16 as UpperHex>::fmt(&self.0, f)
    } else {
      f.write_char('$')?;
      <u16 as UpperHex>::fmt(&self.0, f)
    }
  }
}

impl Pointer for Reg16 {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    <Self as UpperHex>::fmt(self, f)
  }
}
