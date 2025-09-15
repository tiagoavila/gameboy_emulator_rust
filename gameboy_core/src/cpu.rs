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

            // TODO: Refactor using bitmask
            // match value {
            // v if (v & 0b00101000) == 0b00101000 => { /* bits 3 and 5 set */ }
            // v if (v & 0b00010000) != 0 => { /* only bit 4 set */ }
            // _ => { /* other cases */ }
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E => self.ld_r8_imm8(opcode),

            0x7F => Cpu::nop(), // LD A, A
            0x78 | 0x79 | 0x7A | 0x7B | 0x7C | 0x7D => self.ld_r8_r8(opcode),
            0x7E => self.ld_a_hl(),

            _ => return,
        }
    }

    /// No Operation - Do nothing for one CPU cycle.
    fn nop() {
        return;
    }

    /// Load the 8-bit immediate value into the specified 8-bit register.
    fn ld_r8_imm8(&mut self, opcode: u8) {
        let register = (opcode & 0b00111000) >> 3;
        let imm8 = self.memory_bus.read_byte(self.registers.pc);
        self.registers.set_8bit_register(register, imm8);
    }

    fn ld_r8_r8(&mut self, opcode: u8) {
        let dest = (opcode & 0b00111000) >> 3;
        let source = opcode & 0b00000111;
        let value = self.registers.get_8bit_register(source);
        self.registers.set_8bit_register(dest, value);
    }

    // Load the contents of register HL into register A.
    fn ld_a_hl(&mut self) {
        let hl = self.registers.get_hl();
        self.registers.a = self.memory_bus.read_byte(hl);
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
        ((self.h << 8) | self.l) as u16
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
}
