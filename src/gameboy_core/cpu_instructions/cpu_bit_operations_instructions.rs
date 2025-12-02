use crate::gameboy_core::cpu::Cpu;

pub trait CpuBitOperationsInstructions {
    fn reset_b_hl(&mut self, cb_opcode: u8);
    fn reset_b_r8(&mut self, cb_opcode: u8);
    fn set_b_hl(&mut self, cb_opcode: u8);
    fn set_b_r8(&mut self, cb_opcode: u8);
    fn bit_b_r8(&mut self, cb_opcode: u8);
    fn bit_b_hl(&mut self, cb_opcode: u8);
}

impl CpuBitOperationsInstructions for Cpu {
    /// Test bit b in 8-bit register. Set Z flag if the bit is 0.
    /// Set H flag. Reset N flag.
    fn bit_b_r8(&mut self, cb_opcode: u8) {
        let bit_index = (cb_opcode & 0b00111000) >> 3;
        let register = Self::get_source_register(cb_opcode);
        let value = self.registers.get_8bit_register_value(register);
        
        let bit = match bit_index {
            0 => (value >> 0) & 0x01,
            1 => (value >> 1) & 0x01,
            2 => (value >> 2) & 0x01,
            3 => (value >> 3) & 0x01,
            4 => (value >> 4) & 0x01,
            5 => (value >> 5) & 0x01,
            6 => (value >> 6) & 0x01,
            7 => (value >> 7) & 0x01,
            _ => unreachable!(),
        };

        self.registers.flags_register.set_z_flag(bit);
        self.registers.flags_register.set_h_flag(true);
        self.registers.flags_register.n = false;
    }
    
    /// Test bit b in memory location pointed by HL register. Set Z flag if the bit is 0.
    /// Set H flag. Reset N flag.
    fn bit_b_hl(&mut self, cb_opcode: u8) {
        let bit_index = (cb_opcode & 0b00111000) >> 3;
        let value = self.memory_bus.read_byte(self.registers.get_hl());

        let bit = match bit_index {
            0 => (value >> 0) & 0x01,
            1 => (value >> 1) & 0x01,
            2 => (value >> 2) & 0x01,
            3 => (value >> 3) & 0x01,
            4 => (value >> 4) & 0x01,
            5 => (value >> 5) & 0x01,
            6 => (value >> 6) & 0x01,
            7 => (value >> 7) & 0x01,
            _ => unreachable!(),
        };

        self.registers.flags_register.set_z_flag(bit);
        self.registers.flags_register.set_h_flag(true);
        self.registers.flags_register.n = false;
    }

    /// Sets to 1 the specified bit in specified 8-bit register.
    fn set_b_r8(&mut self, cb_opcode: u8) {
        let bit_index = (cb_opcode & 0b00111000) >> 3;
        let register = Self::get_source_register(cb_opcode);
        let mut value = self.registers.get_8bit_register_value(register);
        
        value = match bit_index {
            0 => value | 0b01,
            1 => value | 0b10,
            2 => value | 0b100,
            3 => value | 0b1000,
            4 => value | 0b10000,
            5 => value | 0b100000,
            6 => value | 0b1000000,
            7 => value | 0b10000000,
            _ => unreachable!(),
        };
        
        self.registers.set_8bit_register_value(register, value);
    }
    
    /// Sets to 1 the specified bit in memory location pointed by HL register.
    fn set_b_hl(&mut self, cb_opcode: u8) {
        let bit_index = (cb_opcode & 0b00111000) >> 3;
        let hl = self.registers.get_hl();
        let mut value = self.memory_bus.read_byte(hl);
        
        value = match bit_index {
            0 => value | 0b01,
            1 => value | 0b10,
            2 => value | 0b100,
            3 => value | 0b1000,
            4 => value | 0b10000,
            5 => value | 0b100000,
            6 => value | 0b1000000,
            7 => value | 0b10000000,
            _ => unreachable!(),
        };
        
        self.memory_bus.write_byte(hl, value);
    }

    /// Resets to 0 the specified bit in specified 8-bit register.
    fn reset_b_r8(&mut self, cb_opcode: u8) {
        let bit_index = (cb_opcode & 0b00111000) >> 3;
        let register = Self::get_source_register(cb_opcode);
        let mut value = self.registers.get_8bit_register_value(register);
        
        value = match bit_index {
            0 => value & !0b01,
            1 => value & !0b10,
            2 => value & !0b100,
            3 => value & !0b1000,
            4 => value & !0b10000,
            5 => value & !0b100000,
            6 => value & !0b1000000,
            7 => value & !0b10000000,
            _ => unreachable!(),
        };
        
        self.registers.set_8bit_register_value(register, value);
    }
    
    /// Sets to 1 the specified bit in memory location pointed by HL register.
    fn reset_b_hl(&mut self, cb_opcode: u8) {
        let bit_index = (cb_opcode & 0b00111000) >> 3;
        let hl = self.registers.get_hl();
        let mut value = self.memory_bus.read_byte(hl);
        
        value = match bit_index {
            0 => value & 0b11111110,
            1 => value & 0b11111101,
            2 => value & 0b11111011,
            3 => value & 0b11110111,
            4 => value & 0b11101111,
            5 => value & 0b11011111,
            6 => value & 0b10111111,
            7 => value & 0b01111111,
            _ => unreachable!(),
        };
        
        self.memory_bus.write_byte(hl, value);
    }
}