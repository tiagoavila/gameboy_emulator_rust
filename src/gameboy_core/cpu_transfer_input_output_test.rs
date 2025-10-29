#[cfg(test)]
mod tests {
    use crate::gameboy_core::cpu::Cpu;

    #[test]
    fn test_ld_hli_a() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.set_hl(0xFFFF);  // HL = FFFFh
        cpu.registers.a = 0x56;        // A = 56h

        // Test LD (HLI), A ; (FFFFh) ← 56h, HL = 0000h
        let opcode = 0b00100010; // LD (HLI), A
        cpu.execute(opcode);
        
        // Verify the value was written to memory correctly
        assert_eq!(cpu.memory_bus.read_byte(0xFFFF), 0x56, "Memory at initial HL should contain A's value");
        
        // Verify HL was incremented (with overflow to 0000h)
        assert_eq!(cpu.registers.get_hl(), 0x0000, "HL should overflow to 0x0000");
        
        // Verify A remains unchanged
        assert_eq!(cpu.registers.a, 0x56, "A should remain unchanged");
    }

    #[test]
    fn test_ld_hld_a() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.set_hl(0x4000);  // HL = 4000h
        cpu.registers.a = 0x05;        // A = 5h

        // Test LD (HLD), A ; (4000h) ← 5h, HL = 3FFFh
        let opcode = 0b00110010; // LD (HLD), A
        cpu.execute(opcode);
        
        // Verify the value was written to memory correctly
        assert_eq!(cpu.memory_bus.read_byte(0x4000), 0x05, "Memory at initial HL should contain A's value");
        
        // Verify HL was decremented
        assert_eq!(cpu.registers.get_hl(), 0x3FFF, "HL should be decremented to 0x3FFF");
        
        // Verify A remains unchanged
        assert_eq!(cpu.registers.a, 0x05, "A should remain unchanged");
    }

    #[test]
    fn test_ld_de_a() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.d = 0x20;  // DE = 205Ch
        cpu.registers.e = 0x5C;
        cpu.registers.a = 0x00;  // A = 00h

        // Test LD (DE), A ; (205Ch) ← 00h
        let opcode = 0b00010010; // LD (DE), A
        cpu.execute(opcode);
        
        // Verify the value was written to memory correctly
        assert_eq!(cpu.memory_bus.read_byte(0x205C), 0x00, "Memory at (DE) should contain A's value");
        
        // Verify registers remain unchanged
        assert_eq!(cpu.registers.d, 0x20, "D should remain unchanged");
        assert_eq!(cpu.registers.e, 0x5C, "E should remain unchanged");
        assert_eq!(cpu.registers.a, 0x00, "A should remain unchanged");
    }

    #[test]
    fn test_ld_bc_a() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.b = 0x20;  // BC = 205Fh
        cpu.registers.c = 0x5F;
        cpu.registers.a = 0x3F;  // A = 3Fh

        // Test LD (BC), A ; (205Fh) ← 3Fh
        let opcode = 0b00000010; // LD (BC), A
        cpu.execute(opcode);
        
        // Verify the value was written to memory correctly
        assert_eq!(cpu.memory_bus.read_byte(0x205F), 0x3F, "Memory at (BC) should contain A's value");
        
        // Verify registers remain unchanged
        assert_eq!(cpu.registers.get_bc(), 0x205F, "BC should remain unchanged");
        assert_eq!(cpu.registers.a, 0x3F, "A should remain unchanged");
    }

    #[test]
    fn test_ld_a_hld() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.set_hl(0x8A5C);  // HL = 8A5Ch
        cpu.memory_bus.write_byte(0x8A5C, 0x3C);  // (8A5Ch) = 3Ch

        // Test LD A, (HLD) ; A ← 3Ch, HL ← 8A5Bh
        let opcode = 0b00111010; // LD A, (HLD)
        cpu.execute(opcode);
        
        // Verify A contains the value from memory
        assert_eq!(cpu.registers.a, 0x3C, "A should contain the value from memory at 0x8A5C");
        
        // Verify HL was decremented
        assert_eq!(cpu.registers.get_hl(), 0x8A5B, "HL should be decremented to 0x8A5B");
        
        // Verify memory content remains unchanged
        assert_eq!(cpu.memory_bus.read_byte(0x8A5C), 0x3C, "Memory content should remain unchanged");
    }

    #[test]
    fn test_ld_a_hli() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.set_hl(0x01FF);  // HL = 1FFh
        cpu.memory_bus.write_byte(0x01FF, 0x56);  // (1FFh) = 56h

        // Test LD A, (HLI) ; A ← 56h, HL ← 200h
        let opcode = 0b00101010; // LD A, (HLI)
        cpu.execute(opcode);
        
        // Verify A contains the value from memory
        assert_eq!(cpu.registers.a, 0x56, "A should contain the value from memory at 0x01FF");
        
        // Verify HL was incremented
        assert_eq!(cpu.registers.get_hl(), 0x0200, "HL should be incremented to 0x0200");
        
        // Verify memory content remains unchanged
        assert_eq!(cpu.memory_bus.read_byte(0x01FF), 0x56, "Memory content should remain unchanged");
    }

    #[test]
    fn test_ld_imm16_a() {
        let mut cpu = Cpu::new();
        
        // Test case 1: LD (FF44h), A ; (LY) ← A
        cpu.registers.a = 0x42;
        // Place address FF44h at PC (little-endian: lower byte first)
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x44);
        cpu.memory_bus.write_byte(cpu.registers.pc + 1, 0xFF);

        // Test LD (FF44h), A
        let opcode = 0b11101010; // LD (nn), A
        cpu.execute(opcode);
        
        // Verify the value was written to LY register location correctly
        assert_eq!(cpu.memory_bus.read_byte(0xFF44), 0x42, "Memory at 0xFF44 (LY) should contain A's value");
        
        // Test case 2: LD (8000h), A ; (8000h) ← A
        cpu.registers.a = 0x67;
        // Place address 8000h at PC (little-endian: lower byte first)
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x00);
        cpu.memory_bus.write_byte(cpu.registers.pc + 1, 0x80);

        // Test LD (8000h), A
        cpu.execute(opcode);
        
        // Verify the value was written to memory correctly
        assert_eq!(cpu.memory_bus.read_byte(0x8000), 0x67, "Memory at 0x8000 should contain A's value");
        
        // Verify register A remains unchanged after both operations
        assert_eq!(cpu.registers.a, 0x67, "A should remain unchanged");
    }

    #[test]
    fn test_ld_a_imm16() {
        let mut cpu = Cpu::new();
        
        // Test case 1: LD A, (FF44h) ; A ← (LY)
        // Write test value to LY register location
        cpu.memory_bus.write_byte(0xFF44, 0x42);
        // Place address FF44h at PC (little-endian: lower byte first)
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x44);
        cpu.memory_bus.write_byte(cpu.registers.pc + 1, 0xFF);

        // Test LD A, (FF44h)
        let opcode = 0b11111010; // LD A, (nn)
        cpu.execute(opcode);
        
        // Verify A contains the value from FF44h
        assert_eq!(cpu.registers.a, 0x42, "A should contain the value from memory at 0xFF44 (LY)");
        
        // Test case 2: LD A, (8000h) ; A ← (8000h)
        // Reset A and write test value to memory
        cpu.registers.a = 0x00;
        cpu.memory_bus.write_byte(0x8000, 0x67);
        // Place address 8000h at PC (little-endian: lower byte first)
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x00);
        cpu.memory_bus.write_byte(cpu.registers.pc + 1, 0x80);

        // Test LD A, (8000h)
        cpu.execute(opcode);
        
        // Verify A contains the value from 8000h
        assert_eq!(cpu.registers.a, 0x67, "A should contain the value from memory at 0x8000");
        
        // Verify memory contents remain unchanged
        assert_eq!(cpu.memory_bus.read_byte(0xFF44), 0x42, "Memory content at 0xFF44 should remain unchanged");
        assert_eq!(cpu.memory_bus.read_byte(0x8000), 0x67, "Memory content at 0x8000 should remain unchanged");
    }

    #[test]
    fn test_ld_high_mem_a() {
        let mut cpu = Cpu::new();
        
        // Set up register A with test value
        cpu.registers.a = 0x42;
        
        // Place the low byte of the address (0x34) at PC
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x34);

        // Test LD (FF34h), A ; FF34h ← contents of A
        let opcode = 0b11100000; // LDH (n), A
        cpu.execute(opcode);
        
        // Verify the value was written to high memory correctly
        assert_eq!(cpu.memory_bus.read_byte(0xFF34), 0x42, "Memory at 0xFF34 should contain A's value");
        
        // Verify register A remains unchanged
        assert_eq!(cpu.registers.a, 0x42, "A should remain unchanged");
    }

    #[test]
    fn test_ld_a_high_mem() {
        let mut cpu = Cpu::new();
        
        // Write test value to high memory location FF34h
        cpu.memory_bus.write_byte(0xFF34, 0x42);
        
        // Place the low byte of the address (0x34) at PC
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x34);

        // Test LD A, (FF34h) ; A ← contents of FF34h
        let opcode = 0b11110000; // LDH A, (n)
        cpu.execute(opcode);
        
        // Verify A contains the value from high memory
        assert_eq!(cpu.registers.a, 0x42, "A should contain the value from memory at 0xFF34");
        
        // Verify memory content remains unchanged
        assert_eq!(cpu.memory_bus.read_byte(0xFF34), 0x42, "Memory content at 0xFF34 should remain unchanged");
    }

    #[test]
    fn test_ld_c_a() {
        let mut cpu = Cpu::new();
        
        // Set up registers
        cpu.registers.c = 0x9F;
        cpu.registers.a = 0x42; // arbitrary test value for A

        // Test LD (C), A ; (FF9Fh) ← A
        let opcode = 0b11100010; // LD (C), A
        cpu.execute(opcode);
        
        // Verify the value was written to high memory correctly
        assert_eq!(cpu.memory_bus.read_byte(0xFF9F), 0x42, "Memory at 0xFF9F should contain A's value");
        
        // Verify registers remain unchanged
        assert_eq!(cpu.registers.a, 0x42, "A should remain unchanged");
        assert_eq!(cpu.registers.c, 0x9F, "C should remain unchanged");
    }

    #[test]
    fn test_ld_a_c() {
        let mut cpu = Cpu::new();
        
        // Set up C register
        cpu.registers.c = 0x95;
        
        // Write test value to high memory location FF95h
        cpu.memory_bus.write_byte(0xFF95, 0x42); // arbitrary test value

        // Test LD A, (C) ; A ← contents of (FF95h)
        let opcode = 0b11110010; // LD A, (C)
        cpu.execute(opcode);
        
        // Verify A contains the value from high memory (0xFF00 + C)
        assert_eq!(cpu.registers.a, 0x42, "A should contain the value from memory at 0xFF95");
        
        // Verify memory and C remain unchanged
        assert_eq!(cpu.memory_bus.read_byte(0xFF95), 0x42, "Memory content should remain unchanged");
        assert_eq!(cpu.registers.c, 0x95, "C should remain unchanged");
    }

    #[test]
    fn test_ld_a_de() {
        let mut cpu = Cpu::new();
        
        // Set up DE register pair (using D and E individually)
        cpu.registers.d = 0x12;
        cpu.registers.e = 0x34;
        
        // Write test value to memory at DE address
        cpu.memory_bus.write_byte(0x1234, 0x5F);

        // Test LD A, (DE) ; A ← 5Fh
        let opcode = 0b00011010; // LD A, (DE)
        cpu.execute(opcode);
        
        // Verify A contains the value from memory pointed by DE
        assert_eq!(cpu.registers.a, 0x5F, "A should contain the value from memory pointed by DE");
        
        // Verify memory and DE remain unchanged
        assert_eq!(cpu.memory_bus.read_byte(0x1234), 0x5F, "Memory content should remain unchanged");
        assert_eq!(cpu.registers.d, 0x12, "D should remain unchanged");
        assert_eq!(cpu.registers.e, 0x34, "E should remain unchanged");
    }

    #[test]
    fn test_ld_a_bc() {
        let mut cpu = Cpu::new();
        
        // Set up BC to point to memory location and write the test value
        cpu.registers.b = 0x12;
        cpu.registers.c = 0x34;
        cpu.memory_bus.write_byte(0x1234, 0x2F);

        // Test LD A, (BC) ; A ← 2Fh
        let opcode = 0b00001010; // LD A, (BC)
        cpu.execute(opcode);
        
        // Verify A contains the value from memory
        assert_eq!(cpu.registers.a, 0x2F, "A should contain the value from memory pointed by BC");
        
        // Verify BC and memory content remain unchanged
        assert_eq!(cpu.registers.get_bc(), 0x1234, "BC should remain unchanged");
        assert_eq!(cpu.memory_bus.read_byte(0x1234), 0x2F, "Memory content should remain unchanged");
    }

    #[test]
    fn test_ld_hl_imm8() {
        let mut cpu = Cpu::new();
        
        // Set up HL register
        cpu.registers.set_hl(0x8AC5);
        
        // Place immediate value 0x00 at PC
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x00);

        // Test LD (HL), 0 ; 8AC5h ← 0
        let opcode = 0b00110110; // LD (HL), n
        cpu.execute(opcode);
        
        // Verify the immediate value was written to memory correctly
        assert_eq!(cpu.memory_bus.read_byte(0x8AC5), 0x00, "Memory at HL should contain the immediate value 0");
        
        // Verify HL remains unchanged
        assert_eq!(cpu.registers.get_hl(), 0x8AC5, "HL should remain unchanged");
    }

    #[test]
    fn test_ld_hl_r8() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.a = 0x3C;
        cpu.registers.set_hl(0x8AC5);

        // Test LD (HL), A ; (8AC5h) ← 3Ch
        let opcode = 0b01110111; // LD (HL), A
        cpu.execute(opcode);
        
        // Verify the value was written to memory correctly
        assert_eq!(cpu.memory_bus.read_byte(0x8AC5), 0x3C, "Memory at HL should contain A's value");
        
        // Verify registers remain unchanged
        assert_eq!(cpu.registers.a, 0x3C, "A should remain unchanged");
        assert_eq!(cpu.registers.get_hl(), 0x8AC5, "HL should remain unchanged");
    }

    #[test]
    fn test_ld_r8_hl() {
        let mut cpu = Cpu::new();
        
        // Set up HL to point to memory location and write the test value
        cpu.registers.set_hl(0x1234);
        cpu.memory_bus.write_byte(0x1234, 0x5C);

        // Test LD H, (HL) ; H ← 5Ch
        let opcode = 0b01100110; // LD H, (HL)
        cpu.execute(opcode);
        assert_eq!(cpu.registers.h, 0x5C, "H should contain the value from memory pointed by HL");
        
        // Verify the value in memory remains unchanged
        assert_eq!(cpu.memory_bus.read_byte(0x1234), 0x5C, "Memory content should remain unchanged");
    }

    #[test]
    fn test_ld_r8_imm8() {
        let mut cpu = Cpu::new();
        
        // Place immediate value 0x24 at PC
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x24);

        // Test LD B, 24h ; B ← 24h
        let opcode = 0b00000110; // LD B, n
        cpu.execute(opcode);
        assert_eq!(cpu.registers.b, 0x24, "B should contain the immediate value 0x24");
    }

    #[test]
    fn test_ld_r8_r8() {
        let mut cpu = Cpu::new();
        
        // Set initial values
        cpu.registers.b = 0x42; // Random value for B
        cpu.registers.d = 0x8F; // Random value for D

        // Test LD A, B ; A ← B
        let opcode = 0b01111000; // LD A, B
        cpu.execute(opcode);
        assert_eq!(cpu.registers.a, 0x42, "A should contain B's value");

        // Test LD B, D ; B ← D
        let opcode = 0b01000010; // LD B, D
        cpu.execute(opcode);
        assert_eq!(cpu.registers.b, 0x8F, "B should contain D's value");
        
        // Verify D remains unchanged
        assert_eq!(cpu.registers.d, 0x8F, "D should remain unchanged");
    }

    #[test]
    fn test_and_a_r() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.a = 0x5A;  // A = 5Ah
        cpu.registers.l = 0x3F;  // L = 3Fh

        // Test AND L ; A ← 1Ah, Z ← 0, H ← 1, N ← 0 CY ← 0
        let opcode = 0b10100101; // AND L
        cpu.execute(opcode);
        
        // Verify result and flags
        assert_eq!(cpu.registers.a, 0x1A, "A should contain 0x1A after AND with L");
        assert_eq!(cpu.flags_register.z_flag, false, "Z flag should be 0");
        assert_eq!(cpu.flags_register.h_flag, true, "H flag should be 1");
        assert_eq!(cpu.flags_register.n_flag, false, "N flag should be 0");
        assert_eq!(cpu.flags_register.c_flag, false, "CY flag should be 0");
        
        // Verify L remains unchanged
        assert_eq!(cpu.registers.l, 0x3F, "L should remain unchanged");
    }

    #[test]
    fn test_and_a_imm8() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.a = 0x5A;  // A = 5Ah
        // Place immediate value 0x38 at PC
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x38);

        // Test AND 38h ; A ← 18h, Z ← 0, H ← 1, N ← 0 CY ← 0
        let opcode = 0b11100110; // AND n
        cpu.execute(opcode);
        
        // Verify result and flags
        assert_eq!(cpu.registers.a, 0x18, "A should contain 0x18 after AND with immediate value");
        assert_eq!(cpu.flags_register.z_flag, false, "Z flag should be 0");
        assert_eq!(cpu.flags_register.h_flag, true, "H flag should be 1");
        assert_eq!(cpu.flags_register.n_flag, false, "N flag should be 0");
        assert_eq!(cpu.flags_register.c_flag, false, "CY flag should be 0");
    }

    #[test]
    fn test_and_a_hl() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.a = 0x5A;  // A = 5Ah
        cpu.registers.set_hl(0x1234);  // Any valid address
        cpu.memory_bus.write_byte(0x1234, 0x00);  // (HL) = 0h

        // Test AND (HL) ; A ← 00h, Z ← 1, H ← 1, N ← 0 CY ← 0
        let opcode = 0b10100110; // AND (HL)
        cpu.execute(opcode);
        
        // Verify result and flags
        assert_eq!(cpu.registers.a, 0x00, "A should contain 0x00 after AND with (HL)");
        assert_eq!(cpu.flags_register.z_flag, true, "Z flag should be 1");
        assert_eq!(cpu.flags_register.h_flag, true, "H flag should be 1");
        assert_eq!(cpu.flags_register.n_flag, false, "N flag should be 0");
        assert_eq!(cpu.flags_register.c_flag, false, "CY flag should be 0");
        
        // Verify memory content remains unchanged
        assert_eq!(cpu.memory_bus.read_byte(0x1234), 0x00, "Memory content should remain unchanged");
    }

    #[test]
    fn test_or_a_r() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.a = 0x5A;  // A = 5Ah

        // Test OR A ; A ← 5Ah, Z ← 0
        let opcode = 0b10110111; // OR A
        cpu.execute(opcode);
        
        // Verify result and flags
        assert_eq!(cpu.registers.a, 0x5A, "A should remain 0x5A after OR with itself");
        assert_eq!(cpu.flags_register.z_flag, false, "Z flag should be 0");
        assert_eq!(cpu.flags_register.h_flag, false, "H flag should be 0");
        assert_eq!(cpu.flags_register.n_flag, false, "N flag should be 0");
        assert_eq!(cpu.flags_register.c_flag, false, "CY flag should be 0");
    }

    #[test]
    fn test_or_imm8() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.a = 0x5A;  // A = 5Ah
        // Place immediate value 3 at PC
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x03);

        // Test OR 3 ; A ← 5Bh, Z ← 0
        let opcode = 0b11110110; // OR n
        cpu.execute(opcode);
        
        // Verify result and flags
        assert_eq!(cpu.registers.a, 0x5B, "A should contain 0x5B after OR with immediate value");
        assert_eq!(cpu.flags_register.z_flag, false, "Z flag should be 0");
        assert_eq!(cpu.flags_register.h_flag, false, "H flag should be 0");
        assert_eq!(cpu.flags_register.n_flag, false, "N flag should be 0");
        assert_eq!(cpu.flags_register.c_flag, false, "CY flag should be 0");
    }

    #[test]
    fn test_or_hl() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.a = 0x5A;  // A = 5Ah
        cpu.registers.set_hl(0x1234);  // Any valid address
        cpu.memory_bus.write_byte(0x1234, 0x0F);  // (HL) = 0Fh

        // Test OR (HL) ; A ← 5Fh, Z ← 0
        let opcode = 0b10110110; // OR (HL)
        cpu.execute(opcode);
        
        // Verify result and flags
        assert_eq!(cpu.registers.a, 0x5F, "A should contain 0x5F after OR with (HL)");
        assert_eq!(cpu.flags_register.z_flag, false, "Z flag should be 0");
        assert_eq!(cpu.flags_register.h_flag, false, "H flag should be 0");
        assert_eq!(cpu.flags_register.n_flag, false, "N flag should be 0");
        assert_eq!(cpu.flags_register.c_flag, false, "CY flag should be 0");
        
        // Verify memory content remains unchanged
        assert_eq!(cpu.memory_bus.read_byte(0x1234), 0x0F, "Memory content should remain unchanged");
    }
}
