#[cfg(test)]
mod tests {
    use crate::gameboy_core::cpu::Cpu;

    #[test]
    fn test_rlca() {
        // RLCA ; A ← 0Ah, CY ← 1, Z ← 0, H ← 0, N ← 0
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b10000101;
        cpu.flags_register.c = false;

        let opcode = 0b00000111; // RLCA
        cpu.execute(opcode);

        assert_eq!(cpu.registers.a, 0b00001011, "A should be 0b00001010 after RLCA");
        assert_eq!(cpu.flags_register.c, true, "CY flag should be 1");
        assert_eq!(cpu.flags_register.z, false, "Z flag should be 0");
        assert_eq!(cpu.flags_register.h, false, "H flag should be 0");
        assert_eq!(cpu.flags_register.n, false, "N flag should be 0");
    }

    #[test]
    fn test_rla() {
        // RLA ; A ← 2Bh, C ← 1, Z ← 0, H ← 0, N ← 0
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b00010101; // 0x95
        cpu.flags_register.c = true;

        let opcode = 0b00010111; // RLA
        cpu.execute(opcode);

        assert_eq!(cpu.registers.a, 0b00101011, "A should be 0b00101011 (0x2B) after RLA");
        assert_eq!(cpu.flags_register.c, false, "C flag should be 0");
        assert_eq!(cpu.flags_register.z, false, "Z flag should be 0");
        assert_eq!(cpu.flags_register.h, false, "H flag should be 0");
        assert_eq!(cpu.flags_register.n, false, "N flag should be 0");
    }

    #[test]
    fn test_rla_c_flag_false() {
        // RLA ; A ← 2Bh, C ← 1, Z ← 0, H ← 0, N ← 0
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b10010101; // 0x95
        cpu.flags_register.c = false;

        let opcode = 0b00010111; // RLA
        cpu.execute(opcode);

        assert_eq!(cpu.registers.a, 0b00101010, "A should be 0b00101010 (0x2A) after RLA");
        assert_eq!(cpu.flags_register.c, true, "C flag should be 1");
        assert_eq!(cpu.flags_register.z, false, "Z flag should be 0");
        assert_eq!(cpu.flags_register.h, false, "H flag should be 0");
        assert_eq!(cpu.flags_register.n, false, "N flag should be 0");
    }

    #[test]
    fn test_rrca() {
        // RRCA ; A ← 9Dh, CY ← 1, Z ← 0, H ← 0, N ← 0
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b00111011; // 0x3B
        cpu.flags_register.c = false;

        let opcode = 0b00001111; // RRCA
        cpu.execute(opcode);

        assert_eq!(cpu.registers.a, 0b10011101, "A should be 0b10011101 (0x9D) after RRCA - bit 0 is moved to bit 7");
        assert_eq!(cpu.flags_register.c, true, "CY flag should be 1");
        assert_eq!(cpu.flags_register.z, false, "Z flag should be 0");
        assert_eq!(cpu.flags_register.h, false, "H flag should be 0");
        assert_eq!(cpu.flags_register.n, false, "N flag should be 0");
    }

    #[test]
    fn test_rra() {
        // RRA ; A ← 40h, CY ← 1, Z ← 0, H ← 0, N ← 0
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b10000001; // 0x81
        cpu.flags_register.c = false;

        let opcode = 0b00011111; // RRA
        cpu.execute(opcode);

        assert_eq!(cpu.registers.a, 0b01000000, "A should be 0b01000000 (0x40) after RRA");
        assert_eq!(cpu.flags_register.c, true, "CY flag should be 1");
        assert_eq!(cpu.flags_register.z, false, "Z flag should be 0");
        assert_eq!(cpu.flags_register.h, false, "H flag should be 0");
        assert_eq!(cpu.flags_register.n, false, "N flag should be 0");
    }

    #[test]
    fn test_rra_with_carry() {
        // RRA ; A ← C0h, CY ← 1, Z ← 0, H ← 0, N ← 0
        // When A = 81h and CY = 1 (same initial A value, but with CY = 1)
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b10000001; // 0x81
        cpu.flags_register.c = true;

        let opcode = 0b00011111; // RRA
        cpu.execute(opcode);

        assert_eq!(cpu.registers.a, 0b11000000, "A should be 0b11000000 (0xC0) after RRA with CY=1");
        assert_eq!(cpu.flags_register.c, true, "CY flag should be 1");
        assert_eq!(cpu.flags_register.z, false, "Z flag should be 0");
        assert_eq!(cpu.flags_register.h, false, "H flag should be 0");
        assert_eq!(cpu.flags_register.n, false, "N flag should be 0");
    }

