use std::{
    fs::File,
    io::{self, Read},
};

use crate::gameboy_core::cpu::Cpu;

/// Reads a ROM file from the specified path and returns its contents as a vector of bytes.
///
/// # Errors
/// Returns an error if the file cannot be read.
pub fn read_rom(file_path: &str) -> io::Result<Vec<u8>> {
    // Open the file
    let file = File::open(file_path)?;
    let mut reader = io::BufReader::new(file);

    let mut buffer: Vec<u8> = Vec::new();

    reader.read_to_end(&mut buffer)?;
    // Collect the lines into a vector
    // let lines: Vec<String> = buffer.iter().collect();
    Ok(buffer)
}

pub fn print_state_if_debug_mode(cpu: &Cpu, opcode: u8) {
    if cpu.is_debug_mode {
        println!(
            "Fetched opcode: 0x{:02X} (binary: 0b{:08b}) at PC: 0x{:04X}",
            opcode, opcode, cpu.registers.pc
        );
        print_state(cpu);
    }
}

/// Prints the CPU registers and flags register to the console
pub fn print_state(cpu: &Cpu) {
    println!("\n=== Current CPU State before execute function ===");
    println!("8-bit Registers:");
    println!("  A:  0x{:02X} ({})", cpu.registers.a, cpu.registers.a);
    println!("  B:  0x{:02X} ({})", cpu.registers.b, cpu.registers.b);
    println!("  C:  0x{:02X} ({})", cpu.registers.c, cpu.registers.c);
    println!("  D:  0x{:02X} ({})", cpu.registers.d, cpu.registers.d);
    println!("  E:  0x{:02X} ({})", cpu.registers.e, cpu.registers.e);
    println!("  H:  0x{:02X} ({})", cpu.registers.h, cpu.registers.h);
    println!("  L:  0x{:02X} ({})", cpu.registers.l, cpu.registers.l);

    println!("\n16-bit Registers:");
    println!(
        "  BC: 0x{:04X} ({})",
        cpu.registers.get_bc(),
        cpu.registers.get_bc()
    );
    println!(
        "  DE: 0x{:04X} ({})",
        cpu.registers.get_de(),
        cpu.registers.get_de()
    );
    println!(
        "  HL: 0x{:04X} ({})",
        cpu.registers.get_hl(),
        cpu.registers.get_hl()
    );
    println!("  SP: 0x{:04X} ({})", cpu.registers.sp, cpu.registers.sp);
    println!("  PC: 0x{:04X} ({})", cpu.registers.pc, cpu.registers.pc);

    println!("\nFlags Register:");
    println!("  Z (Zero):     {}", cpu.flags_register.z_flag);
    println!("  N (Subtract): {}", cpu.flags_register.n_flag);
    println!("  H (Half-carry): {}", cpu.flags_register.h_flag);
    println!("  C (Carry):    {}", cpu.flags_register.c_flag);
    println!("================\n");
}
