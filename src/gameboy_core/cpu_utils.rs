use std::{
    fs::File,
    io::{self, Read, Write},
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

pub fn log_state(cpu: &Cpu, opcode: u8) -> io::Result<()> {
    if cpu.is_debug_mode {
        let file_path = "instructions_log.txt";
        
        // Format the log line
        let log_line = format!(
            "Fetched opcode: 0x{:02X} (binary: 0b{:08b}) at PC: 0x{:04X}\n",
            opcode, opcode, cpu.registers.pc
        );
        
        // Open the file in append mode and write the log line
        let mut file = File::options()
            .create(true)
            .append(true)
            .open(file_path)?;
        
        file.write_all(log_line.as_bytes())?;
    }
    
    Ok(())
}

/// Prints the CPU registers and flags register to the console
pub fn print_state(cpu: &Cpu) {
    println!("\n========= Current CPU State before execute function ============");
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
    println!("  Z (Zero):     {}", cpu.registers.flags.z);
    println!("  N (Subtract): {}", cpu.registers.flags.n);
    println!("  H (Half-carry): {}", cpu.registers.flags.h);
    println!("  C (Carry):    {}", cpu.registers.flags.c);
    println!("================================================================\n");
}

/// Appends a line to a Dr. Gameboy log file with CPU state in the format:
/// A:00 F:11 B:22 C:33 D:44 E:55 H:66 L:77 SP:8888 PC:9999 PCMEM:AA,BB,CC,DD
pub fn log_to_dr_gameboy(cpu: &Cpu, pc_before_increment: u16) -> io::Result<()> {
    let file_path = "dr_gameboy_log.txt";
    
    // Get the flags register as a u8 value
    let flags_value = cpu.registers.flags.get_flags_as_u8();
    
    // Read the 4 bytes at PC and PC+1, PC+2, PC+3
    let pc_mem_0 = cpu.memory_bus.read_byte(pc_before_increment);
    let pc_mem_1 = cpu.memory_bus.read_byte(pc_before_increment.wrapping_add(1));
    let pc_mem_2 = cpu.memory_bus.read_byte(pc_before_increment.wrapping_add(2));
    let pc_mem_3 = cpu.memory_bus.read_byte(pc_before_increment.wrapping_add(3));
    
    // Format the log line
    let log_line = format!(
        "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}\n",
        cpu.registers.a,
        flags_value,
        cpu.registers.b,
        cpu.registers.c,
        cpu.registers.d,
        cpu.registers.e,
        cpu.registers.h,
        cpu.registers.l,
        cpu.registers.sp,
        pc_before_increment,
        pc_mem_0,
        pc_mem_1,
        pc_mem_2,
        pc_mem_3
    );
    
    // Open the file in append mode and write the log line
    let mut file = File::options()
        .create(true)
        .append(true)
        .open(file_path)?;
    
    file.write_all(log_line.as_bytes())?;
    
    Ok(())
}

pub(crate) fn clear_dr_gameboy_log() -> io::Result<()> {
    let file_path = "dr_gameboy_log.txt";
    let mut file = File::create(file_path)?;
    file.set_len(0)?;
    Ok(())
}
