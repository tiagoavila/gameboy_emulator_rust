use crate::prelude::MEMORY_SIZE;

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
            0b111 => self.a = value,
            0b001 => self.c = value,
            0b010 => self.d = value,
            0b011 => self.e = value,
            0b100 => self.h = value,
            0b101 => self.l = value,
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

    /// This bit is set if a carry occurred from the last math operation
    pub fn set_c_flag(&mut self, carry: bool) {
        self.c_flag = carry
    }

    /// This bit is set if a carry occurred from the lower nibble (a.k.a the lower four bits) in the last math operation.
    /// We can set this by masking out the upper nibble of both the A register and the value we're adding and testing
    /// if this value is greater than 0xF (0b00001111).
    pub fn calculate_h_flag(value1: u8, value2: u8) -> bool {
        let value1_lower_nibble = value1 & 0b00001111;
        let value2_lower_nibble = value2 & 0b00001111;

        value1_lower_nibble + value2_lower_nibble > 0xF
    }
    
    /// This bit is set if and only if the result of an operation is zero
    pub fn set_z_flag(&mut self, result: u8) {
        self.z_flag = result == 0;
    }
    
    /// Returns the c_flag as u8 to be used in ADC instructions
    pub fn get_c_flag_u8(&self) -> u8 {
        if self.c_flag {
            1
        } else {
            0
        }
    }
    
    pub fn set_h_flag(&mut self, h_flag: bool) {
        self.h_flag = h_flag;
    }
}

impl MemoryBus {
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}