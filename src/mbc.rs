use alloc::{boxed::Box, vec::Vec};

use crate::data_bus::DataBus;

pub struct MBC1 {
  rom: Vec<u8>,
}
impl MBC1 {
  pub fn new_boxed(rom: Vec<u8>) -> Box<Self> {
    Box::new(Self { rom })
  }
}

impl DataBus for MBC1 {
  fn read(&self, addr: u16) -> u8 {
    assert!(addr <= 0x3FFF, "illegal read: {addr:04X}");
    self.rom[addr as usize]
  }
  fn write(&mut self, addr: u16, byte: u8) {
    println!("Wrote ${byte:02X} to ${addr:04X}")
    /* TODO */
  }
}
