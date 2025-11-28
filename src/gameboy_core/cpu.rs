use crate::gameboy_core::{
    constants::{EIGHT_BIT_REGISTERS, SCREEN_HEIGHT, SCREEN_WIDTH, SIXTEEN_BIT_REGISTERS}, cpu_components::{CpuRegisters, FlagsRegister, MemoryBus}, cpu_instructions::{cpu_8bit_arithmetic_logical_instructions::Cpu8BitArithmeticLogicalInstructions, cpu_8bit_transfer_input_output_instructions::Cpu8BitTransferInputOutputInstructions, cpu_16bit_arithmetic_instructions::Cpu16BitArithmeticInstructions, cpu_16bit_transfer_instructions::Cpu16BitTransferInstructions, cpu_bit_operations_instructions::CpuBitOperationsInstructions, cpu_call_and_return_instructions::CpuCallAndReturnInstructions, cpu_jump_instructions::CpuJumpInstructions, cpu_miscellaneous_instructions::CpuMiscellaneousInstructions, cpu_rotate_shift_instructions::CpuRotateShiftInstructions}, cpu_utils, ppu::Ppu
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
            0x00 | 0xE3 | 0xED => self.nop(), // NOP
            0b01110110 => self.halt(),        // HALT

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

            // Bit Operations are all inside CB prefix instructions

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

    fn execute_cb_prefix_instructions(&mut self) {
        let cb_opcode = self.fetch_opcode();
        self.registers.increment_pc();

        match cb_opcode {
            v if (v & 0b11111000) == 0b00000000 && Cpu::source_is_8bit_register(cb_opcode) => {
                self.rlc_r8(cb_opcode)
            }
            0b00000110 => self.rlc_hl(),
            v if (v & 0b11111000) == 0b00010000 && Cpu::source_is_8bit_register(cb_opcode) => {
                self.rl_r8(cb_opcode)
            }
            0b00010110 => self.rl_hl(),
            v if (v & 0b11111000) == 0b00001000 && Cpu::source_is_8bit_register(cb_opcode) => {
                self.rrc_r8(cb_opcode)
            }
            0b00001110 => self.rrc_hl(),
            v if (v & 0b11111000) == 0b00011000 && Cpu::source_is_8bit_register(cb_opcode) => {
                self.rr_r8(cb_opcode)
            }
            0b00011110 => self.rr_hl(),
            v if (v & 0b11111000) == 0b00100000 && Cpu::source_is_8bit_register(cb_opcode) => {
                self.sla_r8(cb_opcode)
            }
            0b00100110 => self.sla_hl(),
            v if (v & 0b11111000) == 0b00101000 && Cpu::source_is_8bit_register(cb_opcode) => {
                self.sra_r8(cb_opcode)
            }
            0b00101110 => self.sra_hl(),
            v if (v & 0b11111000) == 0b00111000 && Cpu::source_is_8bit_register(cb_opcode) => {
                self.srl_r8(cb_opcode)
            }
            0b00111110 => self.srl_hl(),
            v if (v & 0b11111000) == 0b00110000 && Cpu::source_is_8bit_register(cb_opcode) => {
                self.swap_r8(cb_opcode)
            }
            0b00110110 => self.swap_hl(),
            v if (v & 0b11000000) == 0b01000000 && Cpu::source_is_8bit_register(cb_opcode) => {
                self.bit_b_r8(cb_opcode)
            }
            v if (v & 0b11000111) == 0b01000110 => {
                self.bit_b_hl(cb_opcode)
            }
            v if (v & 0b11000000) == 0b11000000 && Cpu::source_is_8bit_register(cb_opcode) => {
                self.set_b_r8(cb_opcode)
            }
            v if (v & 0b11000111) == 0b11000110 => {
                self.set_b_hl(cb_opcode)
            }
            v if (v & 0b11000000) == 0b10000000 && Cpu::source_is_8bit_register(cb_opcode) => {
                self.reset_b_r8(cb_opcode)
            }
            v if (v & 0b11000111) == 0b10000110 => {
                self.reset_b_hl(cb_opcode)
            }
            _ => {
                println!(
                    "*** Unimplemented CB prefix opcode: 0x{:02X} - bin: 0b{:08b} ***",
                    cb_opcode, cb_opcode
                );
                return;
            }
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
