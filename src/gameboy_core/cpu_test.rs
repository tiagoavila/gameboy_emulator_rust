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
