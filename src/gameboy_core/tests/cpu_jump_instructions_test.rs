#[cfg(test)]
mod tests {
    use crate::gameboy_core::cpu::Cpu;

    // Test for jp_imm16 - JP nn instruction (0xC3)
    // Loads the 16-bit immediate value to the program counter (PC).

    #[test]
    fn test_jp_imm16_loads_address_to_pc() {
        let mut cpu = Cpu::new();
        
        let initial_pc = cpu.registers.pc;
        
        // Set up memory with JP nn opcode (0xC3) at current PC
        cpu.memory_bus.write_byte(initial_pc, 0xC3); // JP nn opcode
        // Set up the 16-bit immediate value (little-endian: 0x1234)
        // At PC + 1: low byte (0x34)
        // At PC + 2: high byte (0x12)
        cpu.memory_bus.write_byte(initial_pc + 1, 0x34); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0x12); // High byte
        
        // Increment PC to point to the opcode as the tick function would do
        cpu.registers.increment_pc();
        
        // Execute JP nn via execute function
        cpu.execute(0xC3);
        
        // PC should now be 0x1234
        assert_eq!(cpu.registers.pc, 0x1234, "PC should be set to 0x1234");
    }

    #[test]
    fn test_jp_imm16_with_high_address() {
        let mut cpu = Cpu::new();
        
        let initial_pc = cpu.registers.pc;
        
        // Set up memory with JP nn opcode and immediate value for 0xFFFF
        cpu.memory_bus.write_byte(initial_pc, 0xC3); // JP nn opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0xFF); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0xFF); // High byte
        
        cpu.registers.increment_pc();
        cpu.execute(0xC3);
        
        
        assert_eq!(cpu.registers.pc, 0xFFFF, "PC should be set to 0xFFFF");
    }

    #[test]
    fn test_jp_imm16_with_zero_address() {
        let mut cpu = Cpu::new();
        
        let initial_pc = cpu.registers.pc;
        
        // Set up memory with JP nn opcode and immediate value for 0x0000
        cpu.memory_bus.write_byte(initial_pc, 0xC3); // JP nn opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0x00); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0x00); // High byte
        
        cpu.registers.increment_pc();
        cpu.execute(0xC3);
        
        assert_eq!(cpu.registers.pc, 0x0000, "PC should be set to 0x0000");
    }

    // Test for jp_cc_imm16 - JP cc, nn instruction
    // Loads operand nn in the PC if condition cc and the flag status match.
    // Note: Conditional jump instructions are not yet wired in the execute function,
    // so these tests call the method directly.

    #[test]
    fn test_jp_cc_imm16_when_condition_true_nz() {
        let mut cpu = Cpu::new();
        
        // NZ (Non-Zero) condition = bit 3-4 of opcode = 0b00
        // Z flag should be false for NZ to be true
        cpu.registers.flags.z = false;
        
        let initial_pc = cpu.registers.pc;
        
        // Set up memory with JP NZ nn opcode (0xC2) at current PC
        cpu.memory_bus.write_byte(initial_pc, 0xC2); // JP NZ, nn opcode
        // Set up the 16-bit immediate value (little-endian: 0x2000)
        // At PC + 1: low byte (0x00)
        // At PC + 2: high byte (0x20)
        cpu.memory_bus.write_byte(initial_pc + 1, 0x00); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0x20); // High byte
        
        // Increment PC to point to the opcode as the tick function would do
        cpu.registers.increment_pc();
        
        // Execute JP NZ via execute function
        cpu.execute(0xC2);
        
        // Since NZ condition is true (z = false), PC should jump to 0x2000
        assert_eq!(cpu.registers.pc, 0x2000, "PC should jump to 0x2000 when NZ condition is true");
    }

    #[test]
    fn test_jp_cc_imm16_when_condition_false_nz() {
        let mut cpu = Cpu::new();
        
        // NZ (Non-Zero) condition, but Z flag is true (condition fails)
        cpu.registers.flags.z = true;
        
        let initial_pc = cpu.registers.pc;
        
        // Set up memory with JP NZ nn opcode (0xC2) at current PC
        cpu.memory_bus.write_byte(initial_pc, 0xC2); // JP NZ, nn opcode
        // Set up the 16-bit immediate value (little-endian: 0x2000)
        cpu.memory_bus.write_byte(initial_pc + 1, 0x00); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0x20); // High byte
        
        // Increment PC to point to the opcode as the tick function would do
        cpu.registers.increment_pc();
        
        // Execute JP NZ via execute function
        cpu.execute(0xC2);
        
        // Since NZ condition is false (z = true), PC should increment by 2 (past the immediate values)
        // After execute, PC should be at initial_pc + 1 (from increment_pc before execute) + 2 (from jp_cc_imm16 when condition is false)
        assert_eq!(
            cpu.registers.pc,
            initial_pc + 3,
            "PC should increment by 2 when NZ condition is false"
        );
    }

    #[test]
    fn test_jp_cc_imm16_when_condition_true_z() {
        let mut cpu = Cpu::new();
        
        // Z (Zero) condition = bit 3-4 of opcode = 0b01
        // Z flag should be true for Z condition to be true
        cpu.registers.flags.z = true;
        
        let initial_pc = cpu.registers.pc;
        
        // Set up memory with JP Z nn opcode (0xCA) at current PC
        cpu.memory_bus.write_byte(initial_pc, 0xCA); // JP Z, nn opcode
        // Set up the 16-bit immediate value (little-endian: 0x3000)
        cpu.memory_bus.write_byte(initial_pc + 1, 0x00); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0x30); // High byte
        
        // Increment PC to point to the opcode as the tick function would do
        cpu.registers.increment_pc();
        
        // Execute JP Z via execute function
        cpu.execute(0xCA);
        
        assert_eq!(cpu.registers.pc, 0x3000, "PC should jump to 0x3000 when Z condition is true");
    }

    #[test]
    fn test_jp_cc_imm16_when_condition_true_nc() {
        let mut cpu = Cpu::new();
        
        // NC (No Carry) condition = bit 3-4 of opcode = 0b10
        // C flag should be false for NC condition to be true
        cpu.registers.flags.c = false;
        
        let initial_pc = cpu.registers.pc;
        
        // Set up memory with JP NC nn opcode (0xD2) at current PC
        cpu.memory_bus.write_byte(initial_pc, 0xD2); // JP NC, nn opcode
        // Set up the 16-bit immediate value (little-endian: 0x4000)
        cpu.memory_bus.write_byte(initial_pc + 1, 0x00); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0x40); // High byte
        
        // Increment PC to point to the opcode as the tick function would do
        cpu.registers.increment_pc();
        
        // Execute JP NC via execute function
        cpu.execute(0xD2);
        
        assert_eq!(cpu.registers.pc, 0x4000, "PC should jump to 0x4000 when NC condition is true");
    }

    #[test]
    fn test_jp_cc_imm16_when_condition_true_c() {
        let mut cpu = Cpu::new();
        
        // C (Carry) condition = bit 3-4 of opcode = 0b11
        // C flag should be true for C condition to be true
        cpu.registers.flags.c = true;
        
        let initial_pc = cpu.registers.pc;
        
        // Set up memory with JP C nn opcode (0xDA) at current PC
        cpu.memory_bus.write_byte(initial_pc, 0xDA); // JP C, nn opcode
        // Set up the 16-bit immediate value (little-endian: 0x5000)
        cpu.memory_bus.write_byte(initial_pc + 1, 0x00); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0x50); // High byte
        
        // Increment PC to point to the opcode as the tick function would do
        cpu.registers.increment_pc();
        
        // Execute JP C via execute function
        cpu.execute(0xDA);
        
        assert_eq!(cpu.registers.pc, 0x5000, "PC should jump to 0x5000 when C condition is true");
    }

    // Test for jr_imm8 - JR n instruction (0x18)
    // Jumps to the address by adding the signed 8-bit immediate value to the PC.

    #[test]
    fn test_jr_imm8_positive_offset() {
        let mut cpu = Cpu::new();
        
        let initial_pc = 0x200;
        cpu.registers.pc = initial_pc;
        
        // Set up memory with JR n opcode (0x18) and a positive offset: +50 (0x32)
        cpu.memory_bus.write_byte(initial_pc, 0x18); // JR n opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0x32); // Offset: +50
        
        // Increment PC to point to the opcode as the tick function would do
        cpu.registers.increment_pc();
        // Now PC = 0x201
        
        let initial_cycles = cpu.cycles;
        cpu.execute(0x18);
        
        // PC flow: tick increments to 0x201, then jr_imm8 increments again (0x202) and adds offset (+50)
        // Result: 0x202 + 50 = 0x234
        assert_eq!(cpu.registers.pc, 0x234, "PC should be 0x234 after adding positive offset");
        
        // Cycles should be incremented by 12
        assert_eq!(
            cpu.cycles,
            initial_cycles + 3,
            "Cycles should be incremented by 12"
        );
    }

    #[test]
    fn test_jr_imm8_negative_offset() {
        let mut cpu = Cpu::new();
        
        let initial_pc = 0x300;
        cpu.registers.pc = initial_pc;
        
        // Set up memory with JR n opcode and a negative offset: -10 (0xF6 as u8, -10 as i8)
        cpu.memory_bus.write_byte(initial_pc, 0x18); // JR n opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0xF6); // Offset: -10
        
        cpu.registers.increment_pc();
        // Now PC = 0x301
        
        let initial_cycles = cpu.cycles;
        cpu.execute(0x18);
        
        // PC flow: tick incremented to 0x301, then jr_imm8 increments again (0x302) and subtracts 10
        // Result: 0x302 - 10 = 0x2F8
        assert_eq!(cpu.registers.pc, 0x2F8, "PC should be 0x2F8 after subtracting with negative offset");
        
        // Cycles should be incremented by 12
        assert_eq!(
            cpu.cycles,
            initial_cycles + 12,
            "Cycles should be incremented by 12"
        );
    }

    #[test]
    fn test_jr_imm8_zero_offset() {
        let mut cpu = Cpu::new();
        
        let initial_pc = 0x100;
        cpu.registers.pc = initial_pc;
        
        // Set up memory with JR n opcode and zero offset
        cpu.memory_bus.write_byte(initial_pc, 0x18); // JR n opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0x00); // Offset: 0
        
        cpu.registers.increment_pc();
        // Now PC = 0x101
        
        let initial_cycles = cpu.cycles;
        cpu.execute(0x18);
        
        // PC flow: tick incremented to 0x101, then jr_imm8 increments again (0x102) and adds 0
        // Result: 0x102 + 0 = 0x102
        assert_eq!(
            cpu.registers.pc, 0x102,
            "PC should be 0x102 with zero offset"
        );
        
        assert_eq!(
            cpu.cycles,
            initial_cycles + 12,
            "Cycles should be incremented by 12"
        );
    }

    #[test]
    fn test_jr_imm8_max_positive_offset() {
        let mut cpu = Cpu::new();
        
        let initial_pc = 0x200;
        cpu.registers.pc = initial_pc;
        
        // Set up memory with JR n opcode and max positive offset: +127 (0x7F)
        cpu.memory_bus.write_byte(initial_pc, 0x18); // JR n opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0x7F); // Offset: +127
        
        cpu.registers.increment_pc();
        // Now PC = 0x201
        
        cpu.execute(0x18);
        
        // PC flow: tick incremented to 0x201, then jr_imm8 increments (0x202) and adds 127
        // Result: 0x202 + 127 = 0x281
        assert_eq!(
            cpu.registers.pc, 0x281,
            "PC should be 0x281 after max positive offset"
        );
    }

    #[test]
    fn test_jr_imm8_max_negative_offset() {
        let mut cpu = Cpu::new();
        
        let initial_pc = 0x200;
        cpu.registers.pc = initial_pc;
        
        // Set up memory with JR n opcode and max negative offset: -128 (0x80 as u8, -128 as i8)
        cpu.memory_bus.write_byte(initial_pc, 0x18); // JR n opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0x80); // Offset: -128
        
        cpu.registers.increment_pc();
        // Now PC = 0x201
        
        cpu.execute(0x18);
        
        // PC flow: tick incremented to 0x201, then jr_imm8 increments (0x202) and subtracts 128
        // Result: 0x202 - 128 = 0x182
        assert_eq!(
            cpu.registers.pc, 0x182,
            "PC should be 0x182 after max negative offset"
        );
    }

    #[test]
    fn test_jr_imm8_wrapping_underflow() {
        let mut cpu = Cpu::new();
        
        let initial_pc = 0x0005;
        cpu.registers.pc = initial_pc;
        
        // Set up memory with JR n opcode and offset -10 (0xF6)
        cpu.memory_bus.write_byte(initial_pc, 0x18); // JR n opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0xF6); // Offset: -10
        
        cpu.registers.increment_pc();
        // Now PC = 0x0006
        
        cpu.execute(0x18);
        
        // PC flow: tick incremented to 0x0006, then jr_imm8 increments (0x0007) and subtracts 10
        // Result: (0x0007 as i16) - 10 = 0xFFFD (wrapped)
        assert_eq!(
            cpu.registers.pc, 0xFFFD,
            "PC should wrap to 0xFFFD when underflowing"
        );
    }

    // Test for jr_cc_imm8 - JR cc, n instruction
    // If condition cc and the flag status match, jumps -127 to +129 steps from the current address.
    // Note: Conditional jump instructions are not yet wired in the execute function,
    // so these tests call the method directly.

    #[test]
    fn test_jr_cc_imm8_when_condition_false() {
        let mut cpu = Cpu::new();
        
        let initial_pc = 0x200;
        cpu.registers.pc = initial_pc;
        
        // Set Z flag true so NZ condition will fail
        cpu.registers.flags.z = true;
        
        // Set up memory with JR NZ offset (we'll use 0x20 for JR NZ opcode, 0x14 for offset +20)
        // But we need to set up opcode 0x20 (JR NZ, n) in memory at initial_pc
        // Actually, the opcode for JR NZ is 0x20
        cpu.memory_bus.write_byte(initial_pc, 0x20); // JR NZ, n opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0x14); // Offset: +20
        
        // Increment PC as tick() would do
        cpu.registers.increment_pc();
        // Now PC = 0x201
        
        let initial_cycles = cpu.cycles;
        
        // Execute JR NZ via execute function
        cpu.execute(0x20);
        
        // Since NZ condition is false (z = true), jr_cc_imm8 should not call jr_imm8
        // So PC should stay at 0x201 (after the increment_pc in execute)
        assert_eq!(
            cpu.registers.pc, 0x201,
            "PC should not change when condition is false"
        );
        
        // Cycles should not be incremented
        assert_eq!(cpu.cycles, initial_cycles, "Cycles should not change when condition is false");
    }

    #[test]
    fn test_jr_cc_imm8_when_condition_true() {
        let mut cpu = Cpu::new();
        
        let initial_pc = 0x200;
        cpu.registers.pc = initial_pc;
        
        // Set Z flag to true for Z condition
        cpu.registers.flags.z = true;
        
        // Set up memory with JR Z opcode (0x28) and offset +20 (0x14)
        cpu.memory_bus.write_byte(initial_pc, 0x28); // JR Z, n opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0x14); // Offset: +20
        
        // Increment PC as tick() would do
        cpu.registers.increment_pc();
        // Now PC = 0x201
        
        let initial_cycles = cpu.cycles;
        
        // Execute JR Z via execute function
        cpu.execute(0x28);
        
        // Since Z condition is true (z = true), should jump: 0x201 + 1 (increment in execute) + 20 = 0x216
        // Wait, we need to understand the flow:
        // 1. tick() increments PC (0x200 -> 0x201)
        // 2. execute() is called with opcode 0x28
        // 3. execute() calls jr_cc_imm8() which:
        //    - checks condition (true)
        //    - calls jr_imm8() which:
        //      - reads imm8 from PC (still 0x201, which points to 0x14)
        //      - increments PC (0x201 -> 0x202)
        //      - adds offset: (0x202 + 20) = 0x216
        // But jr_imm8 increments PC and then adds the offset, so result should be 0x216
        // However, we need to check the offset value and expected result
        // Offset +20 (0x14), current PC after tick increment = 0x201
        // jr_imm8 reads imm8, increments PC to 0x202, then adds 20: 0x202 + 20 = 0x216
        // But the test was expecting 0x215, let me recalculate
        // Initial PC = 0x200, after tick increment = 0x201
        // PC + 1 + 20 = 0x200 + 1 + 20 = 0x215
        // So the expected result should be 0x216 (0x202 + 0x14)
        assert_eq!(
            cpu.registers.pc, 0x216,
            "PC should jump when condition is true"
        );
        
        // Cycles should be incremented by 12 (from jr_imm8)
        assert_eq!(
            cpu.cycles,
            initial_cycles + 12,
            "Cycles should be incremented by 12 when condition is true"
        );
    }

    #[test]
    fn test_jr_cc_imm8_with_negative_offset() {
        let mut cpu = Cpu::new();
        
        let initial_pc = 0x300;
        cpu.registers.pc = initial_pc;
        
        // C flag true for C condition
        cpu.registers.flags.c = true;
        
        // Set up memory with JR C opcode (0x38) and offset -15 (0xF1)
        cpu.memory_bus.write_byte(initial_pc, 0x38); // JR C, n opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0xF1); // Offset: -15
        
        // Increment PC as tick() would do
        cpu.registers.increment_pc();
        // Now PC = 0x301
        
        // Execute JR C via execute function
        cpu.execute(0x38);
        
        // Since C condition is true, should jump: 
        // After execute's increment_pc call, PC = 0x302
        // jr_imm8 reads offset 0xF1 as -15, increments PC to 0x302, then: 0x302 - 15 = 0x2F3
        assert_eq!(
            cpu.registers.pc, 0x2F3,
            "PC should jump with negative offset when condition is true"
        );
    }

    // Test for jp_hl - JP HL instruction (0xE9)
    // Loads the contents of register pair HL in program counter PC.
    // Note: This instruction is not yet wired in the execute function,
    // so these tests call the method directly.

    #[test]
    fn test_jp_hl_loads_hl_to_pc() {
        let mut cpu = Cpu::new();
        
        // Set HL register to 0x3456
        cpu.registers.h = 0x34;
        cpu.registers.l = 0x56;
        
        let initial_pc = cpu.registers.pc;
        
        // Set up memory with JP HL opcode (0xE9) at current PC
        cpu.memory_bus.write_byte(initial_pc, 0xE9); // JP HL opcode
        
        // Increment PC as tick() would do
        cpu.registers.increment_pc();
        
        // Execute JP HL via execute function
        cpu.execute(0xE9);
        
        // PC should now be 0x3456
        assert_eq!(cpu.registers.pc, 0x3456, "PC should be set to HL value (0x3456)");
    }

    #[test]
    fn test_jp_hl_with_zero() {
        let mut cpu = Cpu::new();
        
        // Set HL register to 0x0000
        cpu.registers.h = 0x00;
        cpu.registers.l = 0x00;
        
        let initial_pc = cpu.registers.pc;
        
        // Set up memory with JP HL opcode (0xE9) at current PC
        cpu.memory_bus.write_byte(initial_pc, 0xE9); // JP HL opcode
        
        // Increment PC as tick() would do
        cpu.registers.increment_pc();
        
        // Execute JP HL via execute function
        cpu.execute(0xE9);
        
        assert_eq!(cpu.registers.pc, 0x0000, "PC should be set to 0x0000");
    }

    #[test]
    fn test_jp_hl_with_max_value() {
        let mut cpu = Cpu::new();
        
        // Set HL register to 0xFFFF
        cpu.registers.h = 0xFF;
        cpu.registers.l = 0xFF;
        
        let initial_pc = cpu.registers.pc;
        
        // Set up memory with JP HL opcode (0xE9) at current PC
        cpu.memory_bus.write_byte(initial_pc, 0xE9); // JP HL opcode
        
        // Increment PC as tick() would do
        cpu.registers.increment_pc();
        
        // Execute JP HL via execute function
        cpu.execute(0xE9);
        
        assert_eq!(cpu.registers.pc, 0xFFFF, "PC should be set to 0xFFFF");
    }

    #[test]
    fn test_jp_hl_with_various_values() {
        let mut cpu = Cpu::new();
        
        // Test with multiple HL values
        let test_cases = vec![
            (0x12, 0x34, 0x1234),
            (0xAB, 0xCD, 0xABCD),
            (0xFF, 0x00, 0xFF00),
            (0x00, 0xFF, 0x00FF),
            (0x80, 0x00, 0x8000),
        ];
        
        for (h, l, expected_pc) in test_cases {
            cpu.registers.h = h;
            cpu.registers.l = l;
            
            let initial_pc = cpu.registers.pc;
            
            // Set up memory with JP HL opcode (0xE9) at current PC
            cpu.memory_bus.write_byte(initial_pc, 0xE9); // JP HL opcode
            
            // Increment PC as tick() would do
            cpu.registers.increment_pc();
            
            // Execute JP HL via execute function
            cpu.execute(0xE9);
            
            assert_eq!(
                cpu.registers.pc, expected_pc,
                "PC should be set to 0x{:04X} for HL = 0x{:02X}{:02X}",
                expected_pc, h, l
            );
        }
    }

    // Integration tests combining multiple instructions

    #[test]
    fn test_multiple_jumps_sequence() {
        let mut cpu = Cpu::new();
        
        let initial_pc = cpu.registers.pc;
        
        // First jump: JP 0x2000 (opcode 0xC3)
        cpu.memory_bus.write_byte(initial_pc, 0xC3);
        cpu.memory_bus.write_byte(initial_pc + 1, 0x00);
        cpu.memory_bus.write_byte(initial_pc + 2, 0x20);
        cpu.registers.pc = initial_pc;
        cpu.registers.increment_pc();
        cpu.execute(0xC3);
        assert_eq!(cpu.registers.pc, 0x2000);
        
        // Second jump: JR +50 (opcode 0x18)
        let pc_before_jr = cpu.registers.pc;
        cpu.memory_bus.write_byte(pc_before_jr, 0x18);
        cpu.memory_bus.write_byte(pc_before_jr + 1, 0x32);
        cpu.registers.increment_pc();
        cpu.execute(0x18);
        assert_eq!(cpu.registers.pc, 0x2034);
        
        // Third jump: JP HL where HL = 0x5000
        cpu.registers.h = 0x50;
        cpu.registers.l = 0x00;
        let pc_before_jp_hl = cpu.registers.pc;
        cpu.memory_bus.write_byte(pc_before_jp_hl, 0xE9); // JP HL opcode
        cpu.registers.increment_pc();
        cpu.execute(0xE9);
        assert_eq!(cpu.registers.pc, 0x5000);
    }

    #[test]
    fn test_conditional_jumps_with_flags() {
        let mut cpu = Cpu::new();
        
        let initial_pc = 0x1000;
        cpu.registers.pc = initial_pc;
        
        // Test NZ jump when Z flag is clear
        cpu.registers.flags.z = false;
        cpu.memory_bus.write_byte(initial_pc, 0xC2); // JP NZ opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0x20); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0x10); // High byte
        cpu.registers.increment_pc();
        cpu.execute(0xC2); // JP NZ
        assert_eq!(cpu.registers.pc, 0x1020, "NZ jump should execute when Z is clear");
        
        // Reset and test Z jump when Z flag is set
        cpu.registers.pc = initial_pc;
        cpu.registers.flags.z = true;
        cpu.memory_bus.write_byte(initial_pc, 0xCA); // JP Z opcode
        cpu.memory_bus.write_byte(initial_pc + 1, 0x30); // Low byte
        cpu.memory_bus.write_byte(initial_pc + 2, 0x20); // High byte
        cpu.registers.increment_pc();
        cpu.execute(0xCA); // JP Z
        assert_eq!(cpu.registers.pc, 0x2030, "Z jump should execute when Z is set");
    }
}
