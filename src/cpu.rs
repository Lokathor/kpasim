use alloc::collections::VecDeque;
use core::{
  fmt::Debug,
  ops::{Deref, DerefMut, Not},
  slice,
};

use bytemuck::{cast_mut, cast_slice, cast_slice_mut};

use crate::{
  data_bus::DataBus,
  op_actions::{ActionRegister, CpuAction, ACTION_TABLE},
  op_disassembly::DISASSEMBLY_TABLE,
  reg16::Reg16,
  reg8::Reg8,
  reg_flags::RegFlags,
};

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
  pub action_queue: VecDeque<CpuAction>,
  pub imm: u16,
}
impl Debug for Cpu {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let q = self.deref();
    write!(f, "CPU {{ f:{f:?}, a:{a:02X}, b:{b:02X}, c:{c:02X}, d:{d:02X}, e:{e:02X}, h:{h:02X}, l:{l:02X}, sp:{sp:04X}, pc:{pc:04X}, imm:${imm:04X}, t:{t}, action_queue:{action_queue:?} }}", f = q.flags, a = q.a, c = q.c, b = q.b, e = q.e, d=q.e, l=q.l,h=q.h,sp=q.sp, pc=q.pc, t=q.t_cycles, action_queue=q.action_queue, imm = q.imm)
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
      // Note(Lokathor): The CPU registers after the boot vary depend on which
      // Boot ROM was used to do the start up sequence. Each major model of
      // GB-playing-device has its own Boot ROM with small variations. The only
      // registers that have a consistent value after boot are PC and SP. For
      // simplicity we just zero all the general registers.
      //
      // See: https://gbdev.io/pandocs/Power_Up_Sequence.html#cpu-registers
      af: Reg16::new(0),
      bc: Reg16::new(0),
      de: Reg16::new(0),
      hl: Reg16::new(0),
      sp: Reg16::new(0xFFFE),
      pc: Reg16::new(0x0100),
      t_cycles: 0,
      action_queue: VecDeque::default(),
      imm: 0,
    }
  }

  pub fn fetch_pc(&mut self, bus: &mut dyn DataBus) -> u8 {
    let b = bus.read(self.pc.get());
    //println!("FETCH: ${b:02X}");
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
    // When there's no pending actions we have to get a new op code to queue up
    // some actions. After we do this we *also* perform one action, so the
    // actions table must be arranged appropriately. Anything that happens as
    // soon as the op-code comes in (eg: `ld a, b`) will be just 1 action.
    if self.action_queue.is_empty() {
      let op_code = self.fetch_pc(bus);
      let disassembly = DISASSEMBLY_TABLE[usize::from(op_code)];
      let actions = ACTION_TABLE[usize::from(op_code)];
      println!(
        "Queue Code (${op_code:02X}): {disassembly: <17} // {actions:?}"
      );
      self.action_queue.extend(actions.iter().copied());
    }
    let action = self.action_queue.pop_front().unwrap();
    self.process_action(bus, action);
    true
  }

  fn process_action(&mut self, bus: &mut dyn DataBus, action: CpuAction) {
    use CpuAction::*;
    match action {
      Internal => (),
      DisableInterrupts => (/* TODO*/),
      ImmLow => {
        let imm8 = self.fetch_pc(bus);
        let imm_bytes: &mut [u8] =
          cast_slice_mut(slice::from_mut(&mut self.imm));
        let index = usize::from(cfg!(target_endian = "little").not());
        imm_bytes[index] = imm8;
      }
      ImmLowTo(reg) => match reg {
        ActionRegister::A => {
          let imm8 = self.fetch_pc(bus);
          //println!("ImmLowTo({reg:?}): ${imm8:02X}");
          self.a.set(imm8);
          self.imm = 0;
        }
        ActionRegister::PC => todo!(),
        ActionRegister::SP => todo!(),
      },
      ImmHigh => {
        let imm8 = self.fetch_pc(bus);
        let imm_bytes: &mut [u8] =
          cast_slice_mut(slice::from_mut(&mut self.imm));
        let index = usize::from(cfg!(target_endian = "little"));
        imm_bytes[index] = imm8;
      }
      ImmHighTo(reg) => {
        let imm8 = self.fetch_pc(bus);
        let imm_bytes: &mut [u8] =
          cast_slice_mut(slice::from_mut(&mut self.imm));
        let index = usize::from(cfg!(target_endian = "little"));
        imm_bytes[index] = imm8;
        match reg {
          ActionRegister::PC => self.pc.set(self.imm),
          ActionRegister::SP => self.sp.set(self.imm),
          ActionRegister::A => todo!(),
        }
        self.imm = 0;
      }
      WriteRegToImm16(reg) => {
        match reg {
          ActionRegister::A => bus.write(self.imm, self.a.get()),
          ActionRegister::PC => todo!(),
          ActionRegister::SP => todo!(),
        };
        self.imm = 0;
      }
      WriteRegToHalfAddr(reg) => {
        debug_assert!(self.imm <= u16::from(u8::MAX));
        let addr = 0xFF00 + self.imm;
        match reg {
          ActionRegister::A => bus.write(addr, self.a.get()),
          ActionRegister::PC => todo!(),
          ActionRegister::SP => todo!(),
        };
        self.imm = 0;
      }
    }
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
  pub action_queue: VecDeque<CpuAction>,
  pub imm: u16,
}
