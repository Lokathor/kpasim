use core::fmt::{Debug, Display, Write};

use bitfrob::{u8_get_bit, u8_with_bit};

/// The flags register.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct RegFlags(u8);
impl RegFlags {
  #[inline]
  #[must_use]
  pub const fn z(self) -> bool {
    u8_get_bit(7, self.0)
  }
  #[inline]
  pub fn set_z(&mut self, val: bool) {
    self.0 = u8_with_bit(7, self.0, val);
  }
  #[inline]
  #[must_use]
  pub const fn n(self) -> bool {
    u8_get_bit(6, self.0)
  }
  #[inline]
  pub fn set_n(&mut self, val: bool) {
    self.0 = u8_with_bit(6, self.0, val);
  }
  #[inline]
  #[must_use]
  pub const fn h(self) -> bool {
    u8_get_bit(5, self.0)
  }
  #[inline]
  pub fn set_h(&mut self, val: bool) {
    self.0 = u8_with_bit(5, self.0, val);
  }
  #[inline]
  #[must_use]
  pub const fn c(self) -> bool {
    u8_get_bit(4, self.0)
  }
  #[inline]
  pub fn set_c(&mut self, val: bool) {
    self.0 = u8_with_bit(4, self.0, val);
  }
}

impl Debug for RegFlags {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if self.z() {
      f.write_char('Z')?;
    } else {
      f.write_char('_')?;
    }
    if self.n() {
      f.write_char('N')?;
    } else {
      f.write_char('_')?;
    }
    if self.h() {
      f.write_char('H')?;
    } else {
      f.write_char('_')?;
    }
    if self.c() {
      f.write_char('C')?;
    } else {
      f.write_char('_')?;
    }
    Ok(())
  }
}

impl Display for RegFlags {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    <Self as Debug>::fmt(self, f)
  }
}
