use crate::constants::MEMORY_SIZE;

pub struct Cpu {
    pub registers: Registers,
    pub flags_register: FlagsRegister,
    pub memory_bus: MemoryBus,
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

pub struct FlagsRegister {
    pub z_flag: bool, // Zero Flag
    pub n_flag: bool, // Subtract Flag
    pub h_flag: bool, // Half Carry Flag
    pub c_flag: bool, // Carry Flag
}

impl FlagsRegister {
    /// This bit is set if a carry occurred from the last math operation
    pub fn set_c_flag(&mut self, carry: bool) {
        self.c_flag = carry
    }
    /// This bit is set if a carry occurred from the lower nibble (a.k.a the lower four bits) in the last math operation.
    /// We can set this by masking out the upper nibble of both the A register and the value we're adding and testing 
    /// if this value is greater than 0xF (0b00001111).
    pub fn set_h_flag_on_add(&mut self, value1: u8, value2: u8) {
        let value1_lower_nibble = value1 & 0b00001111;
        let value2_lower_nibble = value2 & 0b00001111;
        self.h_flag = value1_lower_nibble + value2_lower_nibble > 0xF;
    }
    
    /// This bit is set if and only if the result of an operation is zero
    pub fn set_z_flag(&mut self, result: u8) {
        self.z_flag = result == 0;
    }
}

pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl Cpu {
    const START_ADDRESS_FOR_LOAD_INSTRUCTIONS: u16 = 0xFF00;

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
    fn execute(&mut self, opcode: u8) {
        match opcode {
            0x00 => Cpu::nop(),        // NOP
            0b01110110 => self.halt(), // HALT

            // 8-Bit Transfer and Input/Output Instructions
            v if (v & 0b00000110) == 0b00000110 => self.ld_r8_imm8(opcode),
            v if (v & 0b01000000) == 0b01000000 => self.ld_r8_r8(opcode),
            v if (v & 0b01000110) == 0b01000110 => self.ld_r8_hl(opcode),
            v if (v & 0b01110000) == 0b01110000 => self.ld_hl_r8(opcode),
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
            v if (v & 0b10000000) == 0b10000000 => self.add_a_r(opcode),
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
        self.registers.set_8bit_register(destination, imm8);
        self.registers.increment_pc();
    }

    fn ld_r8_r8(&mut self, opcode: u8) {
        let destination = Cpu::get_destination_register(opcode);
        let source = Cpu::get_source_register(opcode);

        if destination == source {
            return; // No operation needed if both registers are the same
        }

        let value = self.registers.get_8bit_register(source);
        self.registers.set_8bit_register(destination, value);
    }

    /// Load the contents of register HL into 8-bit register.
    fn ld_r8_hl(&mut self, opcode: u8) {
        let destination = Cpu::get_destination_register(opcode);
        let hl = self.registers.get_hl();
        let value = self.memory_bus.read_byte(hl);
        self.registers.set_8bit_register(destination, value);
    }

    /// Stores the contents of register r in memory specified by register pair HL.
    fn ld_hl_r8(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register(source);
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
        let c_register = 0b001;
        let ram_address = Self::START_ADDRESS_FOR_LOAD_INSTRUCTIONS + c_register;
        let value = self.memory_bus.read_byte(ram_address);
        self.registers.a = value;
    }

    /// Loads the contents of register A in the internal RAM, port register, or mode register at the address in the
    /// range FF00h-FFFFh specified by register C.
    fn ld_c_a(&mut self) {
        let c_register = 0b001;
        let ram_address = Self::START_ADDRESS_FOR_LOAD_INSTRUCTIONS + c_register;
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
        let hl = self.registers.get_hl();
        let value = self.memory_bus.read_byte(hl);
        self.registers.a = value;
        self.registers.increment_hl();
    }

    /// Loads in register A the contents of memory specified by the contents of register pair HL and simultaneously decrements the contents of HL.
    /// Example: When HL = 8A5Ch and (8A5Ch) = 3Ch,
    /// LD A, (HLD) ; A ← 3Ch, HL ← 8A5Bh
    fn ld_a_hld(&mut self) {
        let hl = self.registers.get_hl();
        let value = self.memory_bus.read_byte(hl);
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
        let value = self.registers.get_8bit_register(source);
        let (result, carry) = self.registers.a.overflowing_add(value);
        self.flags_register.n_flag = false;
        self.flags_register.set_c_flag(carry);
        self.flags_register.set_z_flag(result);
        self.flags_register.set_h_flag_on_add(self.registers.a, value); 
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
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0x100,
        }
    }

    pub fn increment_pc(&mut self) {
        self.pc += 1;
    }

    pub fn increment_pc_twice(&mut self) {
        self.pc += 2;
    }

    pub fn set_8bit_register(&mut self, register: u8, value: u8) {
        match register {
            0b000 => self.b = value,
            0b001 => self.c = value,
            0b010 => self.d = value,
            0b011 => self.e = value,
            0b100 => self.h = value,
            0b101 => self.l = value,
            0b111 => self.a = value,
            _ => (),
        }
    }

    /// Register r, r'
    /// A 111
    /// B 000
    /// C 001
    /// D 010
    /// E 011
    /// H 100
    /// L 101
    pub fn get_8bit_register(&self, register: u8) -> u8 {
        match register {
            0b000 => self.b,
            0b111 => self.a,
            0b001 => self.c,
            0b010 => self.d,
            0b011 => self.e,
            0b100 => self.h,
            0b101 => self.l,
            _ => 0,
        }
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn increment_hl(&mut self) {
        let mut hl = self.get_hl();
        hl += 1;
        self.set_hl(hl);
    }

    pub fn decrement_hl(&mut self) {
        let mut hl = self.get_hl();
        hl -= 1;
        self.set_hl(hl);
    }

    pub fn set_hl(&mut self, value: u16) {
        let h = (value >> 8) as u8;
        let l = (value & 0b011111111) as u8;
        self.h = h;
        self.l = l;
    }
}

impl FlagsRegister {
    pub fn new() -> Self {
        Self {
            z_flag: false,
            n_flag: false,
            h_flag: false,
            c_flag: false,
        }
    }

    pub fn get_zero_flag(&self) -> bool {
        self.z_flag
    }
}

impl MemoryBus {
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
        }
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
