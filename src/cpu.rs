use core::{
  fmt::Debug,
  ops::{Deref, DerefMut},
};
use std::collections::VecDeque;

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
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Cpu {
  pub af: Reg16,
  pub bc: Reg16,
  pub de: Reg16,
  pub hl: Reg16,
  pub sp: Reg16,
  pub pc: Reg16,
  pub t_cycles: u32,
  pub action_queue: VecDeque<Action>,
  pub imm: u16,
}
impl Debug for Cpu {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let q = self.deref();
    write!(f, "CPU {{ f:{f:?}, a:{a:02X}, c:{c:02X}, b:{b:02X}, e:{e:02X}, d:{d:02X}, l:{l:02X}, h:{h:02X}, sp:{sp:04X}, pc:{pc:04X}, imm:${imm:04X}, t:{t}, action_queue:{action_queue:?} }}", f = q.flags, a = q.a, c = q.c, b = q.b, e = q.e, d=q.e, l=q.l,h=q.h,sp=q.sp, pc=q.pc, t=q.t_cycles, action_queue=q.action_queue, imm = q.imm)
  }
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
      action_queue: VecDeque::default(),
      imm: 0,
    }
  }

  pub fn fetch_pc(&mut self, bus: &mut dyn DataBus) -> u8 {
    let b = bus.read(self.pc.get());
    self.pc.inc();
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
    if let Some(action) = self.action_queue.pop_front() {
      match action {
        Action::Internal => (),
        Action::FetchImmLow => {
          //println!("fetching immediate low byte");
          let b = self.fetch_pc(bus);
          let imm_mut: &mut [u8; 2] =
            bytemuck::cast_mut::<u16, [u8; 2]>(&mut self.imm);
          if cfg!(target_endian = "little") {
            imm_mut[0] = b;
          } else {
            imm_mut[1] = b;
          }
        }
        Action::FetchImmHigh(target) => {
          //println!("fetching immediate high byte");
          let b = self.fetch_pc(bus);
          let imm_mut: &mut [u8; 2] =
            bytemuck::cast_mut::<u16, [u8; 2]>(&mut self.imm);
          if cfg!(target_endian = "little") {
            imm_mut[1] = b;
          } else {
            imm_mut[0] = b;
          }
          match target {
            R16_::SP => self.sp.set(self.imm),
            R16_::PC => self.pc.set(self.imm),
          }
          self.imm = 0;
        }
      }
    } else {
      // When the queue is empty, we default to fetching the next op code.
      let op_code = self.fetch_pc(bus);
      print!("==== New Instruction({op_code:02X}): ");
      match op_code {
        0x00 => {
          println!("NOP");
        }
        0x31 => {
          println!("LD SP, u16");
          self.action_queue.push_back(Action::FetchImmLow);
          self.action_queue.push_back(Action::FetchImmHigh(R16_::SP));
        }
        0xC3 => {
          println!("JP u16");
          self.action_queue.push_back(Action::FetchImmLow);
          self.action_queue.push_back(Action::FetchImmHigh(R16_::PC));
          self.action_queue.push_back(Action::Internal);
        }
        0xF3 => {
          println!("DI (todo)");
        }
        other => {
          println!("???");
          todo!("Unknown Op Code {other:02X}")
        }
      }
    }
    true
  }
}

/// A view of the CPU with the data registers broken into individual bytes.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
  pub t_cycles: u32,
  pub action_queue: VecDeque<Action>,
  pub imm: u16,
}

/// A pending action for the CPU to perform.
///
/// Each action takes 4 T-cycles to complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Action {
  /// Fetch into the low byte of the imm buffer.
  FetchImmLow,
  /// Fetch into the high byte of the imm buffer and move the buffer to the
  /// 16-bit register specified. For simplicity, this clears the immediate
  /// buffer, which is a fake concept that's not really part of the GB anyway.
  FetchImmHigh(R16_),
  /// Burn some time.
  Internal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum R16_ {
  SP,
  PC,
}
