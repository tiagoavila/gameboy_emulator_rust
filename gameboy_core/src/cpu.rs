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

pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl Cpu {
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
            0x00 => Cpu::nop(), // NOP
            v if (v & 0b00000110) == 0b00000110 => self.ld_r8_imm8(opcode),
            0b01110110 => self.halt(), // HALT
            v if (v & 0b01000000) == 0b01000000 => self.ld_r8_r8(opcode),
            v if (v & 0b01000110) == 0b01000110 => self.ld_r8_hl(opcode),
            v if (v & 0b01110000) == 0b01110000 => self.ld_hl_r8(opcode),
            0b00110110 => self.ld_hl_imm8(),
            0b00001010 => self.ld_a_bc(),
            0b00011010 => self.ld_a_de(),
            0b11110010 => self.ld_a_c(),
            0b11100010 => self.ld_c_a(),

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
    }

    fn ld_r8_r8(&mut self, opcode: u8) {
        let destination = Cpu::get_destination_register(opcode);
        let source = opcode & 0b00000111;

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
        let start_address = 0xFF00;
        let c_register = 0b001;
        let ram_address = start_address + c_register;
        let value = self.memory_bus.read_byte(ram_address);
        self.registers.a = value;
    }
    
    /// Loads the contents of register A in the internal RAM, port register, or mode register at the address in the 
    /// range FF00h-FFFFh specified by register C.
    fn ld_c_a(&mut self) {
        let start_address = 0xFF00;
        let c_register = 0b001;
        let ram_address = start_address + c_register;
        self.memory_bus.write_byte(ram_address, self.registers.a);
    }

    /// Get the 8-bit immediate value
    fn get_imm8(&self) -> u8 {
        let imm8 = self.memory_bus.read_byte(self.registers.pc);
        imm8
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

    pub fn get_8bit_register(&self, register: u8) -> u8 {
        match register {
            0b000 => self.b,
            0b001 => self.c,
            0b010 => self.d,
            0b011 => self.e,
            0b100 => self.h,
            0b101 => self.l,
            0b111 => self.a,
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
