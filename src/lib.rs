//#![no_std]
#![allow(unused)]
#![cfg_attr(test, allow(nonstandard_style))]

extern crate alloc;

pub mod cpu;
pub mod data_bus;
pub mod mbc;
pub mod op_actions;
pub mod op_disassembly;
pub mod reg16;
pub mod reg8;
pub mod reg_flags;
