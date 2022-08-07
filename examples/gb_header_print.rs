//! prints cart header info
//!
//! https://gbdev.io/pandocs/The_Cartridge_Header.html

fn main() {
  let args: Vec<String> = std::env::args().collect();
  println!("ARGS: {args:?}");
  for file_arg in args[1..].iter() {
    let path = std::path::Path::new(file_arg);
    print!("Reading `{}`... ", path.display());
    let bytes = match std::fs::read(path) {
      Ok(bytes) => {
        println!("got {} bytes.", bytes.len());
        bytes
      }
      Err(e) => {
        println!("{e:?}");
        continue;
      }
    };

    let entry_point = &bytes[0x0100..=0x0103];
    println!("entry_point: {entry_point:?}");

    let logo = &bytes[0x0104..=0x0133];
    print!("logo:");
    for (i, byte) in logo.iter().enumerate() {
      if i % 16 == 0 {
        println!();
      } else {
        print!(" ");
      }
      print!("{byte:02X}");
    }
    println!();

    let title_bytes = &bytes[0x0134..=0x0143];
    match core::str::from_utf8(title_bytes) {
      Ok(title) => println!("title: {title:?}"),
      Err(_) => println!("title: {:?}", String::from_utf8_lossy(title_bytes)),
    };

    let manufacture_code = &bytes[0x013F..=0x0142];
    match core::str::from_utf8(manufacture_code) {
      Ok(title) => println!("manufacture_code: {title:?}"),
      Err(_) => {
        println!(
          "manufacture_code: {:?}",
          String::from_utf8_lossy(manufacture_code)
        )
      }
    };

    let cgb_flag = bytes[0x0143];
    print!("cgb_flag: ");
    match cgb_flag {
      0x80 => println!("Both GCB and DMG"),
      0xC0 => println!("CGB Only"),
      _ => println!("No Color"),
    };

    let new_licensee_code = &bytes[0x0144..=0x0145];
    match core::str::from_utf8(new_licensee_code) {
      Ok(nlc) => println!("new_licensee_code: {nlc:?}"),
      Err(_) => println!(
        "new_licensee_code: {:?}",
        String::from_utf8_lossy(new_licensee_code)
      ),
    };

    let sgb_flag = bytes[0x0146];
    print!("sgb_flag: ");
    match sgb_flag {
      0x03 => println!("true"),
      _ => println!("false"),
    };

    let cart_type = bytes[0x0147];
    print!("cart_type: ");
    match cart_type {
      0x00 => println!("RomOnly"),
      0x01 => println!("MBC1"),
      other => println!("Unknown({other:?})"),
    };

    let rom_size = bytes[0x0148];
    println!("rom_size: {}kb", 32 << rom_size);

    let ram_size = bytes[0x0149];
    print!("ram_size: ");
    match ram_size {
      0x00 => println!("none"),
      0x01 => println!("2kb"),
      0x02 => println!("8kb"),
      0x03 => println!("32kb (4x8kb)"),
      0x04 => println!("128kb (16x8kb)"),
      0x05 => println!("64kb (8x8kb)"),
      other => println!("Unknown({other:02X})"),
    }

    let destination_code = bytes[0x014A];
    print!("destination_code: ");
    match destination_code {
      0x00 => println!("Japan"),
      0x01 => println!("non-Japan"),
      other => println!("Unknown({other:?})"),
    };

    let old_licensee_code = bytes[0x014B];
    println!("old_licensee_code: {old_licensee_code:02X}");

    let mask_rom_version = bytes[0x014C];
    println!("mask_rom_version: {mask_rom_version}");

    let checksum_byte = bytes[0x014D];
    println!("checksum_byte: {checksum_byte:02X}");

    let global_checksum =
      u16::from_be_bytes(bytes[0x014E..=0x14F].try_into().unwrap());
    println!("global_checksum: {global_checksum:04X}");
  }
}
