use crate::gameboy_core::{
    constants::{
        EIGHT_BIT_REGISTERS, SCREEN_HEIGHT, SCREEN_WIDTH, SIXTEEN_BIT_REGISTERS,
        START_ADDRESS_FOR_LOAD_INSTRUCTIONS,
    },
    cpu_components::{CpuRegisters, FlagsRegister, MemoryBus},
    cpu_utils,
    ppu::{Ppu},
};

pub struct Cpu {
    pub registers: CpuRegisters,
    pub flags_register: FlagsRegister,
    pub memory_bus: MemoryBus,
    pub is_debug_mode: bool,
    pub ppu: Ppu,
    pub cycles: u64,
    pub ime: bool,
    pub di_instruction_pending: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Self {
            registers: CpuRegisters::new(),
            flags_register: FlagsRegister::new(),
            memory_bus: MemoryBus::new(),
            is_debug_mode: false,
            ppu: Ppu::new(),
            cycles: 0,
            ime: false,
            di_instruction_pending: false,
        };
        cpu.initialize_memory_registers();

        cpu
    }

    /// Initialize the Registers stored in RAM to default values as per Gameboy hardware specs.
    pub fn initialize_memory_registers(&mut self) {
        // Initialize LCDC register to enable LCD and set background tile map area to 0x9800-0x9BFF
        self.memory_bus.set_lcdc_register(0x91); // 10010001: LCD enabled, BG enabled, Tile data area at 0x8000, BG tile map area at 0x9800
        self.memory_bus.set_bgp_register(0xFC); // Set BGP register to a default value

        // Other registers can be initialized here as needed
    }

    /// Start the emulator with the provided ROM binary data.
    pub fn start(rom_binary: Vec<u8>, is_debug_mode: bool) -> Self {
        let mut cpu = Self::new();
        cpu.load_rom(rom_binary);
        cpu.is_debug_mode = is_debug_mode;
        return cpu;
    }

    /// Perform a single CPU tick: fetch, decode, and execute one instruction.
    pub fn tick(&mut self) {
        let opcode = self.fetch_opcode();

        cpu_utils::print_state_if_debug_mode(self, opcode);

        self.registers.increment_pc();
        self.execute(opcode);

        self.disable_ime_if_di_instruction_pending(opcode);
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
            0x00 | 0xE3 | 0xED => Cpu::nop(),        // NOP
            0b01110110 => self.halt(), // HALT

            // 8-Bit Transfer and Input/Output Instructions
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
            v if (v >> 3) == 0b10110 && Cpu::source_is_8bit_register(opcode) => self.or_a_r(opcode),
            0b11110110 => self.or_a_imm8(),
            0b10110110 => self.or_a_hl(),
            v if (v >> 3) == 0b10101 && Cpu::source_is_8bit_register(opcode) => {
                self.xor_a_r(opcode)
            }
            0b11101110 => self.xor_a_imm8(),
            0b10101110 => self.xor_a_hl(),
            v if (v >> 3) == 0b10111 && Cpu::source_is_8bit_register(opcode) => self.cp_a_r(opcode),
            0b11111110 => self.cp_a_imm8(),
            0b10111110 => self.cp_a_hl(),
            v if (v & 0b11000111) == 0b00000100 && Cpu::destination_is_8bit_register(opcode) => {
                self.inc_r(opcode)
            }
            0b00110100 => self.inc_hl(),
            v if (v & 0b11000111) == 0b00000101 && Cpu::destination_is_8bit_register(opcode) => {
                self.dec_r(opcode)
            }
            0b00110101 => self.dec_hl(),

            // 16-Bit Transfer Instructions
            v if (v & 0b11001111) == 0b00000001 && Cpu::destination_is_16bit_register(opcode) => {
                self.ld_r16_imm16(opcode)
            }
            0b11111001 => self.ld_sp_hl(),
            v if (v & 0b11001111) == 0b11000101 && Cpu::destination_is_16bit_register(opcode) => {
                self.push_r16_onto_memory_stack(opcode)
            }
            v if (v & 0b11001111) == 0b11000001 && Cpu::destination_is_16bit_register(opcode) => {
                self.pop_r16_from_memory_stack(opcode)
            }
            0b11111000 => self.ld_hl_sp_imm8(),
            0b00001000 => self.ld_imm16_sp(),

            // 16-Bit Arithmetic Operation Instructions
            v if (v & 0b11001111) == 0b00001001 && Cpu::destination_is_16bit_register(opcode) => {
                self.add_hl_r16(opcode)
            }
            0b11101000 => self.add_sp_imm8(),
            v if (v & 0b11001111) == 0b00000011 && Cpu::destination_is_16bit_register(opcode) => {
                self.inc_r16(opcode)
            }
            v if (v & 0b11001111) == 0b00001011 && Cpu::destination_is_16bit_register(opcode) => {
                self.dec_r16(opcode)
            }

            // Rotate Shift Instructions
            0b00000111 => self.rlca(),
            0b00010111 => self.rla(),
            0b00001111 => self.rrca(),
            0b00011111 => self.rra(),

            // Bit Operations

            // Jump Instructions
            0b00011000 => self.jr_imm8(),
            0b11000011 => self.jp_imm16(),

            // Call and Returns Instructions
            0b11001001 => self.ret(),
            v if (v & 0b11000111) == 0b11000100 => self.call_cc_imm16(opcode),
            v if (v & 0b11000111) == 0b11000111 => self.rst(opcode),

            // CB prefix instructions
            0xCB => self.execute_cb_prefix_instructions(),

            // General-Purpose Arithmetic Operations and CPU Control Instructions
            0xF3 => self.di(),
            0x3F => self.ccf(),

            _ => {
                println!(
                    "*** Unimplemented opcode: 0x{:02X} - bin: 0b{:08b} ***",
                    opcode, opcode
                );
                return;
            }
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
        let ram_address = START_ADDRESS_FOR_LOAD_INSTRUCTIONS + c_register_value;
        let value = self.memory_bus.read_byte(ram_address);
        self.registers.a = value;
    }

    /// Loads the contents of register A in the internal RAM, port register, or mode register at the address in the
    /// range FF00h-FFFFh specified by register C.
    fn ld_c_a(&mut self) {
        let c_register_value = self.registers.c as u16;
        let ram_address = START_ADDRESS_FOR_LOAD_INSTRUCTIONS + c_register_value;
        self.memory_bus.write_byte(ram_address, self.registers.a);
    }

    /// Loads into register A the contents of the internal RAM, port register, or mode register at the address in the range FF00h-FFFFh
    /// specified by the 8-bit immediate operand n.
    /// Note, however, that a 16-bit address should be specified for the mnemonic portion of n, because only the lower-order 8 bits are
    /// automatically reflected in the machine language.
    fn ld_a_imm8(&mut self) {
        let imm8 = self.get_imm8() as u16;
        let address_to_read_from = START_ADDRESS_FOR_LOAD_INSTRUCTIONS + imm8;
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
        let address_to_write = START_ADDRESS_FOR_LOAD_INSTRUCTIONS + imm8;
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
        self.flags_register.n = false;
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
        self.flags_register.n = false;
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
        self.flags_register.n = false;
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
        self.flags_register.n = false;
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
        self.flags_register.n = true;
        self.flags_register.set_c_flag(carry);
        self.flags_register.set_z_flag(result);
        self.flags_register.set_h_flag(half_carry);
    }

    /// Subtracts the contents of register r and CY from the contents of register A and stores the results in register A.
    fn sbc_a_r(&mut self, opcode: u8) {
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
        let mut half_carry = (self.registers.a & 0x0F) < (value & 0x0F);

        // Carry flag (C): Set if no borrow occurred (A < B)
        let mut carry = self.registers.a < value;

        if self.flags_register.c {
            half_carry |= (result & 0x0F) < 1;
            carry |= result < 1;
            let (result_c_flag, _) = result.overflowing_sub(1);
            result = result_c_flag;
        }

        self.registers.a = result;
        self.flags_register.n = true;
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
        self.flags_register.n = false;
        self.flags_register.c = false;
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
        self.flags_register.n = false;
        self.flags_register.c = false;
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
        self.flags_register.n = false;
        self.flags_register.c = false;
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

        self.flags_register.n = true;
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
        self.flags_register.n = false;
        self.flags_register.set_z_flag(result);
        self.flags_register.set_h_flag(h_flag);

        self.registers
            .set_8bit_register_value(destination_register, result);
    }

    /// Increments by 1 the contents of memory specified by register pair HL.
    fn inc_hl(&mut self) {
        let value = self.get_memory_value_at_hl();

        let (result, _carry) = value.overflowing_add(1);
        let h_flag = FlagsRegister::calculate_h_flag_on_add(value, 1);
        self.flags_register.n = false;
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
        self.flags_register.n = true;
        self.flags_register.set_z_flag(result);
        self.flags_register.set_h_flag(h_flag);

        self.registers
            .set_8bit_register_value(destination_register, result);
    }

    /// Decrements by 1 the contents of memory specified by register pair HL.
    fn dec_hl(&mut self) {
        let value = self.get_memory_value_at_hl();

        let (result, _carry) = value.overflowing_sub(1);
        let h_flag = FlagsRegister::calculate_h_flag_on_sub(value, 1);
        self.flags_register.n = true;
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

        self.push_value_to_sp(value);
    }

    /// Pops contents from the memory stack and into register pair qq.
    /// First the contents of memory specified by the contents of SP are loaded in the lower portion of qq.
    /// Next, the contents of SP are incremented by 1 and the contents of the memory they specify are loaded in the upper portion of qq.
    /// The contents of SP are automatically incremented by 2.
    fn pop_r16_from_memory_stack(&mut self, opcode: u8) {
        let low_byte = self.memory_bus.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let high_byte = self.memory_bus.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);

        let value = ((high_byte as u16) << 8) | (low_byte as u16);

        let destination_register = Cpu::get_16bit_destination_register(opcode);
        match destination_register {
            0b00 => self.registers.set_bc(value),
            0b01 => self.registers.set_de(value),
            0b10 => self.registers.set_hl(value),
            0b11 => self.registers.a = high_byte, // Lower nibble of F is always 0
            _ => (),
        }
    }

    /// Adds the signed 8-bit immediate value to the stack pointer SP and stores the result in HL.
    fn ld_hl_sp_imm8(&mut self) {
        let imm8 = self.get_imm8() as u16;
        let (result, carry) = self.registers.sp.overflowing_add(imm8);
        let h_flag = FlagsRegister::calculate_h_flag_on_add_u16_numbers(self.registers.sp, imm8);
        self.registers.set_hl(result);
        self.flags_register.n = false;
        self.flags_register.z = false;
        self.flags_register.set_c_flag(carry);
        self.flags_register.set_h_flag(h_flag);
    }

    /// Stores the lower byte of SP at address nn specified by the 16-bit immediate operand nn and the upper byte of SP at address nn + 1.
    fn ld_imm16_sp(&mut self) {
        let imm16 = self.get_imm16();
        let sp_lower_byte = (self.registers.sp & 0b011111111) as u8;
        self.memory_bus.write_byte(imm16, sp_lower_byte);

        let sp_higher_byte = (self.registers.sp >> 8) as u8;
        self.memory_bus.write_byte(imm16 + 1, sp_higher_byte);
    }

    /// Adds the contents of a 16-bit register to the contents of register pair HL and stores the results in HL.
    /// The 16-bit register can be BC, DE, HL or SP.
    fn add_hl_r16(&mut self, opcode: u8) {
        let source_register = Cpu::get_16bit_destination_register(opcode);
        let value = match source_register {
            0b00 => self.registers.get_bc(),
            0b01 => self.registers.get_de(),
            0b10 => self.registers.get_hl(),
            0b11 => self.registers.sp,
            _ => 0,
        };

        let (result, carry) = self.registers.get_hl().overflowing_add(value);
        let h_flag =
            FlagsRegister::calculate_h_flag_on_add_u16_numbers(self.registers.get_hl(), value);

        self.registers.set_hl(result);
        self.flags_register.n = false;
        self.flags_register.set_c_flag(carry);
        self.flags_register.set_h_flag(h_flag);
    }

    /// Adds the signed 8-bit immediate value to the stack pointer SP and stores the result in SP.
    fn add_sp_imm8(&mut self) {
        let imm8 = self.get_imm8() as u16;
        let (result, carry) = self.registers.sp.overflowing_add(imm8);
        let h_flag = FlagsRegister::calculate_h_flag_on_add_u16_numbers(self.registers.sp, imm8);

        self.registers.sp = result;
        self.flags_register.n = false;
        self.flags_register.z = false;
        self.flags_register.set_c_flag(carry);
        self.flags_register.set_h_flag(h_flag);
        self.registers.increment_pc();
    }

    /// Increments the contents of a 16-bit register by 1. The 16-bit register can be BC, DE, HL or SP.
    fn inc_r16(&mut self, opcode: u8) {
        let source_register = Cpu::get_16bit_destination_register(opcode);
        let value = match source_register {
            0b00 => self.registers.get_bc(),
            0b01 => self.registers.get_de(),
            0b10 => self.registers.get_hl(),
            0b11 => self.registers.sp,
            _ => 0,
        };

        let (result, _carry) = value.overflowing_add(1);

        match source_register {
            0b00 => self.registers.set_bc(result),
            0b01 => self.registers.set_de(result),
            0b10 => self.registers.set_hl(result),
            0b11 => self.registers.sp = result,
            _ => (),
        }
    }

    /// Decrements the contents of a 16-bit register by 1. The 16-bit register can be BC, DE, HL or SP.
    fn dec_r16(&mut self, opcode: u8) {
        let source_register = Cpu::get_16bit_destination_register(opcode);
        let value = match source_register {
            0b00 => self.registers.get_bc(),
            0b01 => self.registers.get_de(),
            0b10 => self.registers.get_hl(),
            0b11 => self.registers.sp,
            _ => 0,
        };

        let (result, _carry) = value.overflowing_sub(1);

        match source_register {
            0b00 => self.registers.set_bc(result),
            0b01 => self.registers.set_de(result),
            0b10 => self.registers.set_hl(result),
            0b11 => self.registers.sp = result,
            _ => (),
        }
    }

    /// Loads the 16-bit immediate value to the program counter (PC).
    fn jp_imm16(&mut self) {
        let imm16 = self.get_imm16();
        self.registers.pc = imm16;
        self.registers.increment_pc_twice();
    }

    /// Pops from the memory stack the PC value pushed when the subroutine was called, returning control to the source program.
    /// In this case, the contents of the address specified by the SP are loaded in the lower-order byte of the PC,
    /// and the content of the SP is incremented by 1. The contents of the address specified by the new SP
    /// value are then loaded in the higher-order byte of the PC, and the SP is again incremented by 1. (The
    /// value of SP is 2 larger than before instruction execution.)
    fn ret(&mut self) {
        let low_byte_pc = self.memory_bus.read_byte(self.registers.sp) as u16;

        self.registers.increment_sp();
        let high_byte_pc = self.memory_bus.read_byte(self.registers.sp) as u16;

        self.registers.increment_sp();

        self.registers.pc = (high_byte_pc << 8) | low_byte_pc;
    }

    /// Rotates the contents of register A to the left.
    /// That is, the contents of bit 0 are copied to bit 1 and the previous contents of bit 1 (the contents before the copy operation)
    /// are copied to bit 2. The same operation is repeated in sequence for the rest of the register.
    /// The contents of bit 7 are placed in CY.
    fn rlca(&mut self) {
        let rotated_value = self.rotate_left_and_update_flags(self.registers.a);
        self.registers.a = rotated_value;
    }

    /// Rotates a 8-bit value to the left, updating the CPU flags accordingly.
    /// The contents of bit 7 are placed in CY.
    /// Z flag is set if the result is 0; and flags N and H are reset.
    /// Returns the rotated value.
    fn rotate_left_and_update_flags(&mut self, mut value: u8) -> u8 {
        let bit7 = value >> 7;
        value <<= 1;
        self.flags_register.set_c_flag(bit7 == 1);
        self.flags_register.set_z_flag(value);
        self.flags_register.n = false;
        self.flags_register.set_h_flag(false);

        value
    }

    /// Rotates a 16-bit value to the left, updating the CPU flags accordingly.
    /// The contents of bit 7 are placed in CY.
    /// Z flag is set if the result is 0; and flags N and H are reset.
    /// Returns the rotated value.
    fn rotate_left_and_update_flags_u16(&mut self, mut value: u16) -> u16 {
        let bit7 = value >> 7;
        value <<= 1;
        self.flags_register.set_c_flag(bit7 == 1);
        self.flags_register.set_z_flag_u16(value);
        self.flags_register.n = false;
        self.flags_register.set_h_flag(false);

        value
    }

    /// Rotates a 8-bit value to the right, updating the CPU flags accordingly.
    /// The contents of bit 0 are placed in CY.
    /// Z flag is set if the result is 0; and flags N and H are reset.
    /// Returns the rotated value.
    fn rotate_right_and_update_flags(&mut self, mut value: u8) -> u8 {
        let bit0 = value & 0b00000001;
        value >>= 1;
        self.flags_register.set_c_flag(bit0 == 1);
        self.flags_register.set_z_flag(value);
        self.flags_register.n = false;
        self.flags_register.set_h_flag(false);

        value
    }

    /// Rotate the contents of register A to the left, through the carry (CY) flag. That is, the contents of bit 0 are copied to bit 1,
    /// and the previous contents of bit 1 (before the copy operation) are copied to bit 2. The same operation is repeated in sequence for the rest
    /// of the register.
    /// The previous contents of the carry flag are copied to bit 0.
    fn rla(&mut self) {
        let c_flag = self.flags_register.c;
        let mut rotated_value = self.rotate_left_and_update_flags(self.registers.a);

        if c_flag {
            rotated_value |= 0b00000001;
        }

        self.registers.a = rotated_value;
    }

    /// Rotate the contents of register A to the right. That is, the contents of bit 7 are copied to bit 6 and the previous contents of bit 6
    /// (the contents before the copy operation) are copied to bit 5. The same operation is repeated in sequence for the rest of the register.
    /// The contents of bit 0 are placed in CY.
    fn rrca(&mut self) {
        let rotated_value = self.rotate_right_and_update_flags(self.registers.a);
        self.registers.a = rotated_value;
    }

    /// Rotate the contents of register A to the right, through the carry (CY) flag.
    /// That is, the contents of bit 7 are copied to bit 6, and the previous contents of bit 6 (before the copy) are copied to bit 5.
    /// The same operation is repeated in sequence for the rest of the register.
    /// The previous contents of the carry flag are copied to bit 7.
    fn rra(&mut self) {
        let c_flag = self.flags_register.c;
        let mut rotated_value = self.rotate_right_and_update_flags(self.registers.a);

        if c_flag {
            rotated_value |= 0b10000000;
        }

        self.registers.a = rotated_value;
    }

    fn execute_cb_prefix_instructions(&mut self) {
        let cb_opcode = self.fetch_opcode();
        self.registers.increment_pc();

        match cb_opcode {
            v if (v & 0b11111000) == 0b00000000 && Cpu::source_is_8bit_register(cb_opcode) => {
                self.rlc_r8(cb_opcode)
            }
            0b00000110 => self.rlc_hl(),
            _ => {
                println!(
                    "*** Unimplemented CB prefix opcode: 0x{:02X} - bin: 0b{:08b} ***",
                    cb_opcode, cb_opcode
                );
                return;
            }
        }
    }

    /// Rotates the contents of a 8-bit register to the left.
    fn rlc_r8(&mut self, cb_opcode: u8) {
        let register = Cpu::get_source_register(cb_opcode);
        let value = self.registers.get_8bit_register_value(register);

        let rotated_value = self.rotate_left_and_update_flags(value);
        self.registers
            .set_8bit_register_value(register, rotated_value);
    }

    /// Rotates the contents of memory specified by register pair HL to the left.
    fn rlc_hl(&mut self) {
        let hl = self.registers.get_hl();
        let value = self.memory_bus.read_byte(hl);
        let rotated_value = self.rotate_left_and_update_flags(value);
        self.memory_bus.write_byte(hl, rotated_value);
    }

    /// Jumps to the address by adding the signed 8-bit immediate value to the PC.
    /// The jump range is -128 to +127 bytes from the current position.
    /// The below logic uses 2's complement to handle negative offsets. When a number is parsed to i8 or i16, Rust automatically
    /// interprets it as a signed number in 2's complement form.
    /// Example: 0xF6 as u8 = 246
    ///          0xF6 as i8 = -10 (two's complement interpretation).
    fn jr_imm8(&mut self) {
        // Read the signed offset (PC is already at opcode + 1)
        let imm8 = self.get_imm8() as i8; // Parse to i8 to handle
        self.registers.increment_pc(); // Move past the offset byte

        // Add the signed offset to PC
        // We need to convert i8 to i16 first to handle negative numbers correctly
        self.registers.pc = (self.registers.pc as i16).wrapping_add(imm8 as i16) as u16;

        // Cycles: 12 (3 machine cycles)
        self.cycles += 12;
    }

    /// This instruction disables interrupts but not immediately. Interrupts are disabled after instruction after DI is executed.
    fn di(&mut self) {
        self.di_instruction_pending = true;
    }

    /// Flips the carry flag CY. H and N flags are reset.
    fn ccf(&mut self) {
        self.flags_register.c = !self.flags_register.c;
        self.flags_register.h = false;
        self.flags_register.n = false;
    }

    /// If condition cc matches the flag, the PC value is pushed onto the stack and the PC is loaded with the 16-bit immediate value.
    /// Conditions:
    ///     00 - NZ (Z flag is reset)
    ///     01 - Z  (Z flag is set)
    ///     10 - NC (C flag is reset)
    ///     11 - C  (C flag is set)
    fn call_cc_imm16(&mut self, opcode: u8) {
        if self.check_cc_condition(opcode) {
            self.push_value_to_sp(self.registers.pc);
            self.registers.pc = self.get_imm16();
        }
    }

    /// Pushes a 16-bit value onto the stack. First 1 is subtracted from SP and the higher byte of the value is placed on the stack.
    /// Then, 1 is subtracted from SP again and the lower byte of the value is placed on the stack.
    /// The contents of SP are automatically decremented by 2.
    pub fn push_value_to_sp(&mut self, value: u16) {
        let high_byte = (value >> 8) as u8;
        let low_byte = (value & 0x00FF) as u8;

        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.memory_bus.write_byte(self.registers.sp, high_byte);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.memory_bus.write_byte(self.registers.sp, low_byte);
    }
    
    /// Pops a 16-bit value from the stack. First the contents of memory specified by SP are loaded into the lower byte of the value,
    /// and SP is incremented by 1. Then, the contents of memory specified by the new SP value are loaded into the higher byte of the value,
    /// and SP is incremented by 1 again. 
    /// The contents of SP are automatically incremented by 2.
    pub fn pop_value_from_sp(&mut self) -> u16 {
        let low_byte = self.memory_bus.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let high_byte = self.memory_bus.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);

        ((high_byte as u16) << 8) | (low_byte as u16)
    }

    /// Check the condition for conditional call/jump instructions based on the opcode.
    /// Returns true if the condition is met, false otherwise.
    pub(crate) fn check_cc_condition(&self, opcode: u8) -> bool {
        match (opcode & 0b00111000) >> 3 {
            0 => !self.flags_register.z,
            1 => self.flags_register.z,
            0b10 => !self.flags_register.c,
            0b11 => self.flags_register.c,
            _ => false,
        }
    }

    /// Pushes the current value of the PC to the memory stack and loads to the PC the page 0 memory addresses provided by operand t.
    /// Then next instruction is fetched from the address specified by the new content of PC.
    /// With the push, the content of the SP is decremented by 1, and the higher-order byte of the PC is loaded
    /// in the memory address specified by the new SP value. The value of the SP is then again decremented
    /// by 1, and the lower-order byte of the PC is loaded in the memory address specified by that value of the SP.
    /// The RST instruction can be used to jump to 1 of 8 addresses.
    fn rst(&mut self, opcode: u8) {
        self.push_value_to_sp(self.registers.pc);
        self.registers.pc = match (opcode & 0b00111000) >> 3 {
            0 => 0x0,
            1 => 0x0008,
            2 => 0x0010,
            3 => 0x0018,
            4 => 0x0020,
            5 => 0x0028,
            6 => 0x0030,
            7 => 0x0038,
            _ => 0x0,
        }
    }

    /// Get the 8-bit immediate value
    pub(crate) fn get_imm8(&self) -> u8 {
        let imm8 = self.memory_bus.read_byte(self.registers.pc);
        imm8
    }

    /// Get the following two bytes, in little-endian order. Little-endian means the least significant byte comes first in memory.
    pub(crate) fn get_imm16(&self) -> u16 {
        let lowest_significant_byte = self.memory_bus.read_byte(self.registers.pc) as u16;
        let most_significant_byte = self.memory_bus.read_byte(self.registers.pc + 1) as u16;
        (most_significant_byte << 8) | lowest_significant_byte
    }

    fn halt(&self) {
        todo!("Implement HALT instruction")
    }

    /// Get the destination register from the opcode.
    /// The destination register is specified by bits 3 to 5 of the opcode.
    pub(crate) fn get_destination_register(opcode: u8) -> u8 {
        (opcode & 0b00111000) >> 3
    }

    /// Get the source register from the opcode.
    /// The source register is specified by bits 0 to 2 of the opcode.
    pub(crate) fn get_source_register(opcode: u8) -> u8 {
        opcode & 0b00000111
    }

    /// Get the 16-bit destination register from the opcode.
    /// The destination register is specified by bits 4 and 5 of the opcode.
    pub(crate) fn get_16bit_destination_register(opcode: u8) -> u8 {
        (opcode & 0b00110000) >> 4
    }

    /// Check if the destination register is an 8-bit register.
    fn destination_is_8bit_register(opcode: u8) -> bool {
        let destination_register = Cpu::get_destination_register(opcode);
        EIGHT_BIT_REGISTERS.contains(&destination_register)
    }

    /// Check if the destination register is a 16-bit register.
    fn destination_is_16bit_register(opcode: u8) -> bool {
        let destination_register = Cpu::get_16bit_destination_register(opcode);
        SIXTEEN_BIT_REGISTERS.contains(&destination_register)
    }

    /// Check if the source register is an 8-bit register.
    fn source_is_8bit_register(opcode: u8) -> bool {
        let source_register = Cpu::get_source_register(opcode);
        EIGHT_BIT_REGISTERS.contains(&source_register)
    }

    /// Reads the content of memory specified by the contents of register pair HL
    pub(crate) fn get_memory_value_at_hl(&mut self) -> u8 {
        let hl = self.registers.get_hl();
        self.memory_bus.read_byte(hl)
    }

    /// Writes a value in the content of memory specified by the contents of register pair HL
    pub(crate) fn write_memory_value_at_hl(&mut self, value: u8) {
        let hl = self.registers.get_hl();
        self.memory_bus.write_byte(hl, value);
    }

    fn load_rom(&mut self, rom_binary: Vec<u8>) {
        self.memory_bus.copy_from_binary(rom_binary);
    }

    pub fn get_screen_buffer(&mut self) -> [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT] {
        self.ppu.get_screen_buffer(&mut self.memory_bus)
    }

    pub(crate) fn set_debug_mode(&mut self, value: bool) {
        self.is_debug_mode = value;
    }

    /// If DI instructions is pending it means we need to set ime to false
    fn disable_ime_if_di_instruction_pending(&mut self, opcode: u8) {
        // ensure the current opcode is to the DI instruction
        if opcode != 0xF3 && self.di_instruction_pending {
            self.set_ime(false);
        }
    }

    /// Set the IME (Interrupt Master Enable) flag
    fn set_ime(&mut self, value: bool) {
        self.ime = value;
        self.di_instruction_pending = false;
    }
}
