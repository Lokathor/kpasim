use kpasim::{cpu::Cpu, data_bus::DataBus, mbc::MBC1};

fn main() {
  let args: Vec<String> = std::env::args().collect();
  println!("ARGS: {args:?}");
  if args.len() < 2 {
    println!("expected a rom as arg[1]");
    return;
  }
  let path = std::path::Path::new(&args[1]);
  print!("Reading `{}`... ", path.display());
  let bytes = match std::fs::read(path) {
    Ok(bytes) => {
      println!("got {} bytes.", bytes.len());
      bytes
    }
    Err(e) => {
      println!("{e:?}");
      return;
    }
  };

  let mut bus: Box<dyn DataBus> = match bytes[0x0147] {
    0x01 => MBC1::new_boxed(bytes),
    unknown => {
      println!("Cart type 0x{unknown:02X} unsupported... exiting.");
      return;
    }
  };

  let mut cpu = Cpu::new();
  println!("==== First Boot");
  println!(">> {cpu:?}");

  loop {
    if cpu.t_cycle(&mut *bus) {
      println!(">> {cpu:?}");
    }
  }
}
