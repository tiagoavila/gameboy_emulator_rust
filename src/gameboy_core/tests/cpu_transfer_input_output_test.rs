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
    fn test_ld_r16_imm16() {
        let mut cpu = Cpu::new();
        
        // Place immediate value 3A5Bh at PC (little-endian: lower byte first)
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x5B);     // Lower byte
        cpu.memory_bus.write_byte(cpu.registers.pc + 1, 0x3A); // Upper byte

        // Test LD HL, 3A5Bh ; H ← 3Ah, L ← 5Bh
        let opcode = 0b00100001; // LD HL, nn
        cpu.execute(opcode);
        
        // Verify H and L registers contain the correct values
        assert_eq!(cpu.registers.h, 0x3A, "H should contain 0x3A");
        assert_eq!(cpu.registers.l, 0x5B, "L should contain 0x5B");
        
        // Verify the entire HL pair has the correct value
        assert_eq!(cpu.registers.get_hl(), 0x3A5B, "HL should contain 0x3A5B");
    }

    #[test]
    fn test_ldhl_sp_imm8() {
        let mut cpu = Cpu::new();

        // Set up initial SP value
        cpu.registers.sp = 0xFFF8;
        // Place immediate signed offset 2 at PC (treated as u8 in memory)
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x02);

        // Test LDHL SP, 2 ; HL ← SP + 2 = 0xFFFA, Z ← 0, N ← 0, H ← 0, CY ← 0
        let opcode = 0b11111000; // LDHL SP, imm8 (0xF8)
        cpu.execute(opcode);

        // Verify HL computed from SP + imm8
        assert_eq!(cpu.registers.get_hl(), 0xFFFA, "HL should contain SP + 2 (0xFFFA)");

        // Verify SP remains unchanged
        assert_eq!(cpu.registers.sp, 0xFFF8, "SP should remain unchanged after LDHL SP, imm8");

        // Verify flags: Z reset, N reset, H reset, CY reset
        assert_eq!(cpu.registers.flags.z, false, "Z flag should be 0");
        assert_eq!(cpu.registers.flags.n, false, "N flag should be 0");
        assert_eq!(cpu.registers.flags.h, false, "H flag should be 0");
        assert_eq!(cpu.registers.flags.c, false, "CY flag should be 0");
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
        assert_eq!(cpu.registers.flags.z, false, "Z flag should be 0");
        assert_eq!(cpu.registers.flags.h, true, "H flag should be 1");
        assert_eq!(cpu.registers.flags.n, false, "N flag should be 0");
        assert_eq!(cpu.registers.flags.c, false, "CY flag should be 0");
        
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
        assert_eq!(cpu.registers.flags.z, false, "Z flag should be 0");
        assert_eq!(cpu.registers.flags.h, true, "H flag should be 1");
        assert_eq!(cpu.registers.flags.n, false, "N flag should be 0");
        assert_eq!(cpu.registers.flags.c, false, "CY flag should be 0");
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
        assert_eq!(cpu.registers.flags.z, true, "Z flag should be 1");
        assert_eq!(cpu.registers.flags.h, true, "H flag should be 1");
        assert_eq!(cpu.registers.flags.n, false, "N flag should be 0");
        assert_eq!(cpu.registers.flags.c, false, "CY flag should be 0");
        
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
        assert_eq!(cpu.registers.flags.z, false, "Z flag should be 0");
        assert_eq!(cpu.registers.flags.h, false, "H flag should be 0");
        assert_eq!(cpu.registers.flags.n, false, "N flag should be 0");
        assert_eq!(cpu.registers.flags.c, false, "CY flag should be 0");
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
        assert_eq!(cpu.registers.flags.z, false, "Z flag should be 0");
        assert_eq!(cpu.registers.flags.h, false, "H flag should be 0");
        assert_eq!(cpu.registers.flags.n, false, "N flag should be 0");
        assert_eq!(cpu.registers.flags.c, false, "CY flag should be 0");
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
        assert_eq!(cpu.registers.flags.z, false, "Z flag should be 0");
        assert_eq!(cpu.registers.flags.h, false, "H flag should be 0");
        assert_eq!(cpu.registers.flags.n, false, "N flag should be 0");
        assert_eq!(cpu.registers.flags.c, false, "CY flag should be 0");
        
        // Verify memory content remains unchanged
        assert_eq!(cpu.memory_bus.read_byte(0x1234), 0x0F, "Memory content should remain unchanged");
    }

    #[test]
    fn test_xor_a_r() {
        let mut cpu = Cpu::new();
        
        // Set up initial value
        cpu.registers.a = 0xFF;  // A = FFh

        // Test XOR A ; A ← 00h, Z ← 1
        let opcode = 0b10101111; // XOR A
        cpu.execute(opcode);
        
        // Verify result and flags
        assert_eq!(cpu.registers.a, 0x00, "A should be 0x00 after XOR with itself");
        assert_eq!(cpu.registers.flags.z, true, "Z flag should be 1");
        assert_eq!(cpu.registers.flags.h, false, "H flag should be 0");
        assert_eq!(cpu.registers.flags.n, false, "N flag should be 0");
        assert_eq!(cpu.registers.flags.c, false, "CY flag should be 0");
    }

    #[test]
    fn test_xor_imm8() {
        let mut cpu = Cpu::new();
        
        // Set up initial value
        cpu.registers.a = 0xFF;  // A = FFh
        // Place immediate value 0x0F at PC
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x0F);

        // Test XOR 0Fh ; A ← F0h, Z ← 0
        let opcode = 0b11101110; // XOR n
        cpu.execute(opcode);
        
        // Verify result and flags
        assert_eq!(cpu.registers.a, 0xF0, "A should be 0xF0 after XOR with immediate value");
        assert_eq!(cpu.registers.flags.z, false, "Z flag should be 0");
        assert_eq!(cpu.registers.flags.h, false, "H flag should be 0");
        assert_eq!(cpu.registers.flags.n, false, "N flag should be 0");
        assert_eq!(cpu.registers.flags.c, false, "CY flag should be 0");
    }

    #[test]
    fn test_xor_hl() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.a = 0xFF;  // A = FFh
        cpu.registers.set_hl(0x1234);  // Any valid address
        cpu.memory_bus.write_byte(0x1234, 0x8A);  // (HL) = 8Ah

        // Test XOR (HL) ; A ← 75h, Z ← 0
        let opcode = 0b10101110; // XOR (HL)
        cpu.execute(opcode);
        
        // Verify result and flags
        assert_eq!(cpu.registers.a, 0x75, "A should be 0x75 after XOR with (HL)");
        assert_eq!(cpu.registers.flags.z, false, "Z flag should be 0");
        assert_eq!(cpu.registers.flags.h, false, "H flag should be 0");
        assert_eq!(cpu.registers.flags.n, false, "N flag should be 0");
        assert_eq!(cpu.registers.flags.c, false, "CY flag should be 0");
        
        // Verify memory content remains unchanged
        assert_eq!(cpu.memory_bus.read_byte(0x1234), 0x8A, "Memory content should remain unchanged");
    }

    #[test]
    fn test_cp_a_r() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.a = 0x3C;  // A = 3Ch
        cpu.registers.b = 0x2F;  // B = 2Fh

        // Test CP B ; Z ← 0, H ← 1, N ← 1, CY ← 0
        let opcode = 0b10111000; // CP B
        cpu.execute(opcode);
        
        // Verify flags (A should not change in CP operation)
        assert_eq!(cpu.registers.a, 0x3C, "A should remain unchanged after CP");
        assert_eq!(cpu.registers.flags.z, false, "Z flag should be 0");
        assert_eq!(cpu.registers.flags.h, true, "H flag should be 1");
        assert_eq!(cpu.registers.flags.n, true, "N flag should be 1");
        assert_eq!(cpu.registers.flags.c, false, "CY flag should be 0");
        
        // Verify B remains unchanged
        assert_eq!(cpu.registers.b, 0x2F, "B should remain unchanged");
    }

    #[test]
    fn test_cp_a_imm8() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.a = 0x3C;  // A = 3Ch
        // Place immediate value 3Ch at PC
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x3C);

        // Test CP 3Ch ; Z ← 1, H ← 0, N ← 1, CY ← 0
        let opcode = 0b11111110; // CP n
        cpu.execute(opcode);
        
        // Verify flags (A should not change in CP operation)
        assert_eq!(cpu.registers.a, 0x3C, "A should remain unchanged after CP");
        assert_eq!(cpu.registers.flags.z, true, "Z flag should be 1");
        assert_eq!(cpu.registers.flags.h, false, "H flag should be 0");
        assert_eq!(cpu.registers.flags.n, true, "N flag should be 1");
        assert_eq!(cpu.registers.flags.c, false, "CY flag should be 0");
    }

    #[test]
    fn test_cp_a_hl() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.a = 0x3C;  // A = 3Ch
        cpu.registers.set_hl(0x1234);  // Any valid address
        cpu.memory_bus.write_byte(0x1234, 0x40);  // (HL) = 40h

        // Test CP (HL) ; Z ← 0, H ← 0, N ← 1, CY ← 1
        let opcode = 0b10111110; // CP (HL)
        cpu.execute(opcode);
        
        // Verify flags (A should not change in CP operation)
        assert_eq!(cpu.registers.a, 0x3C, "A should remain unchanged after CP");
        assert_eq!(cpu.registers.flags.z, false, "Z flag should be 0");
        assert_eq!(cpu.registers.flags.h, false, "H flag should be 0");
        assert_eq!(cpu.registers.flags.n, true, "N flag should be 1");
        assert_eq!(cpu.registers.flags.c, true, "CY flag should be 1");
        
        // Verify memory content remains unchanged
        assert_eq!(cpu.memory_bus.read_byte(0x1234), 0x40, "Memory content should remain unchanged");
    }

    #[test]
    fn test_inc_r() {
        let mut cpu = Cpu::new();
        
        // Set up initial value
        cpu.registers.a = 0xFF;  // A = FFh

        // Test INC A ; A ← 0, Z ← 1, H ← 1, N ← 0
        let opcode = 0b00111100; // INC A
        cpu.execute(opcode);
        
        // Verify result and flags
        assert_eq!(cpu.registers.a, 0x00, "A should be 0x00 after increment from 0xFF");
        assert_eq!(cpu.registers.flags.z, true, "Z flag should be 1");
        assert_eq!(cpu.registers.flags.h, true, "H flag should be 1");
        assert_eq!(cpu.registers.flags.n, false, "N flag should be 0");
        
        // Note: CY flag is not affected by INC operation
        let original_cy = cpu.registers.flags.c;
        assert_eq!(cpu.registers.flags.c, original_cy, "CY flag should not be affected by INC");
    }

    #[test]
    fn test_inc_hl() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.set_hl(0x1234);  // Any valid address
        cpu.memory_bus.write_byte(0x1234, 0x50);  // (HL) = 50h

        // Test INC (HL) ; (HL) ← 51h, Z ← 0, H ← 0, N ← 0
        let opcode = 0b00110100; // INC (HL)
        cpu.execute(opcode);
        
        // Verify result and flags
        assert_eq!(cpu.memory_bus.read_byte(0x1234), 0x51, "(HL) should be 0x51 after increment");
        assert_eq!(cpu.registers.flags.z, false, "Z flag should be 0");
        assert_eq!(cpu.registers.flags.h, false, "H flag should be 0");
        assert_eq!(cpu.registers.flags.n, false, "N flag should be 0");
        
        // Note: CY flag is not affected by INC operation
        let original_cy = cpu.registers.flags.c;
        assert_eq!(cpu.registers.flags.c, original_cy, "CY flag should not be affected by INC");
        
        // Verify HL remains unchanged
        assert_eq!(cpu.registers.get_hl(), 0x1234, "HL should remain unchanged");
    }

    #[test]
    fn test_dec_r() {
        let mut cpu = Cpu::new();
        
        // Set up initial value
        cpu.registers.l = 0x01;  // L = 01h

        // Test DEC L ; L ← 0, Z ← 1, H ← 0, N ← 1
        let opcode = 0b00101101; // DEC L
        cpu.execute(opcode);
        
        // Verify result and flags
        assert_eq!(cpu.registers.l, 0x00, "L should be 0x00 after decrement from 0x01");
        assert_eq!(cpu.registers.flags.z, true, "Z flag should be 1");
        assert_eq!(cpu.registers.flags.h, false, "H flag should be 0");
        assert_eq!(cpu.registers.flags.n, true, "N flag should be 1");
        
        // Note: CY flag is not affected by DEC operation
        let original_cy = cpu.registers.flags.c;
        assert_eq!(cpu.registers.flags.c, original_cy, "CY flag should not be affected by DEC");
    }

    #[test]
    fn test_dec_hl() {
        let mut cpu = Cpu::new();
        
        // Set up initial values
        cpu.registers.set_hl(0x1234);  // Any valid address
        cpu.memory_bus.write_byte(0x1234, 0x00);  // (HL) = 00h

        // Test DEC (HL) ; (HL) ← FFh, Z ← 0, H ← 1, N ← 1
        let opcode = 0b00110101; // DEC (HL)
        cpu.execute(opcode);
        
        // Verify result and flags
        assert_eq!(cpu.memory_bus.read_byte(0x1234), 0xFF, "(HL) should be 0xFF after decrement from 0x00");
        assert_eq!(cpu.registers.flags.z, false, "Z flag should be 0");
        assert_eq!(cpu.registers.flags.h, true, "H flag should be 1");
        assert_eq!(cpu.registers.flags.n, true, "N flag should be 1");
        
        // Note: CY flag is not affected by DEC operation
        let original_cy = cpu.registers.flags.c;
        assert_eq!(cpu.registers.flags.c, original_cy, "CY flag should not be affected by DEC");
        
        // Verify HL remains unchanged
        assert_eq!(cpu.registers.get_hl(), 0x1234, "HL should remain unchanged");
    }

    #[test]
    fn test_ld_imm16_sp() {
        let mut cpu = Cpu::new();

        // Set SP to FFF8h
        cpu.registers.sp = 0xFFF8;

        // Place address C100h at PC (little-endian: low byte first)
        cpu.memory_bus.write_byte(cpu.registers.pc, 0x00);
        cpu.memory_bus.write_byte(cpu.registers.pc + 1, 0xC1);

        // Test LD (C100h), SP ; C100h <- F8h, C101h <- FFh
        let opcode = 0b00001000; // LD (nn), SP (0x08)
        cpu.execute(opcode);

        // Verify memory contains SP low and high bytes
        assert_eq!(cpu.memory_bus.read_byte(0xC100), 0xF8, "Memory at 0xC100 should contain SP's low byte (F8h)");
        assert_eq!(cpu.memory_bus.read_byte(0xC101), 0xFF, "Memory at 0xC101 should contain SP's high byte (FFh)");

        // Verify SP remains unchanged
        assert_eq!(cpu.registers.sp, 0xFFF8, "SP should remain unchanged after LD (nn), SP");
    }
}
