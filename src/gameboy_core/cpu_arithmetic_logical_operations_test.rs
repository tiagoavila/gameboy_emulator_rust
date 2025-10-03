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
}
