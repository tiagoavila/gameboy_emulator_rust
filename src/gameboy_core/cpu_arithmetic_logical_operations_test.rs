#[cfg(test)]
mod tests {
    use crate::gameboy_core::cpu::Cpu;

    fn setup_cpu_with_a(a: u8, cy: bool) -> Cpu {
        let mut cpu = Cpu::new();
        cpu.registers.a = a;
        cpu.flags_register.c_flag = cy;
        cpu
    }

    #[test]
    fn test_adc_a_r() {
        // ADC A, E ; A ← F1h, Z ← 0, H ← 1, CY ← 0
        let mut cpu = setup_cpu_with_a(0xE1, true);
        cpu.registers.e = 0x0F;

        let opcode = 0b10001011; // ADC A, E
        cpu.execute(opcode);
        assert_eq!(cpu.registers.a, 0xF1);
        assert_eq!(cpu.flags_register.z_flag, false);
        assert_eq!(cpu.flags_register.h_flag, true);
        assert_eq!(cpu.flags_register.n_flag, false);
        assert_eq!(cpu.flags_register.c_flag, false);
    }

    #[test]
    fn test_adc_a_imm8() {
        // ADC A, 3Bh ; A ← 1Dh, Z ← 0, H ← 0, CY ← 1
        let mut cpu = setup_cpu_with_a(0xE1, true);
        // Place immediate value 0x3B at PC
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x3B);

