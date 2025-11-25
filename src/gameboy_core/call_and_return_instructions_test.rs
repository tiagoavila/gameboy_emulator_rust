#[cfg(test)]
mod tests {
    use crate::gameboy_core::cpu::Cpu;

    #[test]
    fn test_di_instruction() {
        let mut cpu = Cpu::new();
        
        // Initially, IME should be false
        assert_eq!(cpu.ime, false, "IME should be initially false");
        assert_eq!(cpu.di_instruction_pending, false, "di_instruction_pending should be initially false");

        // Execute DI instruction
        let di_opcode = 0xF3; // DI opcode
        cpu.execute(di_opcode);
        
        // After executing DI, IME should still be false (not immediately disabled)
        // but di_instruction_pending should be set to true
        assert_eq!(cpu.ime, false, "IME should still be false immediately after DI");
        assert_eq!(cpu.di_instruction_pending, true, "di_instruction_pending should be true after DI");
    }

    #[test]
    fn test_di_instruction_disables_ime_after_next_instruction() {
        let mut cpu = Cpu::new();
        
        // Set IME to true so we can verify it gets disabled
        cpu.ime = true;
        assert_eq!(cpu.ime, true, "IME should be true initially");

        // Execute DI instruction (0xF3)
        let di_opcode = 0xF3;
        cpu.execute(di_opcode);
        
        // After DI, IME should still be true (not immediately disabled)
        assert_eq!(cpu.ime, true, "IME should still be true immediately after DI");
        assert_eq!(cpu.di_instruction_pending, true, "di_instruction_pending should be true");

        // Execute next instruction (NOP - 0x00)
        cpu.tick(); // This will fetch, decode, execute NOP and then check di_instruction_pending
        
        // After the next instruction, IME should be disabled
        assert_eq!(cpu.ime, false, "IME should be false after the instruction following DI");
        assert_eq!(cpu.di_instruction_pending, false, "di_instruction_pending should be reset to false");
    }

    #[test]
    fn test_rst_1_instruction() {
        let mut cpu = Cpu::new();
        
        // Set up initial state
        // PC is at 0x8000, after fetch and increment it will be 0x8001
        cpu.registers.pc = 0x8001;
        
        // Initialize SP to a safe location in RAM (0xFFFF)
        cpu.registers.sp = 0xFFFF;
        
        let rst_1_opcode = 0xCF;
        
        // Execute RST 1 instruction
        cpu.execute(rst_1_opcode);
        
        // Verify PC was set to 0x0008
        assert_eq!(cpu.registers.pc, 0x0008, "PC should be set to 0x0008 after RST 1");
        
        // Verify that 0x8001 was pushed onto the stack
        // push_value_to_sp decrements SP by 2, so SP should be at 0xFFFD
        assert_eq!(cpu.registers.sp, 0xFFFD, "SP should be decremented by 2");
        
        // Read the pushed value from memory (little-endian)
        // Low byte is at SP, high byte is at SP + 1. In little endian, low byte comes first.
        let low_byte = cpu.memory_bus.read_byte(cpu.registers.sp);
        let high_byte = cpu.memory_bus.read_byte(cpu.registers.sp + 1);
        let pushed_value = ((high_byte as u16) << 8) | (low_byte as u16);
        
        assert_eq!(pushed_value, 0x8001, "The value 0x8001 should be pushed onto the stack");
    }
}
