mod emulator;
mod opcodes;
mod ui;
mod disassembler;

use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "term-8")]
#[command(about = "A CHIP-8 emulator with interactive debugger", long_about = None)]
struct Args {
    rom_file: PathBuf,

    #[arg(short, long, default_value_t = 10)]
    speed: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let rom_data = fs::read(&args.rom_file)?;
    
    let mut emulator = emulator::Emulator::new();
    emulator.load_rom(&rom_data);

    ui::run(emulator, args.speed)?;

    Ok(())
}


