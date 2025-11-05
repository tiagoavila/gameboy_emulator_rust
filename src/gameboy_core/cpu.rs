use crate::gameboy_core::cpu_components::{FlagsRegister, MemoryBus, Registers};

pub struct Cpu {
    pub registers: Registers,
    pub flags_register: FlagsRegister,
    pub memory_bus: MemoryBus,
}

impl Cpu {
    const START_ADDRESS_FOR_LOAD_INSTRUCTIONS: u16 = 0xFF00;
    const EIGHT_BIT_REGISTERS: [u8; 7] = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111];
    const SIXTEEN_BIT_REGISTERS: [u8; 4] = [0b00, 0b01, 0b10, 0b11];

    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            flags_register: FlagsRegister::new(),
            memory_bus: MemoryBus::new(),
        }
    }

    pub fn tick(&mut self) {
        let opcode = self.fetch_opcode();
        self.registers.increment_pc();
        self.execute(opcode);
    }

    fn fetch_opcode(&mut self) -> u8 {
        self.memory_bus.read_byte(self.registers.pc)
    }

    // The first byte of each instruction is typically called the “opcode” (for “operation code”).
    // By noticing that some instructions perform identical operations but with different parameters, they can be grouped together;
    // for example, inc bc, inc de, inc hl, and inc sp differ only in what 16-bit register they modify.
    //
    // In each table, one line represents one such grouping. Since many groupings have some variation,
    // the variation has to be encoded in the instruction; for example, the above four instructions will be collectively
    // referred to as inc r16. Here are the possible placeholders and their values:
    //         0	1	2	3	4	5	6	    7
    // r8	    b	c	d	e	h	l	[hl]	a
    // r16	    bc	de	hl	sp
    // r16stk	bc	de	hl	af
    // r16mem	bc	de	hl+	hl-
    // cond	nz	z	nc	c
    // b3	A 3-bit bit index
    // tgt3	rst's target address, divided by 8
    // imm8	The following byte
    // imm16	The following two bytes, in little-endian order
    // Table of opcodes: https://gbdev.io/pandocs/CPU_Instruction_Set.html
    pub fn execute(&mut self, opcode: u8) {
        match opcode {
            0x00 => Cpu::nop(),        // NOP
            0b01110110 => self.halt(), // HALT

            // 8-Bit Transfer and Input/Output Instructions
                        //0b11011110
            v if (v & 0b11000111) == 0b01000110 && Cpu::destination_is_8bit_register(opcode) => {
                self.ld_r8_hl(opcode)
            }
            v if (v & 0b11111000) == 0b01110000 && Cpu::source_is_8bit_register(opcode) => {
                self.ld_hl_r8(opcode)
            }
            v if (v & 0b11000000) == 0b01000000
                && Cpu::source_is_8bit_register(opcode)
                && Cpu::destination_is_8bit_register(opcode) =>
            {
                self.ld_r8_r8(opcode)
            }
            v if (v & 0b11000111) == 0b00000110 && Cpu::destination_is_8bit_register(opcode) => {
                self.ld_r8_imm8(opcode)
            }
            0b00110110 => self.ld_hl_imm8(),
            0b00001010 => self.ld_a_bc(),
            0b00011010 => self.ld_a_de(),
            0b11110010 => self.ld_a_c(),
            0b11100010 => self.ld_c_a(),
            0b11110000 => self.ld_a_imm8(),
            0b11100000 => self.ld_imm8_a(),
            0b11111010 => self.ld_a_imm16(),
            0b11101010 => self.ld_imm16_a(),
            0b00101010 => self.ld_a_hli(),
            0b00111010 => self.ld_a_hld(),
            0b00000010 => self.ld_bc_a(),
            0b00010010 => self.ld_de_a(),
            0b00100010 => self.ld_hli_a(),
            0b00110010 => self.ld_hld_a(),

            // 8-Bit Arithmetic and Logical Operation Instructions
            v if (v >> 3) == 0b10000 && Cpu::source_is_8bit_register(opcode) => {
                self.add_a_r(opcode)
            }
            0b11000110 => self.add_a_n(),
            0b10000110 => self.add_a_hl(),
            v if (v >> 3) == 0b10001 && Cpu::source_is_8bit_register(opcode) => {
                self.adc_a_r(opcode)
            }
            0b11001110 => self.adc_a_imm8(),
            0b10001110 => self.adc_a_hl(),
            v if (v >> 3) == 0b10010 && Cpu::source_is_8bit_register(opcode) => {
                self.sub_a_r(opcode)
            }
            0b11010110 => self.sub_a_imm8(),
            0b10010110 => self.sub_a_hl(),
            v if (v >> 3) == 0b10011 && Cpu::source_is_8bit_register(opcode) => {
                self.sbc_a_r(opcode)
            }
            0b11011110 => self.sbc_a_imm8(),
            0b10011110 => self.sbc_a_hl(),
            v if (v >> 3) == 0b10100 && Cpu::source_is_8bit_register(opcode) => {
                self.and_a_r(opcode)
            }
            0b11100110 => self.and_a_imm8(),
            0b10100110 => self.and_a_hl(),
            v if (v >> 3) == 0b10110 && Cpu::source_is_8bit_register(opcode) => {
                self.or_a_r(opcode)
            }
            0b11110110 => self.or_a_imm8(),
            0b10110110 => self.or_a_hl(),
            v if (v >> 3) == 0b10101 && Cpu::source_is_8bit_register(opcode) => {
                self.xor_a_r(opcode)
            }
            0b11101110 => self.xor_a_imm8(),
            0b10101110 => self.xor_a_hl(),
            v if (v >> 3) == 0b10111 && Cpu::source_is_8bit_register(opcode) => {
                self.cp_a_r(opcode)
            }
            0b11111110 => self.cp_a_imm8(),
            0b10111110 => self.cp_a_hl(),
            v if (v & 0b11000111) == 0b00000100 && Cpu::destination_is_8bit_register(opcode) => {
                self.inc_r(opcode)
            },
            0b00110100 => self.inc_hl(),
            v if (v & 0b11000111) == 0b00000101 && Cpu::destination_is_8bit_register(opcode) => {
                self.dec_r(opcode)
            },
            0b00110101 => self.dec_hl(),
            v if (v & 0b11001111) == 0b00000001 && Cpu::destination_is_16bit_register(opcode) => {
                self.ld_r16_imm16(opcode)
            },
            0b11111001 => self.ld_sp_hl(),
            v if (v & 0b11001111) == 0b11000101 && Cpu::destination_is_16bit_register(opcode) => {
                self.push_r16_onto_memory_stack(opcode)
            },
            _ => return,
        }
    }

    /// No Operation - Do nothing for one CPU cycle.
    fn nop() {
        return;
    }

    /// Load the 8-bit immediate value into the specified 8-bit register.
    fn ld_r8_imm8(&mut self, opcode: u8) {
        let destination = Cpu::get_destination_register(opcode);
        let imm8 = self.get_imm8();
        self.registers.set_8bit_register_value(destination, imm8);
        self.registers.increment_pc();
    }

    fn ld_r8_r8(&mut self, opcode: u8) {
        let destination = Cpu::get_destination_register(opcode);
        let source = Cpu::get_source_register(opcode);

        if destination == source {
            return; // No operation needed if both registers are the same
        }

        let value = self.registers.get_8bit_register_value(source);
        self.registers.set_8bit_register_value(destination, value);
    }

    /// Load the contents of register HL into 8-bit register.
    fn ld_r8_hl(&mut self, opcode: u8) {
        let destination = Cpu::get_destination_register(opcode);
        let value = self.get_memory_value_at_hl();
        self.registers.set_8bit_register_value(destination, value);
    }

    /// Stores the contents of register r in memory specified by register pair HL.
    fn ld_hl_r8(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        let hl = self.registers.get_hl();
        self.memory_bus.write_byte(hl, value);
    }

    /// Loads 8-bit immediate data n into memory specified by register pair HL.
    fn ld_hl_imm8(&mut self) {
        let imm8 = self.get_imm8();
        let hl = self.registers.get_hl();
        self.memory_bus.write_byte(hl, imm8);
        self.registers.increment_pc();
    }

    /// Loads the contents specified by the contents of register pair BC into register A.
    fn ld_a_bc(&mut self) {
        let bc = self.registers.get_bc();
        let value = self.memory_bus.read_byte(bc);
        self.registers.a = value;
    }

    /// Loads the contents specified by the contents of register pair DE into register A.
    fn ld_a_de(&mut self) {
        let de = self.registers.get_de();
        let value = self.memory_bus.read_byte(de);
        self.registers.a = value;
    }

    /// Loads into register A the contents of the internal RAM, port register, or mode register at the address in
    /// the range FF00h-FFFFh specified by register C.
    fn ld_a_c(&mut self) {
        let c_register_value = self.registers.c as u16;
        let ram_address = Self::START_ADDRESS_FOR_LOAD_INSTRUCTIONS + c_register_value;
        let value = self.memory_bus.read_byte(ram_address);
        self.registers.a = value;
    }

    /// Loads the contents of register A in the internal RAM, port register, or mode register at the address in the
    /// range FF00h-FFFFh specified by register C.
    fn ld_c_a(&mut self) {
        let c_register_value = self.registers.c as u16;
        let ram_address = Self::START_ADDRESS_FOR_LOAD_INSTRUCTIONS + c_register_value;
        self.memory_bus.write_byte(ram_address, self.registers.a);
    }

    /// Loads into register A the contents of the internal RAM, port register, or mode register at the address in the range FF00h-FFFFh
    /// specified by the 8-bit immediate operand n.
    /// Note, however, that a 16-bit address should be specified for the mnemonic portion of n, because only the lower-order 8 bits are
    /// automatically reflected in the machine language.
    fn ld_a_imm8(&mut self) {
        let imm8 = self.get_imm8() as u16;
        let address_to_read_from = Self::START_ADDRESS_FOR_LOAD_INSTRUCTIONS + imm8;
        let value = self.memory_bus.read_byte(address_to_read_from);
        self.registers.a = value;
        self.registers.increment_pc();
    }

    /// Loads the contents of register A to the internal RAM, port register, or mode register at the address in the range FF00h-FFFFh
    /// specified by the 8-bit immediate operand n.
    /// Note, however, that a 16-bit address should be specified for the mnemonic portion of n, because only the
    /// lower-order 8 bits are automatically reflected in the machine language.
    fn ld_imm8_a(&mut self) {
        let imm8 = self.get_imm8() as u16;
        let address_to_write = Self::START_ADDRESS_FOR_LOAD_INSTRUCTIONS + imm8;
        self.memory_bus
            .write_byte(address_to_write, self.registers.a);
        self.registers.increment_pc();
    }

    /// Loads into register A the contents of the internal RAM or register specified by 16-bit immediate operand nn.
    fn ld_a_imm16(&mut self) {
        let imm16 = self.get_imm16();
        let value = self.memory_bus.read_byte(imm16);
        self.registers.a = value;
        self.registers.increment_pc_twice();
    }

    /// Loads the contents of register A to the internal RAM or register specified by 16-bit immediate operand nn.
    fn ld_imm16_a(&mut self) {
        let imm16 = self.get_imm16();
        self.memory_bus.write_byte(imm16, self.registers.a);
        self.registers.increment_pc_twice();
    }

    /// Loads in register A the contents of memory specified by the contents of register pair HL and simultaneously increments the contents of HL.
    fn ld_a_hli(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.registers.a = value;
        self.registers.increment_hl();
    }

    /// Loads in register A the contents of memory specified by the contents of register pair HL and simultaneously decrements the contents of HL.
    /// Example: When HL = 8A5Ch and (8A5Ch) = 3Ch,
    /// LD A, (HLD) ; A ← 3Ch, HL ← 8A5Bh
    fn ld_a_hld(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.registers.a = value;
        self.registers.decrement_hl();
    }

    /// Stores the contents of register A in the memory specified by register pair BC.
    /// Example: When BC = 205Fh and A = 3Fh,
    /// LD (BC) , A ; (205Fh) ← 3Fh
    fn ld_bc_a(&mut self) {
        let bc = self.registers.get_bc();
        self.memory_bus.write_byte(bc, self.registers.a);
    }

    /// Stores the contents of register A in the memory specified by register pair DE.
    /// Example: When DE = 205Ch and A = 00h,
    /// LD (DE) , A ; (205Ch) ← 00h
    fn ld_de_a(&mut self) {
        let de = self.registers.get_de();
        self.memory_bus.write_byte(de, self.registers.a);
    }

    /// Stores the contents of register A in the memory specified by register pair HL and simultaneously increments the contents of HL.
    /// Example: When HL = FFFFh and A = 56h,
    /// LD (HLI), A ; (0xFFFF) ← 56h, HL = 0000h
    fn ld_hli_a(&mut self) {
        let hl = self.registers.get_hl();
        self.memory_bus.write_byte(hl, self.registers.a);
        self.registers.increment_hl();
    }

    /// Stores the contents of register A in the memory specified by register pair HL and simultaneously decrements the contents of HL.
    /// Example: HL = 4000h and A = 5h,
    /// LD (HLD), A ; (4000h) ← 5h, HL = 3FFFh
    fn ld_hld_a(&mut self) {
        let hl = self.registers.get_hl();
        self.memory_bus.write_byte(hl, self.registers.a);
        self.registers.decrement_hl();
    }

    /// Adds the contents of register r to those of register A and stores the results in register A.
    /// Flag Z: Set if the result is 0; otherwise reset.
    ///      H: Set if there is a carry from bit 3; otherwise reset.
    ///      N: Reset
    ///      CY: Set if there is a carry from bit 7; otherwise reset.
    /// Example: When A = 0x3A and B = 0xC6,
    /// ADD A, B ; A ← 0, Z ← 1, H ← 1, N ← 0, CY ← 1
    fn add_a_r(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        let (result, carry) = self.registers.a.overflowing_add(value);
        let h_flag = FlagsRegister::calculate_h_flag_on_add(self.registers.a, value);
        self.registers.a = result;
        self.flags_register.n_flag = false;
        self.flags_register.set_c_flag(carry);
        self.flags_register.set_z_flag(result);
        self.flags_register.set_h_flag(h_flag);
    }

    /// Adds 8-bit immediate operand n to the contents of register A and stores the results in register A.
    /// Example: When A = 3Ch,
    /// ADD A. FFh ; A ← 3Bh, Z ← 0, H ← 1, N ← 0, CY ← 1
    fn add_a_n(&mut self) {
        let value = self.get_imm8();
        let (result, carry) = self.registers.a.overflowing_add(value);
        let h_flag = FlagsRegister::calculate_h_flag_on_add(self.registers.a, value);

        self.registers.a = result;
        self.flags_register.n_flag = false;
        self.flags_register.set_c_flag(carry);
        self.flags_register.set_z_flag(result);
        self.flags_register.set_h_flag(h_flag);
        self.registers.increment_pc();
    }

    /// Adds the contents of memory specified by the contents of register pair HL to the contents of register A and stores the results in register A.
    /// Example: When A = 3Ch and (HL) = 12h,
    /// ADD A, (HL) ; A ← 4Eh, Z ← 0, H ← 0, N ← 0, CY ← 0
    fn add_a_hl(&mut self) {
        let value = self.get_memory_value_at_hl();
        let (result, carry) = self.registers.a.overflowing_add(value);
        let h_flag = FlagsRegister::calculate_h_flag_on_add(self.registers.a, value);

        self.registers.a = result;
        self.flags_register.n_flag = false;
        self.flags_register.set_c_flag(carry);
        self.flags_register.set_z_flag(result);
        self.flags_register.set_h_flag(h_flag);
    }

    /// Adds the contents of register r and CY to the contents of register A and stores the results in register A.
    fn adc_a_r(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        self.adc_a_value(value);
    }

    /// Adds the contents of the immediate byte and CY to the contents of register A and stores the results in register A.
    fn adc_a_imm8(&mut self) {
        self.adc_a_value(self.get_imm8());
    }

    /// Adds the contents of memory specified by the contents of register pair HL and CY to the contents of register A and stores the results in register A.
    fn adc_a_hl(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.adc_a_value(value);
    }

    /// Adds the contents of operand s and CY to the contents of register A and stores the results in register A. r, n, and (HL) are used for operand s.
    /// Examples: When A = E1h, E = 0Fh, (HL) = 1Eh, and CY = 1,
    ///           ADC A, E ; A ← F1h, Z ← 0, H ← 1, CY ← 0
    ///           ADC A, 3Bh ; A ← 1Dh, Z ← 0, H ← 0, CY ← 0
    ///           ADC A, (HL) ; A ← 00h, Z ← 1, H ← 1, CY ← 1
    fn adc_a_value(&mut self, value: u8) {
        let cy = self.flags_register.get_c_flag_u8();

        let (temp_result, temp_carry) = value.overflowing_add(cy);
        let mut h_flag: bool = FlagsRegister::calculate_h_flag_on_add(value, cy);

        let (final_result, final_carry) = self.registers.a.overflowing_add(temp_result);
        h_flag |= FlagsRegister::calculate_h_flag_on_add(self.registers.a, temp_result);

        self.registers.a = final_result;
        self.flags_register.n_flag = false;
        self.flags_register.set_c_flag(temp_carry | final_carry);
        self.flags_register.set_z_flag(final_result);
        self.flags_register.set_h_flag(h_flag);
    }
    
    /// Subtracts the contents of register r from the contents of register A and stores the results in register A.
    fn sub_a_r(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        self.sub_a_value(value);
    }
    
    /// Subtracts the 8-bit immediate operand n from the contents of register A and stores the results in register A.
    fn sub_a_imm8(&mut self) {
        let value = self.get_imm8();
        self.sub_a_value(value);
        self.registers.increment_pc();
    }
    
    /// Subtracts the contents of memory specified by the contents of register pair HL from the contents of register A and stores the results in register A.
    fn sub_a_hl(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.sub_a_value(value);
    }

    /// Subtracts the contents of operand s from the contents of register A and stores the results in register A.
    /// r, n, and (HL) are used for operand s.
    /// Flags
    ///     Z: Set if result is 0; otherwise reset.
    ///     H: Set if there is a borrow from bit 4; otherwise reset.
    ///     N: Set
    ///     CY: Set if there is a borrow; otherwise reset.
    fn sub_a_value(&mut self, value: u8) {
        let (result, _borrow) = self.registers.a.overflowing_sub(value);
        // Half-carry flag (H): Set if no borrow from bit 4
        // In subtraction, half-carry is set when the lower nibble of A is less than the lower nibble of B
        let half_carry = (self.registers.a & 0x0F) < (value & 0x0F);

        // Carry flag (C): Set if no borrow occurred (A < B)
        let carry = self.registers.a < value;

        self.registers.a = result;
        self.flags_register.n_flag = true;
        self.flags_register.set_c_flag(carry);
        self.flags_register.set_z_flag(result);
        self.flags_register.set_h_flag(half_carry);
    }

    /// Subtracts the contents of register r and CY from the contents of register A and stores the results in register A. 
    fn sbc_a_r(&mut self, opcode: u8) {
        println!("Executing SBC A, r");
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        self.sbc_a_value(value);
    }

    /// Subtracts the 8-bit immediate operand n and CY from the contents of register A and stores the results in register A.
    fn sbc_a_imm8(&mut self) {
        let value = self.get_imm8();
        self.sbc_a_value(value);
        self.registers.increment_pc();
    }

    /// Subtracts the contents of memory specified by the contents of register pair HL and CY from the contents of register A and stores the results in register A.
    fn sbc_a_hl(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.sbc_a_value(value);
    }
    
    /// Subtracts the contents of operand s and CY from the contents of register A and stores the results in register A. 
    /// r, n, and (HL) are used for operand s.
    /// Flags
    ///     Z: Set if result is 0; otherwise reset.
    ///     H: Set if there is a borrow from bit 4; otherwise reset.
    ///     N: Set
    ///     CY: Set if there is a borrow; otherwise reset.
    fn sbc_a_value(&mut self, value: u8) {
        let (mut result, _borrow) = self.registers.a.overflowing_sub(value);
        // Half-carry flag (H): Set if no borrow from bit 4
        // In subtraction, half-carry is set when the lower nibble of A is less than the lower nibble of B
        println!("A: {:02X}, value: {:02X}", self.registers.a, value);
        let mut half_carry = (self.registers.a & 0x0F) < (value & 0x0F);

        // Carry flag (C): Set if no borrow occurred (A < B)
        let mut carry = self.registers.a < value;
        
        if self.flags_register.c_flag {
            half_carry |= (result & 0x0F) < 1;
            carry |= result < 1;
            let (result_c_flag , _) = result.overflowing_sub(1);
            result = result_c_flag;
        }

        self.registers.a = result;
        self.flags_register.n_flag = true;
        self.flags_register.set_c_flag(carry);
        self.flags_register.set_z_flag(result);
        self.flags_register.set_h_flag(half_carry);
    }

    /// Takes the logical-AND for each bit of the contents of register r and register A, and stores the results in register A.
    fn and_a_r(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        self.and_a_value(value);
    }

    /// Takes the logical-AND for each bit of the contents of immediate operand and register A, and stores the results in register A.
    fn and_a_imm8(&mut self) {
        let value = self.get_imm8();
        self.and_a_value(value);
        self.registers.increment_pc();
    }

    /// Takes the logical-AND for each bit of the contents of memory specified by the contents of register pair HL and register A, and stores the results in register A.
    fn and_a_hl(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.and_a_value(value);
    }
    
    /// Takes the logical-AND for each bit of the contents of operand s and register A, and stores the results in register A.
    fn and_a_value(&mut self, value: u8) {
        self.registers.a &= value;
        self.flags_register.set_h_flag(true);
        self.flags_register.set_z_flag(self.registers.a); 
        self.flags_register.n_flag = false;
        self.flags_register.c_flag = false;
    }

    /// Takes the logical-OR for each bit of the contents of register r and register A, and stores the results in register A.
    fn or_a_r(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        self.or_a_value(value);
    }

    /// Takes the logical-OR for each bit of the contents of immediate operand and register A, and stores the results in register A.
    fn or_a_imm8(&mut self) {
        let value = self.get_imm8();
        self.or_a_value(value);
        self.registers.increment_pc();
    }

    /// Takes the logical-OR for each bit of the contents of memory specified by the contents of register pair HL and register A, and stores the results in register A.
    fn or_a_hl(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.or_a_value(value);
    }
    
    /// Takes the logical-OR for each bit of the contents of operand s and register A, and stores the results in register A.
    fn or_a_value(&mut self, value: u8) {
        self.registers.a |= value;
        self.flags_register.set_h_flag(false);
        self.flags_register.set_z_flag(self.registers.a); 
        self.flags_register.n_flag = false;
        self.flags_register.c_flag = false;
    }

    /// Takes the logical exclusive-OR for each bit of the contents of register r and register A, and stores the results in register A.
    fn xor_a_r(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        self.xor_a_value(value);
    }

    /// Takes the logical exclusive-OR for each bit of the contents of immediate operand and register A, and stores the results in register A.
    fn xor_a_imm8(&mut self) {
        let value = self.get_imm8();
        self.xor_a_value(value);
        self.registers.increment_pc();
    }

    /// Takes the logical exclusive-OR for each bit of the contents of memory specified by the contents of register pair HL and register A, and stores the results in register A.
    fn xor_a_hl(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.xor_a_value(value);
    }
    
    /// Takes the logical exclusive-OR for each bit of the contents of operand s and register A, and stores the results in register A.
    fn xor_a_value(&mut self, value: u8) {
        self.registers.a ^= value;
        self.flags_register.set_h_flag(false);
        self.flags_register.set_z_flag(self.registers.a); 
        self.flags_register.n_flag = false;
        self.flags_register.c_flag = false;
    }

    /// Compares the contents of register r and register A and sets the flag if they are equal.
    fn cp_a_r(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        self.cp_a_value(value);
    }
    
    /// Compares the contents of 8-bit immediate operand n and register A and sets the flag if they are equal.
    fn cp_a_imm8(&mut self) {
        let value = self.get_imm8();
        self.cp_a_value(value);
        self.registers.increment_pc();
    }
    
    /// Compares the contents of memory specified by the contents of register pair HL and register A and sets the flag if they are equal.
    fn cp_a_hl(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.cp_a_value(value);
    }

    /// Compares the contents of operand s and register A and sets the flag if they are equal. r, n, and (HL) are used for operand s.
    /// This is basically an A - s subtraction instruction but the results are thrown away.
    /// Flags:
    ///     Z: Set if result is 0; otherwise reset.
    ///     H: Set if there is a borrow from bit 4; otherwise reset.
    ///     N: Set
    ///     CY: Set if there is a borrow; otherwise reset.
    fn cp_a_value(&mut self, value: u8) {
        let (result, _borrow) = self.registers.a.overflowing_sub(value);
        let half_carry = FlagsRegister::calculate_h_flag_on_sub(self.registers.a, value);

        // Carry flag (C): Set if no borrow occurred (A < B)
        let carry = self.registers.a < value;

        self.flags_register.n_flag = true;
        self.flags_register.set_c_flag(carry);
        self.flags_register.set_z_flag(result);
        self.flags_register.set_h_flag(half_carry);
    }
    
    /// Increments the contents of register r by 1.
    fn inc_r(&mut self, opcode: u8) {
        let destination_register = Cpu::get_destination_register(opcode);
        let value = self.registers.get_8bit_register_value(destination_register);

        let (result, _carry) = value.overflowing_add(1);
        let h_flag = FlagsRegister::calculate_h_flag_on_add(value, 1);
        self.flags_register.n_flag = false;
        self.flags_register.set_z_flag(result);
        self.flags_register.set_h_flag(h_flag);

        self.registers.set_8bit_register_value(destination_register, result);
    }

    /// Increments by 1 the contents of memory specified by register pair HL.
    fn inc_hl(&mut self) {
        let value = self.get_memory_value_at_hl();

        let (result, _carry) = value.overflowing_add(1);
        let h_flag = FlagsRegister::calculate_h_flag_on_add(value, 1);
        self.flags_register.n_flag = false;
        self.flags_register.set_z_flag(result);
        self.flags_register.set_h_flag(h_flag);

        self.write_memory_value_at_hl(result);
    }

    /// Subtract 1 from the contents of register r.
    fn dec_r(&mut self, opcode: u8) {
        let destination_register = Cpu::get_destination_register(opcode);
        let value = self.registers.get_8bit_register_value(destination_register);

        let (result, _carry) = value.overflowing_sub(1);
        let h_flag = FlagsRegister::calculate_h_flag_on_sub(value, 1);
        self.flags_register.n_flag = true;
        self.flags_register.set_z_flag(result);
        self.flags_register.set_h_flag(h_flag);

        self.registers.set_8bit_register_value(destination_register, result);
    }

    /// Decrements by 1 the contents of memory specified by register pair HL.
    fn dec_hl(&mut self) {
        let value = self.get_memory_value_at_hl();

        let (result, _carry) = value.overflowing_sub(1);
        let h_flag = FlagsRegister::calculate_h_flag_on_sub(value, 1);
        self.flags_register.n_flag = true;
        self.flags_register.set_z_flag(result);
        self.flags_register.set_h_flag(h_flag);

        self.write_memory_value_at_hl(result);
    }
    
    /// Loads 2 bytes of immediate data to 16-bit register, where it can be the registers BC, DE, HL or SP.
    /// BC = 0b00, DE = 0b01, HL = 0b10, SP = 0b11
    fn ld_r16_imm16(&mut self, opcode: u8) {
        let destination_register = Cpu::get_16bit_destination_register(opcode);
        let value = self.get_imm16();

        match destination_register {
            0b00 => self.registers.set_bc(value),
            0b01 => self.registers.set_de(value),
            0b10 => self.registers.set_hl(value),
            0b11 => self.registers.sp = value,
            _ => (),
        }
    }
    
    /// Loads the contents of register pair HL in stack pointer SP.
    fn ld_sp_hl(&mut self) {
        self.registers.sp = self.registers.get_hl();
    }
    
    /// Pushes the contents of register pair qq (a 16-bit register) onto the memory stack. First 1 is subtracted from SP and the 
    /// contents of the higher portion of qq are placed on the stack. The contents of the lower portion of qq are 
    /// then placed on the stack. The contents of SP are automatically decremented by 2.
    /// FF80h-FFFEh: Can be used as CPU work RAM and/or stack RAM.
    fn push_r16_onto_memory_stack(&mut self, opcode: u8) {
        let source_register = Cpu::get_16bit_destination_register(opcode);
        let value = match source_register {
            0b00 => self.registers.get_bc(),
            0b01 => self.registers.get_de(),
            0b10 => self.registers.get_hl(),
            0b11 => self.registers.get_af(),
            _ => 0,
        };

        let high_byte = (value >> 8) as u8;
        let low_byte = (value & 0x00FF) as u8;

        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.memory_bus
            .write_byte(self.registers.sp, high_byte);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.memory_bus
            .write_byte(self.registers.sp, low_byte);
    }

    /// Get the 8-bit immediate value
    fn get_imm8(&self) -> u8 {
        let imm8 = self.memory_bus.read_byte(self.registers.pc);
        imm8
    }

    /// Get the following two bytes, in little-endian order
    fn get_imm16(&self) -> u16 {
        let lsb = self.memory_bus.read_byte(self.registers.pc) as u16;
        let msb = self.memory_bus.read_byte(self.registers.pc + 1) as u16;
        (msb << 8) | lsb
    }

    fn halt(&self) {
        todo!("Implement HALT instruction")
    }

    /// Get the destination register from the opcode.
    /// The destination register is specified by bits 3 to 5 of the opcode.
    fn get_destination_register(opcode: u8) -> u8 {
        (opcode & 0b00111000) >> 3
    }

    /// Get the source register from the opcode.
    /// The source register is specified by bits 0 to 2 of the opcode.
    fn get_source_register(opcode: u8) -> u8 {
        opcode & 0b00000111
    }

    /// Get the 16-bit destination register from the opcode.
    /// The destination register is specified by bits 4 and 5 of the opcode.
    fn get_16bit_destination_register(opcode: u8) -> u8 {
        (opcode & 0b00110000) >> 4
    }

    /// Check if the destination register is an 8-bit register.
    fn destination_is_8bit_register(opcode: u8) -> bool {
        let destination_register = Cpu::get_destination_register(opcode);
        Self::EIGHT_BIT_REGISTERS.contains(&destination_register)
    }

    /// Check if the destination register is a 16-bit register.
    fn destination_is_16bit_register(opcode: u8) -> bool {
        let destination_register = Cpu::get_16bit_destination_register(opcode);
        Self::SIXTEEN_BIT_REGISTERS.contains(&destination_register)
    }

    /// Check if the source register is an 8-bit register.
    fn source_is_8bit_register(opcode: u8) -> bool {
        let source_register = Cpu::get_source_register(opcode);
        Self::EIGHT_BIT_REGISTERS.contains(&source_register)
    }

    /// Reads the content of memory specified by the contents of register pair HL
    fn get_memory_value_at_hl(&mut self) -> u8 {
        let hl = self.registers.get_hl();
        self.memory_bus.read_byte(hl)
    }

    /// Writes a value in the content of memory specified by the contents of register pair HL
    fn write_memory_value_at_hl(&mut self, value: u8) {
        let hl = self.registers.get_hl();
        self.memory_bus.write_byte(hl, value);
    }
}
