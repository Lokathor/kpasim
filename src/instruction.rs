use bitfrob::{u8_get_bit, u8_get_value};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum U2 {
  #[default]
  _0 = 0,
  _1 = 1,
  _2 = 2,
  _3 = 3,
}
impl U2 {
  pub const fn new_from_byte(base: u32, byte: u8) -> Self {
    let value = u8_get_value(base, base + 1, byte);
    match value {
      0 => Self::_0,
      1 => Self::_1,
      2 => Self::_2,
      3 => Self::_3,
      _ => unimplemented!(),
    }
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum U3 {
  #[default]
  _0 = 0,
  _1 = 1,
  _2 = 2,
  _3 = 3,
  _4 = 4,
  _5 = 5,
  _6 = 6,
  _7 = 7,
}
impl U3 {
  pub const fn new_from_byte(base: u32, byte: u8) -> Self {
    let value = u8_get_value(base, base + 2, byte);
    match value {
      0 => Self::_0,
      1 => Self::_1,
      2 => Self::_2,
      3 => Self::_3,
      4 => Self::_4,
      5 => Self::_5,
      6 => Self::_6,
      7 => Self::_7,
      _ => unimplemented!(),
    }
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum R8m {
  #[default]
  B = 0,
  C = 1,
  D = 2,
  E = 3,
  H = 4,
  L = 5,
  HLm = 6,
  A = 7,
}
impl R8m {
  pub const fn new(u: U3) -> Self {
    match u {
      U3::_0 => Self::B,
      U3::_1 => Self::C,
      U3::_2 => Self::D,
      U3::_3 => Self::E,
      U3::_4 => Self::H,
      U3::_5 => Self::L,
      U3::_6 => Self::HLm,
      U3::_7 => Self::A,
    }
  }
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum R16p {
  #[default]
  BC = 0,
  DE = 1,
  HL = 2,
  SP = 3,
}
impl R16p {
  pub const fn new(u: U2) -> Self {
    match u {
      U2::_0 => Self::BC,
      U2::_1 => Self::DE,
      U2::_2 => Self::HL,
      U2::_3 => Self::SP,
    }
  }
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum R16f {
  #[default]
  BC = 0,
  DE = 1,
  HL = 2,
  AF = 3,
}
impl R16f {
  pub const fn new(u: U2) -> Self {
    match u {
      U2::_0 => Self::BC,
      U2::_1 => Self::DE,
      U2::_2 => Self::HL,
      U2::_3 => Self::AF,
    }
  }
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum R16id {
  #[default]
  BC = 0,
  DE = 1,
  HLi = 2,
  HLd = 3,
}
impl R16id {
  pub const fn new(u: U2) -> Self {
    match u {
      U2::_0 => Self::BC,
      U2::_1 => Self::DE,
      U2::_2 => Self::HLi,
      U2::_3 => Self::HLd,
    }
  }
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cond {
  #[default]
  NZ = 0,
  Z = 1,
  NC = 2,
  C = 3,
}
impl Cond {
  pub const fn new(u: u8) -> Self {
    match u {
      0 => Self::NZ,
      1 => Self::Z,
      2 => Self::NC,
      3 => Self::C,
      _ => panic!(),
    }
  }
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Alu {
  #[default]
  Add = 0,
  Adc = 1,
  Sub = 2,
  Sbc = 3,
  And = 4,
  Xor = 5,
  Or = 6,
  Cp = 7,
}
impl Alu {
  pub const fn new(u: U3) -> Self {
    match u {
      U3::_0 => Self::Add,
      U3::_1 => Self::Adc,
      U3::_2 => Self::Sub,
      U3::_3 => Self::Sbc,
      U3::_4 => Self::And,
      U3::_5 => Self::Xor,
      U3::_6 => Self::Or,
      U3::_7 => Self::Cp,
    }
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum IllegalOpByte {
  #[default]
  D3 = 0xD3,
  DB = 0xDB,
  DD = 0xDD,
  E3 = 0xE3,
  E4 = 0xE4,
  EB = 0xEB,
  EC = 0xEC,
  ED = 0xED,
  F4 = 0xF4,
  FC = 0xFC,
  FD = 0xFD,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Instruction {
  /// `nop`
  #[default]
  Nop,
  /// `ld [<u16>], sp`
  LdImm16SP(u16),
  /// `stop`
  Stop,
  /// `jr <i8>`
  JumpRelative(i8),
  /// `jr cond, <i8>`
  JumpRelativeCond(Cond, i8),
  /// `add hl, <r8m>`
  AddHLR16p(R16p),
  /// `ld <r16p>, <u16>`
  LdR16pImm16(R16p, u16),
  /// `ld a, [<r16id>]` (true) OR `ld [<r16id>], a` (false)
  LdR16idToA(R16id, bool),
  /// `dec <r16p>` (true) OR `inc <r16p>` (false)
  DecIncR16p(R16p, bool),
  /// `dec <r8m>` (true) OR `inc <r8m>` (false)
  DecIncR8m(R8m, bool),
  /// `ld <r8m>, <u8>`
  LdR8mImm8(R8m, u8),
  /// `rlca`
  Rlca,
  /// `rrca`
  Rrca,
  /// `rla`
  Rla,
  /// `rra`
  Rra,
  /// `daa`
  Daa,
  /// `cpl`
  Cpl,
  /// `scf`
  Scf,
  /// `ccf`
  Ccf,
  /// `halt`
  Halt,
  /// `ld <r8m>, <r8m>`
  LdR8mR8m(R8m, R8m),
  /// `<op> a, <r8m>`
  AluR8m(Alu, R8m),
  /// `ret <cond>`
  ReturnCond(Cond),
  /// `ldh a, [<u8>]` (true) or `ldh [<u8>], a` (false)
  LdhImm8ToA(u8, bool),
  /// `add sp, <i8>`
  AddSPImm8(i8),
  /// `ld hl, sp+<i8>`
  LdHLSPImm8(i8),
  /// `pop <r16f>`
  Pop(R16f),
  /// `ret`
  Return,
  /// `reti`
  ReturnIrq,
  /// `jp hl`
  JumpHL,
  /// `ld sp, hl`
  LdSPHL,
  /// `jp <cond>, <u16>`
  JumpCond(Cond, u16),
  /// `ldh a, [c]` (true) or `ldh [c], a` (false)
  LdhCToA(bool),
  /// `ld a, [<u16>]` (true) or `ld [<u16>], a` (false)
  LdImm16ToA(bool),
  /// `jp <u16>`
  JumpImm16(u16),
  /// rot, bit, res, set
  Cb(PrefixedOp),
  /// `di`
  DI,
  /// `ei`
  EI,
  /// `call <cond>, <u16>`
  CallCond(Cond, u16),
  Push(R16p),
  Call(u16),
  /// `<op> a, <u8>`
  AluImm8(Alu, u8),
  /// `rst <u8>` (but only multiples of 8 can be restarted to).
  Restart(U3),
  /// A byte that isn't an illegal op code.
  Illegal(IllegalOpByte),
}
impl Instruction {
  pub fn new(op_code: u8) -> Self {
    let x = U2::new_from_byte(6, op_code);
    let y = U3::new_from_byte(3, op_code);
    let z = U3::new_from_byte(0, op_code);
    let p = U2::new_from_byte(4, op_code);
    let q = u8_get_bit(3, op_code);
    //
    match x {
      U2::_0 => match z {
        U3::_0 => match y {
          U3::_0 => Self::Nop,
          U3::_1 => Self::LdImm16SP(0),
          U3::_2 => Self::Stop,
          U3::_3 => Self::JumpRelative(0),
          U3::_4 | U3::_5 | U3::_6 | U3::_7 => {
            Self::JumpRelativeCond(Cond::new((y as u8) - 4), 0)
          }
        },
        U3::_1 => {
          if q {
            Self::AddHLR16p(R16p::new(p))
          } else {
            Self::LdR16pImm16(R16p::new(p), 0)
          }
        }
        U3::_2 => Instruction::LdR16idToA(R16id::new(p), q),
        U3::_3 => Instruction::DecIncR16p(R16p::new(p), q),
        U3::_4 => Instruction::DecIncR8m(R8m::new(y), false),
        U3::_5 => Instruction::DecIncR8m(R8m::new(y), true),
        U3::_6 => Instruction::LdR8mImm8(R8m::new(y), 0),
        U3::_7 => match y {
          U3::_0 => Instruction::Rlca,
          U3::_1 => Instruction::Rrca,
          U3::_2 => Instruction::Rla,
          U3::_3 => Instruction::Rra,
          U3::_4 => Instruction::Daa,
          U3::_5 => Instruction::Cpl,
          U3::_6 => Instruction::Scf,
          U3::_7 => Instruction::Ccf,
        },
      },
      U2::_1 => {
        if (z as u8 == 6) & (y as u8 == 6) {
          Instruction::Halt
        } else {
          Instruction::LdR8mR8m(R8m::new(y), R8m::new(z))
        }
      }
      U2::_2 => Instruction::AluR8m(Alu::new(y), R8m::new(z)),
      U2::_3 => match z {
        U3::_0 => match y {
          U3::_0 | U3::_1 | U3::_2 | U3::_3 => {
            Instruction::ReturnCond(Cond::new(y as u8))
          }
          U3::_4 => Instruction::LdhImm8ToA(0, false),
          U3::_5 => Instruction::AddSPImm8(0),
          U3::_6 => Instruction::LdhImm8ToA(0, true),
          U3::_7 => Instruction::LdHLSPImm8(0),
        },
        U3::_1 => {
          if q {
            match p {
              U2::_0 => Instruction::Return,
              U2::_1 => Instruction::ReturnIrq,
              U2::_2 => Instruction::JumpHL,
              U2::_3 => Instruction::LdSPHL,
            }
          } else {
            Instruction::Pop(R16f::new(p))
          }
        }
        U3::_2 => match y {
          U3::_0 | U3::_1 | U3::_2 | U3::_3 => {
            Instruction::JumpCond(Cond::new(y as u8), 0)
          }
          U3::_4 => Instruction::LdhCToA(false),
          U3::_5 => Instruction::LdImm16ToA(false),
          U3::_6 => Instruction::LdhCToA(true),
          U3::_7 => Instruction::LdImm16ToA(true),
        },
        U3::_3 => match y {
          U3::_0 => Instruction::JumpImm16(0),
          U3::_1 => Instruction::Cb(PrefixedOp::default()),
          U3::_2 => Instruction::Illegal(IllegalOpByte::D3),
          U3::_3 => Instruction::Illegal(IllegalOpByte::DB),
          U3::_4 => Instruction::Illegal(IllegalOpByte::E3),
          U3::_5 => Instruction::Illegal(IllegalOpByte::EB),
          U3::_6 => Instruction::DI,
          U3::_7 => Instruction::EI,
        },
        U3::_4 => match y {
          U3::_0 | U3::_1 | U3::_2 | U3::_3 => {
            Instruction::CallCond(Cond::new(y as u8), 0)
          }
          U3::_4 => Instruction::Illegal(IllegalOpByte::E4),
          U3::_5 => Instruction::Illegal(IllegalOpByte::EC),
          U3::_6 => Instruction::Illegal(IllegalOpByte::F4),
          U3::_7 => Instruction::Illegal(IllegalOpByte::FC),
        },
        U3::_5 => {
          if q {
            match p {
              U2::_0 => Instruction::Call(0_u16),
              U2::_1 => Instruction::Illegal(IllegalOpByte::DD),
              U2::_2 => Instruction::Illegal(IllegalOpByte::ED),
              U2::_3 => Instruction::Illegal(IllegalOpByte::FD),
            }
          } else {
            Instruction::Push(R16p::new(p))
          }
        }
        U3::_6 => Instruction::AluImm8(Alu::new(y), 0),
        U3::_7 => Instruction::Restart(y),
      },
    }
  }
}

#[test]
fn test_Instruction_new() {
  for op_code in 0..=u8::MAX {
    Instruction::new(op_code);
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrefixedOp {
  RotR8m(Rot, R8m),
  Bit(U3, R8m),
  Res(U3, R8m),
  Set(U3, R8m),
}
impl Default for PrefixedOp {
  fn default() -> Self {
    PrefixedOp::RotR8m(Rot::default(), R8m::default())
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rot {
  #[default]
  Rlc = 0,
  Rrc = 1,
  Rl = 2,
  RR = 3,
  Sla = 4,
  Sra = 5,
  Swap = 6,
  Srl = 7,
}
impl Rot {
  pub const fn new(u: U3) -> Self {
    match u {
      U3::_0 => Self::Rlc,
      U3::_1 => Self::Rrc,
      U3::_2 => Self::Rl,
      U3::_3 => Self::RR,
      U3::_4 => Self::Sla,
      U3::_5 => Self::Sra,
      U3::_6 => Self::Swap,
      U3::_7 => Self::Srl,
    }
  }
}

pub fn instruction_length(op_code: u8) -> usize {
  let x = U2::new_from_byte(6, op_code);
  let y = U3::new_from_byte(3, op_code);
  let z = U3::new_from_byte(0, op_code);
  let p = U2::new_from_byte(4, op_code);
  let q = u8_get_bit(3, op_code);
  //
  match x {
    U2::_0 => match z {
      U3::_0 => match y {
        U3::_0 => 1,
        U3::_1 => 3,
        U3::_2 => 1,
        U3::_3 => 2,
        U3::_4 | U3::_5 | U3::_6 | U3::_7 => 2,
      },
      U3::_1 => {
        if q {
          1
        } else {
          3
        }
      }
      U3::_2 => 1,
      U3::_3 => 1,
      U3::_4 => 1,
      U3::_5 => 1,
      U3::_6 => 2,
      U3::_7 => 1,
    },
    U2::_1 => {
      if (z as u8 == 6) & (y as u8 == 6) {
        1
      } else {
        1
      }
    }
    U2::_2 => 1,
    U2::_3 => match z {
      U3::_0 => match y {
        U3::_0 | U3::_1 | U3::_2 | U3::_3 => 1,
        U3::_4 => 2,
        U3::_5 => 2,
        U3::_6 => 2,
        U3::_7 => 2,
      },
      U3::_1 => {
        if q {
          1
        } else {
          1
        }
      }
      U3::_2 => match y {
        U3::_0 | U3::_1 | U3::_2 | U3::_3 => 3,
        U3::_4 => 1,
        U3::_5 => 1,
        U3::_6 => 1,
        U3::_7 => 1,
      },
      U3::_3 => match y {
        U3::_0 => 3,
        U3::_1 => 2,
        U3::_2 => 1,
        U3::_3 => 1,
        U3::_4 => 1,
        U3::_5 => 1,
        U3::_6 => 1,
        U3::_7 => 1,
      },
      U3::_4 => match y {
        U3::_0 | U3::_1 | U3::_2 | U3::_3 => 3,
        U3::_4 => 1,
        U3::_5 => 1,
        U3::_6 => 1,
        U3::_7 => 1,
      },
      U3::_5 => {
        if q {
          match p {
            U2::_0 => 3,
            U2::_1 => 1,
            U2::_2 => 1,
            U2::_3 => 1,
          }
        } else {
          1
        }
      }
      U3::_6 => 2,
      U3::_7 => 1,
    },
  }
}
