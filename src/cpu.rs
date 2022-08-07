use core::ops::{Deref, DerefMut};

use tinyvec::ArrayVec;

use crate::{data_bus::DataBus, reg16::Reg16, reg8::Reg8, reg_flags::RegFlags};

/// Simulates the Game Boy's LR35902 CPU.
///
/// This is the view of the CPU with 16-bit registers. To access the 8-bit
/// registers, we have a [Deref] impl from this type to the [CpuByteFields]
/// type. This lets us easily access any data register as either the 16-bit or
/// 8-bit form, but brings a drawback: the `Deref` borrows the entire struct, so
/// we can't easily "split borrow" when we're accessing the fields in 8-bit
/// mode. I don't think that will be an issue, because we won't normally have to
/// hold a borrow on the CPU longer than a single statement.
///
/// * See Also: [Pandocs: CPU Registers and flags](https://gbdev.io/pandocs/CPU_Registers_and_Flags.html)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Cpu {
  pub af: Reg16,
  pub bc: Reg16,
  pub de: Reg16,
  pub hl: Reg16,
  pub sp: Reg16,
  pub pc: Reg16,
  pub t_cycles: u32,
  pub action_queue: ArrayVec<[Action; 8]>,
  pub imm: [u8; 2],
}
impl Deref for Cpu {
  type Target = CpuByteFields;
  #[inline]
  #[must_use]
  fn deref(&self) -> &Self::Target {
    unsafe { &*(self as *const Self as *const CpuByteFields) }
  }
}
impl DerefMut for Cpu {
  #[inline]
  #[must_use]
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *(self as *mut Self as *mut CpuByteFields) }
  }
}
impl Cpu {
  pub fn new() -> Self {
    Self {
      af: Reg16::new(0),
      bc: Reg16::new(0),
      de: Reg16::new(0),
      hl: Reg16::new(0),
      sp: Reg16::new(0),
      pc: Reg16::new(0x0100),
      t_cycles: 0,
      action_queue: ArrayVec::default(),
      imm: [0_u8; 2],
    }
  }

  pub fn fetch_pc(&mut self, bus: &mut dyn DataBus) -> u8 {
    let b = bus.read(self.pc.get());
    self.pc.inc();
    println!("fetch_pc: {b:02X}");
    b
  }

  /// Grants a T-cycle worth of time to the CPU.
  ///
  /// The CPU only actually acts once per 4 T-cycles.
  ///
  /// * **Returns:** If the CPU took an action.
  pub fn t_cycle(&mut self, bus: &mut dyn DataBus) -> bool {
    self.t_cycles = self.t_cycles.wrapping_add(1);
    if self.t_cycles % 4 != 0 {
      return false;
    }
    if let Some(action) = self.action_queue.pop() {
      match action {
        Action::Internal => println!("internal cycle"),
        Action::FetchPCImmLow => {
          println!("fetching immediate low byte");
          let b = self.fetch_pc(bus);
          self.imm[0] = b;
        }
        Action::FetchPCImmHigh => {
          println!("fetching immediate low byte");
          let b = self.fetch_pc(bus);
          self.imm[1] = b;
        }
        Action::JumpImm => {
          println!("jumping");
          self.pc.set(u16::from_le_bytes(self.imm));
          self.imm = [0_u8; 2];
        }
      }
    } else {
      // When the queue is empty, we default to fetching the next op code.
      let op_code = self.fetch_pc(bus);
      match op_code {
        0x00 => (),
        0xC3 => {
          self.action_queue.push(Action::JumpImm);
          self.action_queue.push(Action::FetchPCImmHigh);
          self.action_queue.push(Action::FetchPCImmLow);
        }
        other => todo!("Op Code {other:02X}"),
      }
    }
    true
  }
}

/// A view of the CPU with the data registers broken into individual bytes.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
// Note(Lokathor): To support big-endian we'd just have to swap the ordering of
// each pair. However, until big-endian support is really requested, it's better
// to not have two near-identical structs in the codebase.
#[cfg(target_endian = "little")]
pub struct CpuByteFields {
  pub flags: RegFlags,
  pub a: Reg8,
  pub c: Reg8,
  pub b: Reg8,
  pub e: Reg8,
  pub d: Reg8,
  pub l: Reg8,
  pub h: Reg8,
  pub sp: Reg16,
  pub pc: Reg16,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Action {
  #[default]
  Internal,
  FetchPCImmLow,
  FetchPCImmHigh,
  JumpImm,
}