        let opcode = 0b11001110; // ADC A, imm8
        cpu.execute(opcode);
        assert_eq!(cpu.registers.a, 0x1D);
        assert_eq!(cpu.flags_register.z_flag, false);
        assert_eq!(cpu.flags_register.h_flag, false);
        assert_eq!(cpu.flags_register.n_flag, false);
        assert_eq!(cpu.flags_register.c_flag, true);
    }

    #[test]
    fn test_sub_a_r() {
        // SUB E ; A ← 00h, Z ← 1, H ← 0, N ← 1 CY ← 0
        let mut cpu = setup_cpu_with_a(0x3E, false);
        cpu.registers.e = 0x3E;

        let opcode = 0b10010011; // SUB E
        cpu.execute(opcode);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.flags_register.z_flag, true);
        assert_eq!(cpu.flags_register.h_flag, false);
        assert_eq!(cpu.flags_register.n_flag, true);
        assert_eq!(cpu.flags_register.c_flag, false);
    }

    #[test]
    fn test_sub_a_imm8() {
        // SUB 0Fh ; A ← 2Fh, Z ← 0, H ← 1, N ← 1 CY← 0
        let mut cpu = setup_cpu_with_a(0x3E, false);
        // Place immediate value 0x0F at PC
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x0F);

        let opcode = 0b11010110; // SUB n
        cpu.execute(opcode);
        assert_eq!(cpu.registers.a, 0x2F);
        assert_eq!(cpu.flags_register.z_flag, false);
        assert_eq!(cpu.flags_register.h_flag, true);
        assert_eq!(cpu.flags_register.n_flag, true);
        assert_eq!(cpu.flags_register.c_flag, false);
    }

    #[test]
    fn test_sub_a_hl() {
        // SUB (HL) ; A ← FEh, Z ← 0, H ← 0, N ← 1 CY ← 1
        let mut cpu = setup_cpu_with_a(0x3E, false);
        cpu.registers.set_hl(0x1234);
        cpu.memory_bus.write_byte(0x1234, 0x40);

        let opcode = 0b10010110; // SUB (HL)
        cpu.execute(opcode);
        assert_eq!(cpu.registers.a, 0xFE);
        assert_eq!(cpu.flags_register.z_flag, false);
        assert_eq!(cpu.flags_register.h_flag, false);
        assert_eq!(cpu.flags_register.n_flag, true);
        assert_eq!(cpu.flags_register.c_flag, true);
    }

    #[test]
    fn test_adc_a_hl() {
        // ADC A, (HL) ; A ← 00h, Z ← 1, H ← 1, CY ← 1
        let mut cpu = setup_cpu_with_a(0xE1, true);
        cpu.registers.set_hl(0x1234);
        cpu.memory_bus.write_byte(0x1234, 0x1E);

        let opcode = 0b10001110; // ADC A, (HL)
        cpu.execute(opcode);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.flags_register.z_flag, true);
        assert_eq!(cpu.flags_register.h_flag, true);
        assert_eq!(cpu.flags_register.n_flag, false);
        assert_eq!(cpu.flags_register.c_flag, true);
    }

    #[test]
    fn test_add_a_hl() {
        // ADD A, (HL) ; A ← 4Eh, Z ← 0, H ← 0, N ← 0, CY ← 0
        let mut cpu = setup_cpu_with_a(0x3C, false);
        cpu.registers.set_hl(0x1234);
        cpu.memory_bus.write_byte(0x1234, 0x12);

        let opcode = 0b10000110; // ADD A, (HL)
        cpu.execute(opcode);
        assert_eq!(cpu.registers.a, 0x4E);
        assert_eq!(cpu.flags_register.z_flag, false);
        assert_eq!(cpu.flags_register.h_flag, false);
        assert_eq!(cpu.flags_register.n_flag, false);
        assert_eq!(cpu.flags_register.c_flag, false);
    }

    #[test]
    fn test_add_a_n() {
        // ADD A, FFh ; A ← 3Bh, Z ← 0, H ← 1, N ← 0, CY ← 1
        let mut cpu = setup_cpu_with_a(0x3C, false);
        // Place immediate value 0xFF at PC
        cpu.memory_bus.write_byte(cpu.registers.pc, 0xFF);

        let opcode = 0b11000110; // ADD A, n
        cpu.execute(opcode);
        assert_eq!(cpu.registers.a, 0x3B);
        assert_eq!(cpu.flags_register.z_flag, false);
        assert_eq!(cpu.flags_register.h_flag, true);
        assert_eq!(cpu.flags_register.n_flag, false);
        assert_eq!(cpu.flags_register.c_flag, true);
    }

    #[test]
    fn test_add_a_r() {
        // ADD A, B ; A ← 0, Z ← 1, H ← 1, N ← 0, CY ← 1
        let mut cpu = setup_cpu_with_a(0x3A, false);
        cpu.registers.b = 0xC6;

        let opcode = 0b10000000; // ADD A, B
        cpu.execute(opcode);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.flags_register.z_flag, true);
        assert_eq!(cpu.flags_register.h_flag, true);
        assert_eq!(cpu.flags_register.n_flag, false);
        assert_eq!(cpu.flags_register.c_flag, true);
    }

    #[test]
    fn test_sbc_a_r() {
        // SBC A, H ; A ← 10h, Z ← 0, H ← 0, N ← 1 CY ← 0
        let mut cpu = setup_cpu_with_a(0x3B, true);
        cpu.registers.h = 0x2A;

        let opcode = 0b10011100; // SBC A, H
        cpu.execute(opcode);
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.flags_register.z_flag, false);
        assert_eq!(cpu.flags_register.h_flag, false);
        assert_eq!(cpu.flags_register.n_flag, true);
        assert_eq!(cpu.flags_register.c_flag, false);
    }

    #[test]
    fn test_sbc_a_imm8() {
        // SBC A, 3Ah ; A ← 00h, Z ← 1, H ← 0, N ← 1 CY ← 0
        let mut cpu = setup_cpu_with_a(0x3B, true);
        // Place immediate value 0x3A at PC
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x3A);
        let opcode = 0b11011110; // SBC A, imm8
        cpu.execute(opcode);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.flags_register.z_flag, true);
        assert_eq!(cpu.flags_register.h_flag, false);
        assert_eq!(cpu.flags_register.n_flag, true);
        assert_eq!(cpu.flags_register.c_flag, false);
    }

    #[test]
    fn test_sbc_a_hl() {
        // SBC A, (HL) ; A ← EBh, Z ← 0, H ← 1, N ← 1 CY ← 1
        let mut cpu = setup_cpu_with_a(0x3B, true);
        cpu.registers.set_hl(0x1234);
        cpu.memory_bus.write_byte(0x1234, 0x4F);
        let opcode = 0b10011110; // SBC A, (HL)
        cpu.execute(opcode);
        assert_eq!(cpu.registers.a, 0xEB);
        assert_eq!(cpu.flags_register.z_flag, false);
        assert_eq!(cpu.flags_register.h_flag, true);
        assert_eq!(cpu.flags_register.n_flag, true);
        assert_eq!(cpu.flags_register.c_flag, true);
    }

    #[test]
    fn test_push_r16_onto_memory_stack() {
        // PUSH BC ; with SP = FFFEh -> SP becomes FFFCh, memory[FFFCh]=low(C), memory[FFFCh+1]=high(B)
        let mut cpu = Cpu::new();

        // Set SP and BC
        cpu.registers.sp = 0xFFFE;
        cpu.registers.b = 0xAB; // high byte
        cpu.registers.c = 0xCD; // low byte

        let opcode = 0b11000101; // PUSH BC (0xC5)
        cpu.execute(opcode);

        // After push, SP should be decremented by 2
        assert_eq!(cpu.registers.sp, 0xFFFC, "SP should be decremented by 2 after PUSH");

        // Memory at SP (0xFFFC) should contain low byte (C)
        assert_eq!(cpu.memory_bus.read_byte(0xFFFC), 0xCD, "Memory at 0xFFFC should contain C (low byte)");

        // Memory at SP + 1 (0xFFFD) should contain high byte (B)
        assert_eq!(cpu.memory_bus.read_byte(0xFFFD), 0xAB, "Memory at 0xFFFD should contain B (high byte)");

        // Registers B and C should remain unchanged
        assert_eq!(cpu.registers.b, 0xAB);
        assert_eq!(cpu.registers.c, 0xCD);
    }

    #[test]
    fn test_pop_r16_from_memory_stack() {
        // POP BC ; with SP = FFFCh and memory[FFFCh]=5Fh, memory[FFFD]=3Ch -> B=3Ch, C=5Fh, SP=FFFE
        let mut cpu = Cpu::new();

        // Set SP and memory values representing the stack
        cpu.registers.sp = 0xFFFC;
        cpu.memory_bus.write_byte(0xFFFC, 0x5F); // low byte (C)
        cpu.memory_bus.write_byte(0xFFFD, 0x3C); // high byte (B)

        let opcode = 0b11000001; // POP BC (0xC1)
        cpu.execute(opcode);

        // After pop, SP should be incremented by 2
        assert_eq!(cpu.registers.sp, 0xFFFE, "SP should be incremented by 2 after POP");

        // Registers should be loaded from memory: B = 0x3C, C = 0x5F
        assert_eq!(cpu.registers.b, 0x3C, "B should contain high byte popped from stack");
        assert_eq!(cpu.registers.c, 0x5F, "C should contain low byte popped from stack");
    }
}
