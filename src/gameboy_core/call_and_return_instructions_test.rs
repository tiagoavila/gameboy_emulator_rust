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
    fn test_di_instruction_pending_flag_only_affects_di() {
        let mut cpu = Cpu::new();
        
        // Set IME to true
        cpu.ime = true;
        
        // Execute DI instruction
        let di_opcode = 0xF3;
        cpu.execute(di_opcode);
        
        // Verify di_instruction_pending is set
        assert_eq!(cpu.di_instruction_pending, true, "di_instruction_pending should be true after DI");
        assert_eq!(cpu.ime, true, "IME should still be true");

        // Execute a different instruction (INC A - 0x3C)
        cpu.registers.pc = 0; // Reset PC
        let inc_opcode = 0x3C;
        let original_a = cpu.registers.a;
        cpu.execute(inc_opcode);
        
        // After a non-DI instruction, IME should be disabled
        assert_eq!(cpu.ime, false, "IME should be false after non-DI instruction following DI");
        assert_eq!(cpu.di_instruction_pending, false, "di_instruction_pending should be reset");
        assert_ne!(cpu.registers.a, original_a, "INC A should have incremented A");
    }
}
