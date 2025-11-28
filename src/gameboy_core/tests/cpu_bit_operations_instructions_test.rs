#[cfg(test)]
mod tests {
    use crate::gameboy_core::cpu::Cpu;

    #[test]
    fn test_bit_7_a() {
        // BIT 7, A ; Z ← 0, H ← 1, N ← 0
        // When A = 80h (0b10000000)
        let mut cpu = Cpu::new();
        cpu.registers.b = 0b10000000; // 0x80

        // Execute CB prefix followed by BIT 7, B (0xCB 0x78)
        let cb_opcode = 0xCB;
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x78); // BIT 7, B (0xCB 0x78)

        cpu.execute(cb_opcode);

        assert_eq!(cpu.registers.b, 0b10000000, "B should remain 0b10000000 (0x80) after BIT 7, B");
        assert_eq!(cpu.flags_register.z, false, "Z flag should be 0 (bit 7 is set)");
        assert_eq!(cpu.flags_register.h, true, "H flag should be 1");
        assert_eq!(cpu.flags_register.n, false, "N flag should be 0");
    }

    #[test]
    fn test_bit_4_l() {
        // BIT 4, L ; Z ← 1, H ← 1, N ← 0
        // When L = EFh (0b11101111)
        let mut cpu = Cpu::new();
        cpu.registers.l = 0b11101111; // 0xEF

        // Execute CB prefix followed by BIT 4, L (0xCB 0x65)
        let cb_opcode = 0xCB;
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x65); // BIT 4, L (0xCB 0x65)

        cpu.execute(cb_opcode);

        assert_eq!(cpu.registers.l, 0b11101111, "L should remain 0b11101111 (0xEF) after BIT 4, L");
        assert_eq!(cpu.flags_register.z, true, "Z flag should be 1 (bit 4 is not set)");
        assert_eq!(cpu.flags_register.h, true, "H flag should be 1");
        assert_eq!(cpu.flags_register.n, false, "N flag should be 0");
    }

    #[test]
    fn test_bit_0_hl() {
        // BIT 0, (HL) ; Z ← 1, H ← 1, N ← 0
        // When (HL) = FEh (0b11111110)
        let mut cpu = Cpu::new();
        cpu.registers.set_hl(0x1234);
        cpu.memory_bus.write_byte(0x1234, 0b11111110); // 0xFE

        // Execute CB prefix followed by BIT 0, (HL) (0xCB 0x46)
        let cb_opcode = 0xCB;
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x46); // BIT 0, (HL) (0xCB 0x46)

        cpu.execute(cb_opcode);

        assert_eq!(cpu.memory_bus.read_byte(0x1234), 0b11111110, "Memory at HL should remain 0b11111110 (0xFE) after BIT 0, (HL)");
        assert_eq!(cpu.flags_register.z, true, "Z flag should be 1 (bit 0 is not set)");
        assert_eq!(cpu.flags_register.h, true, "H flag should be 1");
        assert_eq!(cpu.flags_register.n, false, "N flag should be 0");
    }

    #[test]
    fn test_bit_1_hl() {
        // BIT 1, (HL) ; Z ← 0, H ← 1, N ← 0
        // When (HL) = FEh (0b11111110)
        let mut cpu = Cpu::new();
        cpu.registers.set_hl(0x1234);
        cpu.memory_bus.write_byte(0x1234, 0b11111110); // 0xFE

        // Execute CB prefix followed by BIT 1, (HL) (0xCB 0x4E)
        let cb_opcode = 0xCB;
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x4E); // BIT 1, (HL) (0xCB 0x4E)

        cpu.execute(cb_opcode);

        assert_eq!(cpu.memory_bus.read_byte(0x1234), 0b11111110, "Memory at HL should remain 0b11111110 (0xFE) after BIT 1, (HL)");
        assert_eq!(cpu.flags_register.z, false, "Z flag should be 0 (bit 1 is set)");
        assert_eq!(cpu.flags_register.h, true, "H flag should be 1");
        assert_eq!(cpu.flags_register.n, false, "N flag should be 0");
    }

    #[test]
    fn test_set_3_e() {
        // SET 3, E ; E ← 84h
        // When E = 80h (0b10000000)
        let mut cpu = Cpu::new();
        cpu.registers.e = 0b10000000; // 0x80

        // Execute CB prefix followed by SET 3, E (0xCB 0xDB)
        let cb_opcode = 0xCB;
        cpu.memory_bus.write_byte(cpu.registers.pc, 0xDB); // SET 3, E (0xCB 0xDB)

        cpu.execute(cb_opcode);

        assert_eq!(cpu.registers.e, 0b10001000, "E should be 0b10001000 (0x88) after SET 3, E");
    }

    #[test]
    fn test_set_7_l() {
        // SET 7, L ; L ← BBh
        // When L = 3Bh (0b00111011)
        let mut cpu = Cpu::new();
        cpu.registers.l = 0b00111011; // 0x3B

        // Execute CB prefix followed by SET 7, L (0xCB 0xFD)
        let cb_opcode = 0xCB;
        cpu.memory_bus.write_byte(cpu.registers.pc, 0xFD); // SET 7, L (0xCB 0xFD)

        cpu.execute(cb_opcode);

        assert_eq!(cpu.registers.l, 0b10111011, "L should be 0b10111011 (0xBB) after SET 7, L");
    }

    #[test]
    fn test_set_3_hl() {
        // SET 3, (HL) ; (HL) ← 04h
        // When (HL) = 00h (0b00000000)
        let mut cpu = Cpu::new();
        cpu.registers.set_hl(0x1234);
        cpu.memory_bus.write_byte(0x1234, 0b00000000); // 0x00

        // Execute CB prefix followed by SET 3, (HL) (0xCB 0xDE)
        let cb_opcode = 0xCB;
        cpu.memory_bus.write_byte(cpu.registers.pc, 0xDE); // SET 3, (HL) (0xCB 0xDE)

        cpu.execute(cb_opcode);

        assert_eq!(cpu.memory_bus.read_byte(0x1234), 0b00001000, "Memory at HL should be 0b00001000 (0x08) after SET 3, (HL)");
    }

    #[test]
    fn test_res_3_e() {
        // RES 3, E ; E ← 80h (clear bit 3)
        // When E = 88h (0b10001000)
        let mut cpu = Cpu::new();
        cpu.registers.e = 0b10001000; // 0x88

        // Execute CB prefix followed by RES 3, E (0xCB 0x9B)
        let cb_opcode = 0xCB;
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x9B); // RES 3, E (0xCB 0x9B)

        cpu.execute(cb_opcode);

        assert_eq!(cpu.registers.e, 0b10000000, "E should be 0b10000000 (0x80) after RES 3, E");
    }

    #[test]
    fn test_res_7_l() {
        // RES 7, L ; L ← 3Bh (clear bit 7)
        // When L = BBh (0b10111011)
        let mut cpu = Cpu::new();
        cpu.registers.l = 0b10111011; // 0xBB

        // Execute CB prefix followed by RES 7, L (0xCB 0xBD)
        let cb_opcode = 0xCB;
        cpu.memory_bus.write_byte(cpu.registers.pc, 0xBD); // RES 7, L (0xCB 0xBD)

        cpu.execute(cb_opcode);

        assert_eq!(cpu.registers.l, 0b00111011, "L should be 0b00111011 (0x3B) after RES 7, L");
    }

    #[test]
    fn test_res_3_hl() {
        // RES 3, (HL) ; (HL) ← 00h (clear bit 3)
        // When (HL) = 08h (0b00001000)
        let mut cpu = Cpu::new();
        cpu.registers.set_hl(0x1234);
        cpu.memory_bus.write_byte(0x1234, 0b00001000); // 0x08

        // Execute CB prefix followed by RES 3, (HL) (0xCB 0x9E)
        let cb_opcode = 0xCB;
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x9E); // RES 3, (HL) (0xCB 0x9E)

        cpu.execute(cb_opcode);

        assert_eq!(cpu.memory_bus.read_byte(0x1234), 0b00000000, "Memory at HL should be 0b00000000 (0x00) after RES 3, (HL)");
    }
}
