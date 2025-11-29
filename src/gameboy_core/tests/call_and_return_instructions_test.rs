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
    fn test_call_imm16_basic() {
        let mut cpu = Cpu::new();
        
        // Set up initial state according to the provided data
        let initial_pc = 0x8000;
        cpu.registers.pc = initial_pc;
        cpu.registers.sp = 0xFFFE;
        
        // Set up memory with CALL 1234H opcode (0xCD) at PC
        cpu.memory_bus.write_byte(initial_pc, 0xCD); // CALL nn opcode
        // Set up the 16-bit immediate value (little-endian: 0x1234)
        cpu.memory_bus.write_byte(initial_pc + 1, 0x34); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0x12); // High byte
        
        // Increment PC as tick() would do
        cpu.registers.increment_pc();
        // Now PC = 0x8001
        
        // Execute CALL imm16 via execute function
        cpu.execute(0xCD);
        
        // Verify PC was set to 0x1234
        assert_eq!(cpu.registers.pc, 0x1234, "PC should be set to 0x1234 after CALL");
        
        // Verify SP was decremented by 2
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should be decremented by 2 to 0xFFFC");
        
        // Verify that 0x8003 was pushed onto the stack (return address)
        // According to the data: (FFFDH) ← 80H, (FFFCH) ← 03H
        let low_byte = cpu.memory_bus.read_byte(0xFFFC); // Should be 03H
        let high_byte = cpu.memory_bus.read_byte(0xFFFD); // Should be 80H
        let pushed_value = ((high_byte as u16) << 8) | (low_byte as u16);
        
        assert_eq!(low_byte, 0x03, "Low byte at (FFFCH) should be 03H");
        assert_eq!(high_byte, 0x80, "High byte at (FFFDH) should be 80H");
        assert_eq!(pushed_value, 0x8003, "Return address 0x8003 should be pushed onto stack");
    }

    #[test]
    fn test_call_imm16_with_different_target_address() {
        let mut cpu = Cpu::new();
        
        let initial_pc = 0x2000;
        cpu.registers.pc = initial_pc;
        cpu.registers.sp = 0xFFFE;
        
        // Set up CALL to different address (0x3456)
        cpu.memory_bus.write_byte(initial_pc, 0xCD); // CALL nn opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0x56); // Low byte (0x56)
        cpu.memory_bus.write_byte(initial_pc + 2, 0x34); // High byte (0x34)
        
        cpu.registers.increment_pc();
        cpu.execute(0xCD);
        
        // PC should jump to 0x3456
        assert_eq!(cpu.registers.pc, 0x3456, "PC should jump to 0x3456");
        
        // SP should be decremented by 2
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should be 0xFFFC");
        
        // Return address should be 0x2003 (initial_pc + 3 for the 3-byte CALL instruction)
        let pushed_value = ((cpu.memory_bus.read_byte(0xFFFD) as u16) << 8) 
                          | (cpu.memory_bus.read_byte(0xFFFC) as u16);
        assert_eq!(pushed_value, 0x2003, "Return address should be 0x2003");
    }

    #[test]
    fn test_call_imm16_with_zero_address() {
        let mut cpu = Cpu::new();
        
        let initial_pc = 0x4000;
        cpu.registers.pc = initial_pc;
        cpu.registers.sp = 0xFFFE;
        
        // CALL to address 0x0000
        cpu.memory_bus.write_byte(initial_pc, 0xCD);
        cpu.memory_bus.write_byte(initial_pc + 1, 0x00); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0x00); // High byte
        
        cpu.registers.increment_pc();
        cpu.execute(0xCD);
        
        assert_eq!(cpu.registers.pc, 0x0000, "PC should jump to 0x0000");
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should be decremented by 2");
        
        let pushed_value = ((cpu.memory_bus.read_byte(0xFFFD) as u16) << 8)
                          | (cpu.memory_bus.read_byte(0xFFFC) as u16);
        assert_eq!(pushed_value, 0x4003, "Return address should be 0x4003");
    }

    #[test]
    fn test_call_imm16_with_max_address() {
        let mut cpu = Cpu::new();
        
        let initial_pc = 0x6000;
        cpu.registers.pc = initial_pc;
        cpu.registers.sp = 0xFFFE;
        
        // CALL to address 0xFFFF (maximum address)
        cpu.memory_bus.write_byte(initial_pc, 0xCD);
        cpu.memory_bus.write_byte(initial_pc + 1, 0xFF); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0xFF); // High byte
        
        cpu.registers.increment_pc();
        cpu.execute(0xCD);
        
        assert_eq!(cpu.registers.pc, 0xFFFF, "PC should jump to 0xFFFF");
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should be decremented by 2");
        
        let pushed_value = ((cpu.memory_bus.read_byte(0xFFFD) as u16) << 8)
                          | (cpu.memory_bus.read_byte(0xFFFC) as u16);
        assert_eq!(pushed_value, 0x6003, "Return address should be 0x6003");
    }

    #[test]
    fn test_call_imm16_sp_decrement() {
        let mut cpu = Cpu::new();
        
        let initial_pc = 0x1000;
        cpu.registers.pc = initial_pc;
        cpu.registers.sp = 0x0100; // Start with SP at 0x0100
        
        cpu.memory_bus.write_byte(initial_pc, 0xCD);
        cpu.memory_bus.write_byte(initial_pc + 1, 0x00);
        cpu.memory_bus.write_byte(initial_pc + 2, 0x20);
        
        cpu.registers.increment_pc();
        cpu.execute(0xCD);
        
        // SP should be decremented from 0x0100 to 0x00FE
        assert_eq!(cpu.registers.sp, 0x00FE, "SP should be decremented by 2");
        
        // Return address should be at 0x00FE and 0x00FF
        let pushed_value = ((cpu.memory_bus.read_byte(0x00FF) as u16) << 8)
                          | (cpu.memory_bus.read_byte(0x00FE) as u16);
        assert_eq!(pushed_value, 0x1003, "Return address should be pushed correctly");
    }

    #[test]
    fn test_call_imm16_multiple_sequential_calls() {
        let mut cpu = Cpu::new();
        
        // First CALL from 0x5000
        cpu.registers.pc = 0x5000;
        cpu.registers.sp = 0xFFFE;
        
        cpu.memory_bus.write_byte(0x5000, 0xCD);
        cpu.memory_bus.write_byte(0x5001, 0x00);
        cpu.memory_bus.write_byte(0x5002, 0x30);
        
        cpu.registers.increment_pc();
        cpu.execute(0xCD);
        
        assert_eq!(cpu.registers.pc, 0x3000, "First call should jump to 0x3000");
        assert_eq!(cpu.registers.sp, 0xFFFC, "First call: SP should be 0xFFFC");
        
        let first_return = ((cpu.memory_bus.read_byte(0xFFFD) as u16) << 8)
                          | (cpu.memory_bus.read_byte(0xFFFC) as u16);
        assert_eq!(first_return, 0x5003, "First return address should be 0x5003");
        
        // Second CALL from 0x3000 (nested call)
        cpu.registers.pc = 0x3000;
        
        cpu.memory_bus.write_byte(0x3000, 0xCD);
        cpu.memory_bus.write_byte(0x3001, 0x00);
        cpu.memory_bus.write_byte(0x3002, 0x40);
        
        cpu.registers.increment_pc();
        cpu.execute(0xCD);
        
        assert_eq!(cpu.registers.pc, 0x4000, "Second call should jump to 0x4000");
        assert_eq!(cpu.registers.sp, 0xFFFA, "Second call: SP should be 0xFFFA");
        
        let second_return = ((cpu.memory_bus.read_byte(0xFFFB) as u16) << 8)
                           | (cpu.memory_bus.read_byte(0xFFFA) as u16);
        assert_eq!(second_return, 0x3003, "Second return address should be 0x3003");
    }

    #[test]
    fn test_call_cc_imm16_nz_when_z_flag_clear() {
        let mut cpu = Cpu::new();
        
        // Set Z flag to false (condition NZ is true)
        cpu.flags_register.z = false;
        
        let initial_pc = 0x7FFC;
        cpu.registers.pc = initial_pc;
        cpu.registers.sp = 0xFFFE;
        
        // Set up CALL NZ, 1234h opcode (0xC4)
        cpu.memory_bus.write_byte(initial_pc, 0xC4); // CALL NZ opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0x34); // Low byte of 0x1234
        cpu.memory_bus.write_byte(initial_pc + 2, 0x12); // High byte of 0x1234
        
        // Increment PC as tick() would do
        cpu.registers.increment_pc();
        
        // Execute CALL NZ via execute function
        cpu.execute(0xC4);
        
        // Since Z flag is false, NZ condition is true, so CALL should execute
        // PC should jump to 0x1234
        assert_eq!(cpu.registers.pc, 0x1234, "PC should jump to 0x1234 when NZ condition is true");
        
        // SP should be decremented by 2
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should be decremented by 2");
        
        // Return address should be 0x7FFF (initial_pc + 3)
        let pushed_value = ((cpu.memory_bus.read_byte(0xFFFD) as u16) << 8)
                          | (cpu.memory_bus.read_byte(0xFFFC) as u16);
        assert_eq!(pushed_value, 0x7FFF, "Return address should be 0x7FFF");
    }

    #[test]
    fn test_call_cc_imm16_nz_when_z_flag_set() {
        let mut cpu = Cpu::new();
        
        // Set Z flag to true (condition NZ is false)
        cpu.flags_register.z = true;
        
        let initial_pc = 0x7FFC;
        cpu.registers.pc = initial_pc;
        cpu.registers.sp = 0xFFFE;
        
        // Set up CALL NZ, 1234h opcode (0xC4)
        cpu.memory_bus.write_byte(initial_pc, 0xC4); // CALL NZ opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0x34); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0x12); // High byte
        
        cpu.registers.increment_pc();
        
        let initial_cycles = cpu.cycles;
        
        // Execute CALL NZ via execute function
        cpu.execute(0xC4);
        
        // Since Z flag is true, NZ condition is false, so CALL should NOT execute
        // PC should only increment past the immediate values (to 0x7FFF)
        assert_eq!(cpu.registers.pc, 0x7FFF, "PC should move to next instruction when NZ condition is false");
        
        // SP should not change
        assert_eq!(cpu.registers.sp, 0xFFFE, "SP should not change when condition is false");
    }

    #[test]
    fn test_call_cc_imm16_z_when_z_flag_set() {
        let mut cpu = Cpu::new();
        
        // Set Z flag to true (condition Z is true)
        cpu.flags_register.z = true;
        
        let initial_pc = 0x8000;
        cpu.registers.pc = initial_pc;
        cpu.registers.sp = 0xFFFE;
        
        // Set up CALL Z, 1234h opcode (0xCC)
        cpu.memory_bus.write_byte(initial_pc, 0xCC); // CALL Z opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0x34); // Low byte of 0x1234
        cpu.memory_bus.write_byte(initial_pc + 2, 0x12); // High byte of 0x1234
        
        cpu.registers.increment_pc();
        
        // Execute CALL Z via execute function
        cpu.execute(0xCC);
        
        // Since Z flag is true, Z condition is true, so CALL should execute
        // PC should jump to 0x1234
        assert_eq!(cpu.registers.pc, 0x1234, "PC should jump to 0x1234 when Z condition is true");
        
        // SP should be decremented by 2
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should be decremented by 2");
        
        // According to the data: Pushes 8003h to the stack
        let pushed_value = ((cpu.memory_bus.read_byte(0xFFFD) as u16) << 8)
                          | (cpu.memory_bus.read_byte(0xFFFC) as u16);
        assert_eq!(pushed_value, 0x8003, "Return address should be 0x8003");
    }

    #[test]
    fn test_call_cc_imm16_z_when_z_flag_clear() {
        let mut cpu = Cpu::new();
        
        // Set Z flag to false (condition Z is false)
        cpu.flags_register.z = false;
        
        let initial_pc = 0x8000;
        cpu.registers.pc = initial_pc;
        cpu.registers.sp = 0xFFFE;
        
        // Set up CALL Z, 1234h opcode (0xCC)
        cpu.memory_bus.write_byte(initial_pc, 0xCC); // CALL Z opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0x34); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0x12); // High byte
        
        cpu.registers.increment_pc();
        
        // Execute CALL Z via execute function
        cpu.execute(0xCC);
        
        // Since Z flag is false, Z condition is false, so CALL should NOT execute
        // PC should move to next instruction (to 0x8003)
        assert_eq!(cpu.registers.pc, 0x8003, "PC should move to next instruction when Z condition is false");
        
        // SP should not change
        assert_eq!(cpu.registers.sp, 0xFFFE, "SP should not change when condition is false");
    }

    #[test]
    fn test_call_cc_imm16_nc_when_c_flag_clear() {
        let mut cpu = Cpu::new();
        
        // Set C flag to false (condition NC is true)
        cpu.flags_register.c = false;
        
        let initial_pc = 0x6000;
        cpu.registers.pc = initial_pc;
        cpu.registers.sp = 0xFFFE;
        
        // Set up CALL NC, 5678h opcode (0xD4)
        cpu.memory_bus.write_byte(initial_pc, 0xD4); // CALL NC opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0x78); // Low byte of 0x5678
        cpu.memory_bus.write_byte(initial_pc + 2, 0x56); // High byte of 0x5678
        
        cpu.registers.increment_pc();
        
        // Execute CALL NC via execute function
        cpu.execute(0xD4);
        
        // Since C flag is false, NC condition is true, so CALL should execute
        assert_eq!(cpu.registers.pc, 0x5678, "PC should jump to 0x5678 when NC condition is true");
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should be decremented by 2");
        
        let pushed_value = ((cpu.memory_bus.read_byte(0xFFFD) as u16) << 8)
                          | (cpu.memory_bus.read_byte(0xFFFC) as u16);
        assert_eq!(pushed_value, 0x6003, "Return address should be 0x6003");
    }

    #[test]
    fn test_call_cc_imm16_nc_when_c_flag_set() {
        let mut cpu = Cpu::new();
        
        // Set C flag to true (condition NC is false)
        cpu.flags_register.c = true;
        
        let initial_pc = 0x6000;
        cpu.registers.pc = initial_pc;
        cpu.registers.sp = 0xFFFE;
        
        // Set up CALL NC, 5678h opcode (0xD4)
        cpu.memory_bus.write_byte(initial_pc, 0xD4); // CALL NC opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0x78); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0x56); // High byte
        
        cpu.registers.increment_pc();
        
        // Execute CALL NC via execute function
        cpu.execute(0xD4);
        
        // Since C flag is true, NC condition is false, so CALL should NOT execute
        assert_eq!(cpu.registers.pc, 0x6003, "PC should move to next instruction when NC condition is false");
        assert_eq!(cpu.registers.sp, 0xFFFE, "SP should not change when condition is false");
    }

    #[test]
    fn test_call_cc_imm16_c_when_c_flag_set() {
        let mut cpu = Cpu::new();
        
        // Set C flag to true (condition C is true)
        cpu.flags_register.c = true;
        
        let initial_pc = 0x5000;
        cpu.registers.pc = initial_pc;
        cpu.registers.sp = 0xFFFE;
        
        // Set up CALL C, ABCDh opcode (0xDC)
        cpu.memory_bus.write_byte(initial_pc, 0xDC); // CALL C opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0xCD); // Low byte of 0xABCD
        cpu.memory_bus.write_byte(initial_pc + 2, 0xAB); // High byte of 0xABCD
        
        cpu.registers.increment_pc();
        
        // Execute CALL C via execute function
        cpu.execute(0xDC);
        
        // Since C flag is true, C condition is true, so CALL should execute
        assert_eq!(cpu.registers.pc, 0xABCD, "PC should jump to 0xABCD when C condition is true");
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should be decremented by 2");
        
        let pushed_value = ((cpu.memory_bus.read_byte(0xFFFD) as u16) << 8)
                          | (cpu.memory_bus.read_byte(0xFFFC) as u16);
        assert_eq!(pushed_value, 0x5003, "Return address should be 0x5003");
    }

    #[test]
    fn test_call_cc_imm16_c_when_c_flag_clear() {
        let mut cpu = Cpu::new();
        
        // Set C flag to false (condition C is false)
        cpu.flags_register.c = false;
        
        let initial_pc = 0x5000;
        cpu.registers.pc = initial_pc;
        cpu.registers.sp = 0xFFFE;
        
        // Set up CALL C, ABCDh opcode (0xDC)
        cpu.memory_bus.write_byte(initial_pc, 0xDC); // CALL C opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0xCD); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0xAB); // High byte
        
        cpu.registers.increment_pc();
        
        // Execute CALL C via execute function
        cpu.execute(0xDC);
        
        // Since C flag is false, C condition is false, so CALL should NOT execute
        assert_eq!(cpu.registers.pc, 0x5003, "PC should move to next instruction when C condition is false");
        assert_eq!(cpu.registers.sp, 0xFFFE, "SP should not change when condition is false");
    }

    #[test]
    fn test_call_cc_imm16_all_conditions_true() {
        let mut cpu = Cpu::new();
        
        // Set both flags to test all conditions
        cpu.flags_register.z = true;
        cpu.flags_register.c = true;
        
        // Test Z condition (true)
        cpu.registers.pc = 0x4000;
        cpu.registers.sp = 0xFFFE;
        cpu.memory_bus.write_byte(0x4000, 0xCC); // CALL Z
        cpu.memory_bus.write_byte(0x4001, 0x11);
        cpu.memory_bus.write_byte(0x4002, 0x20);
        cpu.registers.increment_pc();
        cpu.execute(0xCC);
        assert_eq!(cpu.registers.pc, 0x2011, "CALL Z should execute when Z=1");
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should be decremented");
        
        // Test C condition (true)
        cpu.registers.pc = 0x3000;
        cpu.registers.sp = 0xFFFE;
        cpu.memory_bus.write_byte(0x3000, 0xDC); // CALL C
        cpu.memory_bus.write_byte(0x3001, 0x22);
        cpu.memory_bus.write_byte(0x3002, 0x30);
        cpu.registers.increment_pc();
        cpu.execute(0xDC);
        assert_eq!(cpu.registers.pc, 0x3022, "CALL C should execute when C=1");
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should be decremented");
    }

    #[test]
    fn test_call_cc_imm16_all_conditions_false() {
        let mut cpu = Cpu::new();
        
        // Clear both flags to test all conditions
        cpu.flags_register.z = false;
        cpu.flags_register.c = false;
        
        // Test Z condition (false)
        cpu.registers.pc = 0x4000;
        cpu.registers.sp = 0xFFFE;
        cpu.memory_bus.write_byte(0x4000, 0xCC); // CALL Z
        cpu.memory_bus.write_byte(0x4001, 0x11);
        cpu.memory_bus.write_byte(0x4002, 0x20);
        cpu.registers.increment_pc();
        cpu.execute(0xCC);
        assert_eq!(cpu.registers.pc, 0x4003, "CALL Z should not execute when Z=0");
        assert_eq!(cpu.registers.sp, 0xFFFE, "SP should not change");
        
        // Test C condition (false)
        cpu.registers.pc = 0x3000;
        cpu.registers.sp = 0xFFFE;
        cpu.memory_bus.write_byte(0x3000, 0xDC); // CALL C
        cpu.memory_bus.write_byte(0x3001, 0x22);
        cpu.memory_bus.write_byte(0x3002, 0x30);
        cpu.registers.increment_pc();
        cpu.execute(0xDC);
        assert_eq!(cpu.registers.pc, 0x3003, "CALL C should not execute when C=0");
        assert_eq!(cpu.registers.sp, 0xFFFE, "SP should not change");
    }

    #[test]
    fn test_ret_basic() {
        let mut cpu = Cpu::new();
        
        // Set up the scenario from the provided data:
        // CALL 9000H from 8000H pushes return address 8003H to stack
        // RET at 9000H should return to 8003H
        
        let call_address = 0x8000;
        let ret_address = 0x9000;
        
        // First, simulate the CALL instruction
        cpu.registers.pc = call_address;
        cpu.registers.sp = 0xFFFE;
        
        // Set up CALL 9000H opcode (0xCD) at 0x8000
        cpu.memory_bus.write_byte(call_address, 0xCD);
        cpu.memory_bus.write_byte(call_address + 1, 0x00); // Low byte of 0x9000
        cpu.memory_bus.write_byte(call_address + 2, 0x90); // High byte of 0x9000
        
        // Increment PC as tick() would do
        cpu.registers.increment_pc();
        
        // Execute CALL instruction
        cpu.execute(0xCD);
        
        // Verify CALL worked
        assert_eq!(cpu.registers.pc, 0x9000, "PC should be at 0x9000 after CALL");
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should be decremented by 2");
        
        // Verify return address was pushed (0x8003)
        let pushed_value = ((cpu.memory_bus.read_byte(0xFFFD) as u16) << 8)
                          | (cpu.memory_bus.read_byte(0xFFFC) as u16);
        assert_eq!(pushed_value, 0x8003, "Return address 0x8003 should be on stack");
        
        // Now execute RET instruction at 9000H
        // Set up RET opcode (0xC9) at 0x9000
        cpu.memory_bus.write_byte(ret_address, 0xC9);
        
        // Increment PC as tick() would do
        cpu.registers.increment_pc();
        
        // Execute RET instruction
        cpu.execute(0xC9);
        
        // Verify RET worked - PC should return to 0x8003
        assert_eq!(cpu.registers.pc, 0x8003, "PC should return to 0x8003");
        
        // Verify SP was incremented by 2
        assert_eq!(cpu.registers.sp, 0xFFFE, "SP should be incremented by 2 back to 0xFFFE");
    }

    #[test]
    fn test_ret_from_nested_call() {
        let mut cpu = Cpu::new();
        
        // Set up two nested calls and return from the inner one
        cpu.registers.pc = 0x5000;
        cpu.registers.sp = 0xFFFE;
        
        // First CALL from 0x5000 to 0x6000
        cpu.memory_bus.write_byte(0x5000, 0xCD);
        cpu.memory_bus.write_byte(0x5001, 0x00);
        cpu.memory_bus.write_byte(0x5002, 0x60);
        
        cpu.registers.increment_pc();
        cpu.execute(0xCD);
        
        assert_eq!(cpu.registers.pc, 0x6000, "First CALL should jump to 0x6000");
        assert_eq!(cpu.registers.sp, 0xFFFC, "First CALL: SP should be 0xFFFC");
        
        // Verify first return address was pushed (0x5003)
        let first_return = ((cpu.memory_bus.read_byte(0xFFFD) as u16) << 8)
                          | (cpu.memory_bus.read_byte(0xFFFC) as u16);
        assert_eq!(first_return, 0x5003, "First return address should be 0x5003");
        
        // Second CALL from 0x6000 to 0x7000 (nested)
        cpu.memory_bus.write_byte(0x6000, 0xCD);
        cpu.memory_bus.write_byte(0x6001, 0x00);
        cpu.memory_bus.write_byte(0x6002, 0x70);
        
        cpu.registers.increment_pc();
        cpu.execute(0xCD);
        
        assert_eq!(cpu.registers.pc, 0x7000, "Second CALL should jump to 0x7000");
        assert_eq!(cpu.registers.sp, 0xFFFA, "Second CALL: SP should be 0xFFFA");
        
        // Verify second return address was pushed (0x6003)
        let second_return = ((cpu.memory_bus.read_byte(0xFFFB) as u16) << 8)
                           | (cpu.memory_bus.read_byte(0xFFFA) as u16);
        assert_eq!(second_return, 0x6003, "Second return address should be 0x6003");
        
        // Now RET from 0x7000 (inner function)
        cpu.memory_bus.write_byte(0x7000, 0xC9);
        
        cpu.registers.increment_pc();
        cpu.execute(0xC9);
        
        // Should return to 0x6003
        assert_eq!(cpu.registers.pc, 0x6003, "RET should return to 0x6003");
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should be incremented by 2 to 0xFFFC");
        
        // Now RET from 0x6003 (outer function)
        cpu.memory_bus.write_byte(0x6003, 0xC9);
        
        cpu.registers.increment_pc();
        cpu.execute(0xC9);
        
        // Should return to 0x5003
        assert_eq!(cpu.registers.pc, 0x5003, "Second RET should return to 0x5003");
        assert_eq!(cpu.registers.sp, 0xFFFE, "SP should be incremented by 2 to 0xFFFE");
    }

    #[test]
    fn test_ret_with_various_addresses() {
        let mut cpu = Cpu::new();
        
        // Test RET with various return addresses
        let test_cases = vec![
            (0x0000, 0xFFFE),  // Return to 0x0000
            (0x1234, 0xFFFE),  // Return to 0x1234
            (0x5678, 0xFFFE),  // Return to 0x5678
            (0xFFFF, 0xFFFE),  // Return to 0xFFFF (max address)
        ];
        
        for (return_address, sp_before) in test_cases {
            cpu.registers.sp = sp_before;
            
            // Set up the return address on the stack (little-endian)
            let low_byte = (return_address & 0xFF) as u8;
            let high_byte = ((return_address >> 8) & 0xFF) as u8;
            
            cpu.memory_bus.write_byte(sp_before, low_byte);
            cpu.memory_bus.write_byte(sp_before + 1, high_byte);
            
            // Set PC to some address before RET
            cpu.registers.pc = 0x2000;
            
            // Set up RET opcode (0xC9)
            cpu.memory_bus.write_byte(0x2000, 0xC9);
            
            // Increment PC as tick() would do
            cpu.registers.increment_pc();
            
            // Execute RET
            cpu.execute(0xC9);
            
            // Verify PC was set to the return address
            assert_eq!(
                cpu.registers.pc, return_address,
                "PC should return to 0x{:04X}", return_address
            );
            
            // Verify SP was incremented by 2
            let expected_sp = sp_before.wrapping_add(2);
            assert_eq!(
                cpu.registers.sp, expected_sp,
                "SP should be incremented by 2"
            );
        }
    }

    #[test]
    fn test_ret_from_low_memory() {
        let mut cpu = Cpu::new();
        
        // Test RET when stack is in low memory
        cpu.registers.pc = 0x3000;
        cpu.registers.sp = 0x0100;
        
        // Set up return address 0x2500 on stack
        cpu.memory_bus.write_byte(0x0100, 0x00); // Low byte
        cpu.memory_bus.write_byte(0x0101, 0x25); // High byte
        
        // Set up RET opcode
        cpu.memory_bus.write_byte(0x3000, 0xC9);
        
        cpu.registers.increment_pc();
        cpu.execute(0xC9);
        
        assert_eq!(cpu.registers.pc, 0x2500, "PC should return to 0x2500");
        assert_eq!(cpu.registers.sp, 0x0102, "SP should be incremented by 2");
    }

    #[test]
    fn test_ret_high_memory() {
        let mut cpu = Cpu::new();
        
        // Test RET when stack is in high memory
        cpu.registers.pc = 0x9000;
        cpu.registers.sp = 0xFFFE;
        
        // Set up return address 0xABCD on stack
        cpu.memory_bus.write_byte(0xFFFE, 0xCD); // Low byte
        cpu.memory_bus.write_byte(0xFFFF, 0xAB); // High byte
        
        // Set up RET opcode
        cpu.memory_bus.write_byte(0x9000, 0xC9);
        
        cpu.registers.increment_pc();
        cpu.execute(0xC9);
        
        assert_eq!(cpu.registers.pc, 0xABCD, "PC should return to 0xABCD");
        // SP wraps around to 0x0000
        assert_eq!(cpu.registers.sp, 0x0000, "SP should wrap to 0x0000");
    }

    #[test]
    fn test_ret_cc_z_when_z_flag_set() {
        let mut cpu = Cpu::new();
        
        // Set Z flag to true (condition Z is true)
        cpu.flags_register.z = true;
        
        // Set up the scenario from the provided data:
        // CALL from 8000H pushes return address 8003H to stack
        // RET Z at 9000H should return to 8003H when Z = 1
        
        let ret_address = 0x9000;
        cpu.registers.pc = ret_address;
        cpu.registers.sp = 0xFFFC; // Stack already has return address from CALL
        
        // Set up return address on stack (0x8003, little-endian)
        cpu.memory_bus.write_byte(0xFFFC, 0x03); // Low byte
        cpu.memory_bus.write_byte(0xFFFD, 0x80); // High byte
        
        // Set up RET Z opcode (0xC8) at 0x9000
        cpu.memory_bus.write_byte(ret_address, 0xC8);
        
        // Increment PC as tick() would do
        cpu.registers.increment_pc();
        
        // Execute RET Z via execute function
        cpu.execute(0xC8);
        
        // Since Z flag is true, Z condition is true, so RET should execute
        // PC should return to 0x8003
        assert_eq!(cpu.registers.pc, 0x8003, "PC should return to 0x8003 when Z condition is true");
        
        // SP should be incremented by 2
        assert_eq!(cpu.registers.sp, 0xFFFE, "SP should be incremented by 2 to 0xFFFE");
    }

    #[test]
    fn test_ret_cc_z_when_z_flag_clear() {
        let mut cpu = Cpu::new();
        
        // Set Z flag to false (condition Z is false)
        cpu.flags_register.z = false;
        
        let ret_address = 0x9000;
        cpu.registers.pc = ret_address;
        cpu.registers.sp = 0xFFFC;
        
        // Set up return address on stack (doesn't matter since we won't return)
        cpu.memory_bus.write_byte(0xFFFC, 0x03);
        cpu.memory_bus.write_byte(0xFFFD, 0x80);
        
        // Set up RET Z opcode (0xC8)
        cpu.memory_bus.write_byte(ret_address, 0xC8);
        
        cpu.registers.increment_pc();
        
        let initial_sp = cpu.registers.sp;
        let initial_cycles = cpu.cycles;
        
        // Execute RET Z via execute function
        cpu.execute(0xC8);
        
        // Since Z flag is false, Z condition is false, so RET should NOT execute
        // PC should move to next instruction (9001H)
        assert_eq!(cpu.registers.pc, 0x9001, "PC should move to next instruction when Z condition is false");
        
        // SP should not change
        assert_eq!(cpu.registers.sp, initial_sp, "SP should not change when condition is false");
    }

    #[test]
    fn test_ret_cc_nz_when_z_flag_clear() {
        let mut cpu = Cpu::new();
        
        // Set Z flag to false (condition NZ is true)
        cpu.flags_register.z = false;
        
        let ret_address = 0x8000;
        cpu.registers.pc = ret_address;
        cpu.registers.sp = 0xFFFC;
        
        // Set up return address on stack (0x1234, little-endian)
        cpu.memory_bus.write_byte(0xFFFC, 0x34);
        cpu.memory_bus.write_byte(0xFFFD, 0x12);
        
        // Set up RET NZ opcode (0xC0) at 0x8000
        cpu.memory_bus.write_byte(ret_address, 0xC0);
        
        cpu.registers.increment_pc();
        
        // Execute RET NZ via execute function
        cpu.execute(0xC0);
        
        // Since Z flag is false, NZ condition is true, so RET should execute
        // PC should return to 0x1234
        assert_eq!(cpu.registers.pc, 0x1234, "PC should return to 0x1234 when NZ condition is true");
        
        // SP should be incremented by 2
        assert_eq!(cpu.registers.sp, 0xFFFE, "SP should be incremented by 2");
    }

    #[test]
    fn test_ret_cc_nz_when_z_flag_set() {
        let mut cpu = Cpu::new();
        
        // Set Z flag to true (condition NZ is false)
        cpu.flags_register.z = true;
        
        let ret_address = 0x8000;
        cpu.registers.pc = ret_address;
        cpu.registers.sp = 0xFFFC;
        
        // Set up return address on stack
        cpu.memory_bus.write_byte(0xFFFC, 0x34);
        cpu.memory_bus.write_byte(0xFFFD, 0x12);
        
        // Set up RET NZ opcode (0xC0)
        cpu.memory_bus.write_byte(ret_address, 0xC0);
        
        cpu.registers.increment_pc();
        
        // Execute RET NZ via execute function
        cpu.execute(0xC0);
        
        // Since Z flag is true, NZ condition is false, so RET should NOT execute
        // PC should move to next instruction (8001H)
        assert_eq!(cpu.registers.pc, 0x8001, "PC should move to next instruction when NZ condition is false");
        
        // SP should not change
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should not change when condition is false");
    }

    #[test]
    fn test_ret_cc_c_when_c_flag_set() {
        let mut cpu = Cpu::new();
        
        // Set C flag to true (condition C is true)
        cpu.flags_register.c = true;
        
        let ret_address = 0x7000;
        cpu.registers.pc = ret_address;
        cpu.registers.sp = 0xFFFC;
        
        // Set up return address on stack (0x5000, little-endian)
        cpu.memory_bus.write_byte(0xFFFC, 0x00);
        cpu.memory_bus.write_byte(0xFFFD, 0x50);
        
        // Set up RET C opcode (0xD8) at 0x7000
        cpu.memory_bus.write_byte(ret_address, 0xD8);
        
        cpu.registers.increment_pc();
        
        // Execute RET C via execute function
        cpu.execute(0xD8);
        
        // Since C flag is true, C condition is true, so RET should execute
        assert_eq!(cpu.registers.pc, 0x5000, "PC should return to 0x5000 when C condition is true");
        assert_eq!(cpu.registers.sp, 0xFFFE, "SP should be incremented by 2");
    }

    #[test]
    fn test_ret_cc_c_when_c_flag_clear() {
        let mut cpu = Cpu::new();
        
        // Set C flag to false (condition C is false)
        cpu.flags_register.c = false;
        
        let ret_address = 0x7000;
        cpu.registers.pc = ret_address;
        cpu.registers.sp = 0xFFFC;
        
        // Set up return address on stack
        cpu.memory_bus.write_byte(0xFFFC, 0x00);
        cpu.memory_bus.write_byte(0xFFFD, 0x50);
        
        // Set up RET C opcode (0xD8)
        cpu.memory_bus.write_byte(ret_address, 0xD8);
        
        cpu.registers.increment_pc();
        
        // Execute RET C via execute function
        cpu.execute(0xD8);
        
        // Since C flag is false, C condition is false, so RET should NOT execute
        assert_eq!(cpu.registers.pc, 0x7001, "PC should move to next instruction when C condition is false");
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should not change when condition is false");
    }

    #[test]
    fn test_ret_cc_nc_when_c_flag_clear() {
        let mut cpu = Cpu::new();
        
        // Set C flag to false (condition NC is true)
        cpu.flags_register.c = false;
        
        let ret_address = 0x6000;
        cpu.registers.pc = ret_address;
        cpu.registers.sp = 0xFFFC;
        
        // Set up return address on stack (0x3000, little-endian)
        cpu.memory_bus.write_byte(0xFFFC, 0x00);
        cpu.memory_bus.write_byte(0xFFFD, 0x30);
        
        // Set up RET NC opcode (0xD0) at 0x6000
        cpu.memory_bus.write_byte(ret_address, 0xD0);
        
        cpu.registers.increment_pc();
        
        // Execute RET NC via execute function
        cpu.execute(0xD0);
        
        // Since C flag is false, NC condition is true, so RET should execute
        assert_eq!(cpu.registers.pc, 0x3000, "PC should return to 0x3000 when NC condition is true");
        assert_eq!(cpu.registers.sp, 0xFFFE, "SP should be incremented by 2");
    }

    #[test]
    fn test_ret_cc_nc_when_c_flag_set() {
        let mut cpu = Cpu::new();
        
        // Set C flag to true (condition NC is false)
        cpu.flags_register.c = true;
        
        let ret_address = 0x6000;
        cpu.registers.pc = ret_address;
        cpu.registers.sp = 0xFFFC;
        
        // Set up return address on stack
        cpu.memory_bus.write_byte(0xFFFC, 0x00);
        cpu.memory_bus.write_byte(0xFFFD, 0x30);
        
        // Set up RET NC opcode (0xD0)
        cpu.memory_bus.write_byte(ret_address, 0xD0);
        
        cpu.registers.increment_pc();
        
        // Execute RET NC via execute function
        cpu.execute(0xD0);
        
        // Since C flag is true, NC condition is false, so RET should NOT execute
        assert_eq!(cpu.registers.pc, 0x6001, "PC should move to next instruction when NC condition is false");
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should not change when condition is false");
    }

    #[test]
    fn test_ret_cc_with_nested_calls() {
        let mut cpu = Cpu::new();
        
        // Scenario: Two nested calls with conditional returns
        // First CALL from 0x4000 to 0x5000
        cpu.registers.pc = 0x4000;
        cpu.registers.sp = 0xFFFE;
        
        cpu.memory_bus.write_byte(0x4000, 0xCD);
        cpu.memory_bus.write_byte(0x4001, 0x00);
        cpu.memory_bus.write_byte(0x4002, 0x50);
        
        cpu.registers.increment_pc();
        cpu.execute(0xCD);
        
        assert_eq!(cpu.registers.pc, 0x5000, "First CALL to 0x5000");
        
        // Second CALL from 0x5000 to 0x6000
        cpu.memory_bus.write_byte(0x5000, 0xCD);
        cpu.memory_bus.write_byte(0x5001, 0x00);
        cpu.memory_bus.write_byte(0x5002, 0x60);
        
        cpu.registers.increment_pc();
        cpu.execute(0xCD);
        
        assert_eq!(cpu.registers.pc, 0x6000, "Second CALL to 0x6000");
        assert_eq!(cpu.registers.sp, 0xFFFA, "SP after two CALLs");
        
        // RET Z from 0x6000 with Z flag true (should return)
        cpu.flags_register.z = true;
        cpu.memory_bus.write_byte(0x6000, 0xC8);
        
        cpu.registers.increment_pc();
        cpu.execute(0xC8);
        
        assert_eq!(cpu.registers.pc, 0x5003, "RET Z should return to 0x5003");
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should be restored");
        
        // RET Z from 0x5003 with Z flag false (should skip)
        cpu.flags_register.z = false;
        cpu.memory_bus.write_byte(0x5003, 0xC8);
        
        cpu.registers.increment_pc();
        cpu.execute(0xC8);
        
        assert_eq!(cpu.registers.pc, 0x5004, "RET Z should skip to 0x5004 when Z flag is false");
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should not change");
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

    #[test]
    fn test_rst_basic() {
        let mut cpu = Cpu::new();
        
        // Test RST 1 with data from provided scenario
        let initial_pc = 0x8001; // Address after RST instruction at 0x8000 (after fetch and increment)
        cpu.registers.pc = initial_pc;
        cpu.registers.sp = 0xFFFF;
        
        // Set up RST 1 opcode (0xCF) at 0x8000
        cpu.memory_bus.write_byte(0x8000, 0xCF);
        
        // Execute RST 1 via execute function (PC is already at 0x8001)
        cpu.execute(0xCF);
        
        // Verify PC was set to 0x0008
        assert_eq!(cpu.registers.pc, 0x0008, "PC should be set to 0x0008 after RST 1");
        
        // Verify SP was decremented by 2
        assert_eq!(cpu.registers.sp, 0xFFFD, "SP should be decremented by 2 to 0xFFFD");
        
        // Verify that 0x8001 was pushed onto the stack (return address)
        let pushed_value = ((cpu.memory_bus.read_byte(0xFFFE) as u16) << 8)
                          | (cpu.memory_bus.read_byte(0xFFFD) as u16);
        assert_eq!(pushed_value, 0x8001, "Return address 0x8001 should be pushed onto stack");
    }

    #[test]
    fn test_rst_all_vectors() {
        let mut cpu = Cpu::new();
        
        // Test all RST vectors (0x0000, 0x0008, 0x0010, 0x0018, 0x0020, 0x0028, 0x0030, 0x0038)
        let test_cases = vec![
            (0xC7, 0x0000, "RST 0"),  // 0xC7 = RST 0
            (0xCF, 0x0008, "RST 1"),  // 0xCF = RST 1
            (0xD7, 0x0010, "RST 2"),  // 0xD7 = RST 2
            (0xDF, 0x0018, "RST 3"),  // 0xDF = RST 3
            (0xE7, 0x0020, "RST 4"),  // 0xE7 = RST 4
            (0xEF, 0x0028, "RST 5"),  // 0xEF = RST 5
            (0xF7, 0x0030, "RST 6"),  // 0xF7 = RST 6
            (0xFF, 0x0038, "RST 7"),  // 0xFF = RST 7
        ];
        
        for (opcode, target_address, rst_name) in test_cases {
            cpu.registers.pc = 0x5001; // PC after fetch and increment (instruction at 0x5000)
            cpu.registers.sp = 0xFFFF;
            
            // Set up RST opcode
            cpu.memory_bus.write_byte(0x5000, opcode);
            
            cpu.execute(opcode);
            
            assert_eq!(
                cpu.registers.pc, target_address,
                "{} should jump to 0x{:04X}", rst_name, target_address
            );
            
            assert_eq!(cpu.registers.sp, 0xFFFD, "{} should decrement SP by 2", rst_name);
            
            // Verify return address was pushed
            let pushed_value = ((cpu.memory_bus.read_byte(0xFFFE) as u16) << 8)
                              | (cpu.memory_bus.read_byte(0xFFFD) as u16);
            assert_eq!(
                pushed_value, 0x5001,
                "{} should push return address 0x5001", rst_name
            );
        }
    }

    #[test]
    fn test_rst_with_different_sp() {
        let mut cpu = Cpu::new();
        
        // Test RST with different stack pointer values
        let test_cases = vec![
            (0xFFFE, 0xFFFC),  // Normal case
            (0x0100, 0x00FE),  // Low memory
            (0x8000, 0x7FFE),  // Mid-range
        ];
        
        for (initial_sp, expected_sp) in test_cases {
            cpu.registers.pc = 0x4001; // PC after fetch and increment (instruction at 0x4000)
            cpu.registers.sp = initial_sp;
            
            cpu.memory_bus.write_byte(0x4000, 0xCF);
            
            cpu.execute(0xCF);
            
            assert_eq!(
                cpu.registers.sp, expected_sp,
                "SP should be decremented from 0x{:04X} to 0x{:04X}",
                initial_sp, expected_sp
            );
            
            let pushed_value = ((cpu.memory_bus.read_byte(expected_sp + 1) as u16) << 8)
                              | (cpu.memory_bus.read_byte(expected_sp) as u16);
            assert_eq!(pushed_value, 0x4001, "Return address should be 0x4001");
        }
    }

    #[test]
    fn test_rst_nested_calls() {
        let mut cpu = Cpu::new();
        
        // First RST from 0x3000 (PC at 0x3001 after fetch and increment)
        cpu.registers.pc = 0x3001;
        cpu.registers.sp = 0xFFFF;
        
        cpu.memory_bus.write_byte(0x3000, 0xCF); // RST 1
        
        cpu.execute(0xCF);
        
        assert_eq!(cpu.registers.pc, 0x0008, "First RST 1 should jump to 0x0008");
        assert_eq!(cpu.registers.sp, 0xFFFD, "SP after first RST");
        
        let first_return = ((cpu.memory_bus.read_byte(0xFFFE) as u16) << 8)
                          | (cpu.memory_bus.read_byte(0xFFFD) as u16);
        assert_eq!(first_return, 0x3001, "First return address should be 0x3001");
        
        // Second RST from 0x0008 (PC at 0x0009 after fetch and increment)
        cpu.registers.pc = 0x0009;
        cpu.memory_bus.write_byte(0x0008, 0xD7); // RST 2
        
        cpu.execute(0xD7);
        
        assert_eq!(cpu.registers.pc, 0x0010, "Second RST 2 should jump to 0x0010");
        assert_eq!(cpu.registers.sp, 0xFFFB, "SP after second RST");
        
        let second_return = ((cpu.memory_bus.read_byte(0xFFFC) as u16) << 8)
                           | (cpu.memory_bus.read_byte(0xFFFB) as u16);
        assert_eq!(second_return, 0x0009, "Second return address should be 0x0009");
    }

    #[test]
    fn test_rst_preserves_return_address() {
        let mut cpu = Cpu::new();
        
        // Test that RST correctly preserves the return address (PC at the time of execution)
        let initial_pc_values = vec![0x1000, 0x2000, 0x5555, 0x7FFF];
        
        for initial_pc in initial_pc_values {
            cpu.registers.pc = initial_pc;
            cpu.registers.sp = 0xFFFF;
            
            cpu.memory_bus.write_byte(initial_pc - 1, 0xCF); // RST 1 at previous address
            
            let expected_return_address = initial_pc; // PC is at the next address after fetch+increment
            
            cpu.execute(0xCF);
            
            let pushed_value = ((cpu.memory_bus.read_byte(0xFFFE) as u16) << 8)
                              | (cpu.memory_bus.read_byte(0xFFFD) as u16);
            
            assert_eq!(
                pushed_value, expected_return_address,
                "RST should push return address 0x{:04X} when executed from PC 0x{:04X}",
                expected_return_address, initial_pc
            );
        }
    }

    #[test]
    fn test_rst_from_different_locations() {
        let mut cpu = Cpu::new();
        
        // Test RST from different memory locations to verify vector calculation
        let test_scenarios = vec![
            (0x8001, 0xCF, 0x8001, 0x0008), // RST 1 from PC 0x8001 (instruction at 0x8000)
            (0x4001, 0xC7, 0x4001, 0x0000), // RST 0 from PC 0x4001 (instruction at 0x4000)
            (0x2001, 0xD7, 0x2001, 0x0010), // RST 2 from PC 0x2001 (instruction at 0x2000)
            (0x6001, 0xE7, 0x6001, 0x0020), // RST 4 from PC 0x6001 (instruction at 0x6000)
        ];
        
        for (initial_pc, opcode, expected_return, expected_target) in test_scenarios {
            cpu.registers.pc = initial_pc;
            cpu.registers.sp = 0xFFFF;
            
            cpu.memory_bus.write_byte(initial_pc - 1, opcode);
            
            cpu.execute(opcode);
            
            assert_eq!(
                cpu.registers.pc, expected_target,
                "RST from PC 0x{:04X} (opcode 0x{:02X}) should jump to 0x{:04X}",
                initial_pc, opcode, expected_target
            );
            
            let pushed_value = ((cpu.memory_bus.read_byte(0xFFFE) as u16) << 8)
                              | (cpu.memory_bus.read_byte(0xFFFD) as u16);
            assert_eq!(
                pushed_value, expected_return,
                "Should push return address 0x{:04X}", expected_return
            );
        }
    }
}