    #[test]
    fn test_rlc_r8() {
        // RLC B ; B ← 0b00001011, CY ← 1, Z ← 0, H ← 0, N ← 0
        // When B = 0b10000101
        let mut cpu = Cpu::new();
        cpu.registers.b = 0b10000101;

        let opcode = 0xCB; // Prefix for CB-prefixed instructions
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x00); // RLC B (0xCB 0x00)
        cpu.registers.increment_pc();

        cpu.execute(opcode);

        assert_eq!(cpu.registers.b, 0b00001011, "B should be 0b00001010 after RLC B");
        assert_eq!(cpu.flags_register.c, true, "CY flag should be 1");
        assert_eq!(cpu.flags_register.z, false, "Z flag should be 0");
        assert_eq!(cpu.flags_register.h, false, "H flag should be 0");
        assert_eq!(cpu.flags_register.n, false, "N flag should be 0");
    }

    #[test]
    fn test_rlc_hl() {
        // RLC (HL) ; Memory at HL ← 0b00001011, CY ← 1, Z ← 0, H ← 0, N ← 0
        // When memory at HL = 0b10000101
        let mut cpu = Cpu::new();
        cpu.registers.set_hl(0x1234);
        cpu.memory_bus.write_byte(0x1234, 0b10000101);

        let cb_opcode = 0xCB; // Prefix for CB-prefixed instructions
        cpu.memory_bus.write_byte(cpu.registers.pc, cb_opcode); // CB prefix
        cpu.memory_bus.write_byte(cpu.registers.pc + 1, 0b00000110); // RLC (HL) (0xCB 0x06)

        cpu.registers.increment_pc(); // Move to CB opcode
        cpu.execute(cb_opcode);

        assert_eq!(cpu.memory_bus.read_byte(0x1234), 0b00001011, "Memory at HL should be 0b00001010 after RLC (HL)");
        assert_eq!(cpu.flags_register.c, true, "CY flag should be 1");
        assert_eq!(cpu.flags_register.z, false, "Z flag should be 0");
        assert_eq!(cpu.flags_register.h, false, "H flag should be 0");
        assert_eq!(cpu.flags_register.n, false, "N flag should be 0");
    }

    #[test]
    fn test_rrc_r8() {
        // RRC C ; C ← 80h, CY ← 1, Z ← 0, H ← 0, N ← 0
        // When C = 1h
        let mut cpu = Cpu::new();
        cpu.registers.c = 0x1; // 0b00000001

        let cb_opcode = 0xCB; // Prefix for CB-prefixed instructions
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x09); // RRC C (0xCB 0x09)

        cpu.execute(cb_opcode);

        assert_eq!(cpu.registers.c, 0x80, "C should be 0x80 (0b10000000) after RRC C");
        assert_eq!(cpu.flags_register.c, true, "CY flag should be 1");
        assert_eq!(cpu.flags_register.z, false, "Z flag should be 0");
        assert_eq!(cpu.flags_register.h, false, "H flag should be 0");
        assert_eq!(cpu.flags_register.n, false, "N flag should be 0");
    }

    #[test]
    fn test_rrc_hl() {
        // RRC (HL) ; (HL) ← 00h, CY ← 0, Z ← 1, H ← 0, N ← 0
        // When (HL) = 0h
        let mut cpu = Cpu::new();
        cpu.registers.set_hl(0x1234);
        cpu.memory_bus.write_byte(0x1234, 0x00);

        let cb_opcode = 0xCB; // Prefix for CB-prefixed instructions
        cpu.memory_bus.write_byte(cpu.registers.pc, cb_opcode); // CB prefix
        cpu.memory_bus.write_byte(cpu.registers.pc + 1, 0x0E); // RRC (HL) (0xCB 0x0E)

        cpu.registers.increment_pc(); // Move to CB opcode
        cpu.execute(cb_opcode);

        assert_eq!(cpu.memory_bus.read_byte(0x1234), 0x00, "Memory at HL should be 0x00 after RRC (HL)");
        assert_eq!(cpu.flags_register.c, false, "CY flag should be 0");
        assert_eq!(cpu.flags_register.z, true, "Z flag should be 1");
        assert_eq!(cpu.flags_register.h, false, "H flag should be 0");
        assert_eq!(cpu.flags_register.n, false, "N flag should be 0");
    }
}
