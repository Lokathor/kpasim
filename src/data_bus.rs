pub trait DataBus {
  fn read(&self, addr: u16) -> u8;
  fn write(&mut self, addr: u16, byte: u8);
}
