/// Trait for rotate and shift instruction operations
pub trait CpuRotateShiftInstructions {
    fn swap_hl(&mut self);
    fn swap_r8(&mut self, cb_opcode: u8);
    fn srl_hl(&mut self);
    fn srl_r8(&mut self, cb_opcode: u8);
    fn sra_hl(&mut self);
    fn sra_r8(&mut self, cb_opcode: u8);
    fn rlca(&mut self);
    fn rla(&mut self);
    fn rrca(&mut self);
    fn rra(&mut self);
    fn rotate_left_and_update_flags(&mut self, value: u8, copy_c_flag_to_bit0: bool, set_z_flag: bool) -> u8;
    fn rotate_right_and_update_flags(&mut self, value: u8, rotate_through_c_flag: bool, set_z_flag: bool) -> u8;
    fn rlc_r8(&mut self, cb_opcode: u8);
    fn rlc_hl(&mut self);
    fn rl_r8(&mut self, cb_opcode: u8);
    fn rl_hl(&mut self);
    fn rrc_r8(&mut self, cb_opcode: u8);
    fn rrc_hl(&mut self);
    fn rr_r8(&mut self, cb_opcode: u8);
    fn rr_hl(&mut self);
    fn sla_r8(&mut self, cb_opcode: u8);
    fn sla_hl(&mut self);
}

impl CpuRotateShiftInstructions for crate::gameboy_core::cpu::Cpu {
    /// Rotates the contents of register A to the left.
    /// That is, the contents of bit 0 are copied to bit 1 and the previous contents of bit 1 (the contents before the copy operation)
    /// are copied to bit 2. The same operation is repeated in sequence for the rest of the register.
    /// The contents of bit 7 are placed in both the CY flag and bit 0 of register A.
    fn rlca(&mut self) {
        let rotated_value = self.rotate_left_and_update_flags(self.registers.a, false, false);
        self.registers.a = rotated_value;
        self.increment_4_clock_cycles();
    }

    /// Rotate the contents of register A to the left, through the carry (CY) flag. That is, the contents of bit 0 are copied to bit 1,
    /// and the previous contents of bit 1 (before the copy operation) are copied to bit 2. The same operation is repeated in sequence for the rest
    /// of the register.
    /// The previous contents of the carry flag are copied to bit 0.
    fn rla(&mut self) {
        let rotated_value = self.rotate_left_and_update_flags(self.registers.a, true, false);
        self.registers.a = rotated_value;
        self.increment_4_clock_cycles();
    }

    /// Rotate the contents of register A to the right. That is, the contents of bit 7 are copied to bit 6,
    /// and the previous contents of bit 6 (before the copy) are copied to bit 5.
    /// The same operation is repeated in sequence for the rest of the register.
    /// The contents of bit 0 are placed in both the C flag and bit 7 of register A
    fn rrca(&mut self) {
        let rotated_value = self.rotate_right_and_update_flags(self.registers.a, false, false);
        self.registers.a = rotated_value;
        self.increment_4_clock_cycles();
    }

    /// Rotate the contents of register A to the right, through the carry (CY) flag.
    /// That is, the contents of bit 7 are copied to bit 6, and the previous contents of bit 6 (before the copy) are copied to bit 5.
    /// The same operation is repeated in sequence for the rest of the register.
    /// The previous contents of the carry flag are copied to bit 7.
    fn rra(&mut self) {
        let rotated_value = self.rotate_right_and_update_flags(self.registers.a, true, false);
        self.registers.a = rotated_value;
        self.increment_4_clock_cycles();
    }

    /// Rotate the contents of 8-bit register to the left. That is, the contents of bit 0 are copied to bit 1,
    /// and the previous contents of bit 1 (before the copy operation) are copied to bit 2.
    /// The same operation is repeated in sequence for the rest of the register.
    /// The contents of bit 7 are placed in both the CY flag and bit 0 of register B
    fn rlc_r8(&mut self, cb_opcode: u8) {
        let register = Self::get_source_register(cb_opcode);
        let value = self.registers.get_8bit_register_value(register);

        let rotated_value = self.rotate_left_and_update_flags(value, false, true);
        self.registers
            .set_8bit_register_value(register, rotated_value);
        self.increment_8_clock_cycles();
    }

    /// Rotates the contents of memory specified by register pair HL to the left.
    /// The contents of bit 7 are placed in both the CY flag and bit 0 of register B
    fn rlc_hl(&mut self) {
        let hl = self.registers.get_hl();
        let value = self.memory_bus.read_byte(hl);

        let rotated_value = self.rotate_left_and_update_flags(value, false, true);
        self.memory_bus.write_byte(hl, rotated_value);
        self.increment_16_clock_cycles();
    }

    /// Rotate the contents of 8-bit register to the left. That is, the contents of bit 0 are copied to bit 1,
    /// and the previous contents of bit 1 (before the copy operation) are copied to bit 2.
    /// The same operation is repeated in sequence for the rest of the register.
    /// The previous contents of the carry (CY) flag are copied to bit 0 of register
    fn rl_r8(&mut self, cb_opcode: u8) {
        let register = Self::get_source_register(cb_opcode);
        let value = self.registers.get_8bit_register_value(register);

        let rotated_value = self.rotate_left_and_update_flags(value, true, true);
        self.registers
            .set_8bit_register_value(register, rotated_value);
        self.increment_8_clock_cycles();
    }

    /// Rotates the contents of memory specified by register pair HL to the left.
    /// The previous contents of the carry (CY) flag are copied to bit 0 of register B
    fn rl_hl(&mut self) {
        let hl = self.registers.get_hl();
        let value = self.memory_bus.read_byte(hl);

        let rotated_value = self.rotate_left_and_update_flags(value, true, true);
        self.memory_bus.write_byte(hl, rotated_value);
        self.increment_16_clock_cycles();
    }

    /// Rotates the contents of a 8-bit register to the right.
    /// That is, the contents of bit 7 are copied to bit 6,
    /// and the previous contents of bit 6 (before the copy) are copied to bit 5.
    /// The same operation is repeated in sequence for the rest of the register.
    /// The contents of bit 0 are placed in both the C flag and bit 7 of the register.
    fn rrc_r8(&mut self, cb_opcode: u8) {
        let register = Self::get_source_register(cb_opcode);
        let value = self.registers.get_8bit_register_value(register);

        let rotated_value = self.rotate_right_and_update_flags(value, false, true);
        self.registers
            .set_8bit_register_value(register, rotated_value);
        self.increment_8_clock_cycles();
    }

    /// Rotates the contents of memory specified by register pair HL to the right.
    /// The contents of bit 0 are placed in both the C flag and bit 7 of the register.
    fn rrc_hl(&mut self) {
        let hl = self.registers.get_hl();
        let value = self.memory_bus.read_byte(hl);

        let rotated_value = self.rotate_right_and_update_flags(value, false, true);
        self.memory_bus.write_byte(hl, rotated_value);
        self.increment_16_clock_cycles();
    }

    /// Rotates the contents of a 8-bit register to the right.
    /// That is, the contents of bit 7 are copied to bit 6,
    /// and the previous contents of bit 6 (before the copy) are copied to bit 5.
    /// The same operation is repeated in sequence for the rest of the register.
    /// The previous contents of the carry (CY) flag are copied to bit 7 of the register.
    fn rr_r8(&mut self, cb_opcode: u8) {
        let register = Self::get_source_register(cb_opcode);
        let value = self.registers.get_8bit_register_value(register);

        let rotated_value = self.rotate_right_and_update_flags(value, true, true);
        self.registers
            .set_8bit_register_value(register, rotated_value);
        self.increment_8_clock_cycles();
    }

    /// Rotates the contents of memory specified by register pair HL to the right.
    /// The previous contents of the carry (CY) flag are copied to bit 7 of the register.
    fn rr_hl(&mut self) {
        let hl = self.registers.get_hl();
        let value = self.memory_bus.read_byte(hl);

        let rotated_value = self.rotate_right_and_update_flags(value, true, true);
        self.memory_bus.write_byte(hl, rotated_value);
        self.increment_16_clock_cycles();
    }
    
    /// Shifts the contents of a 8-bit register to the left. That is, the contents of bit 0 are copied to bit 1 and the 
    /// previous contents of bit 1 (the contents before the copy operation) are copied to bit 2. 
    /// The same operation is repeated in sequence for the rest of the operand. 
    /// The content of bit 7 is copied to CY, and bit 0 is reset.
    fn sla_r8(&mut self, cb_opcode: u8) {
        let register = Self::get_source_register(cb_opcode);
        let mut value = self.registers.get_8bit_register_value(register);

        let bit7 = value >> 7;
        value <<= 1;
        
        self.registers.flags.set_c_flag(bit7 == 1);
        self.registers.flags.set_z_flag_from_u8(value);
        self.registers.flags.n = false;
        self.registers.flags.set_h_flag(false);
        
        self.registers.set_8bit_register_value(register, value);
        self.increment_8_clock_cycles();
    }
    
    /// Shifts the contents of memory specified by register pair HL to the left.
    /// The content of bit 7 is copied to CY, and bit 0 is reset.
    fn sla_hl(&mut self) {
        let hl = self.registers.get_hl();
        let mut value = self.memory_bus.read_byte(hl);

        let bit7 = value >> 7;
        value <<= 1;
        
        self.registers.flags.set_c_flag(bit7 == 1);
        self.registers.flags.set_z_flag_from_u8(value);
        self.registers.flags.n = false;
        self.registers.flags.set_h_flag(false);
        
        self.memory_bus.write_byte(hl, value);
        self.increment_16_clock_cycles();
    }

    /// Shifts the contents of 8-bit register to the right. That is, the contents of bit 7 are copied to bit 6 and the
    /// previous contents of bit 6 (the contents before the copy operation) are copied to bit 5. 
    /// The same operation is repeated in sequence for the rest of the operand. 
    /// The contents of bit 0 are copied to CY, and the content of bit 7 is unchanged.
    fn sra_r8(&mut self, cb_opcode: u8) {
        let register = Self::get_source_register(cb_opcode);
        let mut value = self.registers.get_8bit_register_value(register);

        let bit7 = value & 0b10000000;
        let bit0 = value & 0b00000001;

        value >>= 1;
        value |= bit7;
        
        self.registers.flags.set_c_flag(bit0 == 1);
        self.registers.flags.set_z_flag_from_u8(value);
        self.registers.flags.n = false;
        self.registers.flags.set_h_flag(false);
        
        self.registers.set_8bit_register_value(register, value);
        self.increment_8_clock_cycles();
    }
    
    /// Shifts the contents of memory specified by register pair HL to the right.
    /// The contents of bit 0 are copied to CY, and the content of bit 7 is unchanged.
    fn sra_hl(&mut self) {
        let hl = self.registers.get_hl();
        let mut value = self.memory_bus.read_byte(hl);

        let bit7 = value & 0b10000000;
        let bit0 = value & 0b00000001;
        value >>= 1;
        value |= bit7;
        
        self.registers.flags.set_c_flag(bit0 == 1);
        self.registers.flags.set_z_flag_from_u8(value);
        self.registers.flags.n = false;
        self.registers.flags.set_h_flag(false);
        
        self.memory_bus.write_byte(hl, value);
        self.increment_16_clock_cycles();
    }

    /// Shifts the contents of operand m to the right. That is, the contents of bit 7 are copied to bit 6 and the 
    /// previous contents of bit 6 (the contents before the copy operation) are copied to bit 5. 
    /// The same operation is repeated in sequence for the rest of the operand. 
    /// The contents of bit 0 are copied to CY, and bit 7 is reset. 
    fn srl_r8(&mut self, cb_opcode: u8) {
        let register = Self::get_source_register(cb_opcode);
        let mut value = self.registers.get_8bit_register_value(register);

        let bit0 = value & 0b00000001;

        value >>= 1;
        
        self.registers.flags.set_c_flag(bit0 == 1);
        self.registers.flags.set_z_flag_from_u8(value);
        self.registers.flags.n = false;
        self.registers.flags.set_h_flag(false);
        
        self.registers.set_8bit_register_value(register, value);
        self.increment_8_clock_cycles();
    }
    
    /// Shifts the contents of memory specified by register pair HL to the right.
    /// The contents of bit 0 are copied to CY, and bit 7 is reset
    fn srl_hl(&mut self) {
        let hl = self.registers.get_hl();
        let mut value = self.memory_bus.read_byte(hl);

        let bit0 = value & 0b00000001;
        value >>= 1;
        
        self.registers.flags.set_c_flag(bit0 == 1);
        self.registers.flags.set_z_flag_from_u8(value);
        self.registers.flags.n = false;
        self.registers.flags.set_h_flag(false);
        
        self.memory_bus.write_byte(hl, value);
        self.increment_16_clock_cycles();
    }
    
    /// Shifts the contents of the lower-order and higher-order 4 bits of a 8-bit register.
    fn swap_r8(&mut self, cb_opcode: u8) {
        let register = Self::get_source_register(cb_opcode);
        let value = self.registers.get_8bit_register_value(register);
        
        let high_order_4_bits = value & 0b11110000;
        let low_order_4_bits = value & 0b00001111;
        
        let swapped_value = high_order_4_bits >> 4 | low_order_4_bits << 4;
        self.registers.set_8bit_register_value(register, swapped_value);
        self.registers.flags.set_z_flag_from_u8(swapped_value);
        self.registers.flags.n = false;
        self.registers.flags.set_h_flag(false);
        self.registers.flags.set_c_flag(false);
        self.increment_8_clock_cycles();
    }
    
    /// Shifts the contents of the lower-order and higher-order 4 bits of a 8-bit register.
    fn swap_hl(&mut self) {
        let hl = self.registers.get_hl();
        let value = self.memory_bus.read_byte(hl);
        
        let high_order_4_bits = value & 0b11110000;
        let low_order_4_bits = value & 0b00001111;
        
        let swapped_value = high_order_4_bits >> 4 | low_order_4_bits << 4;
        self.memory_bus.write_byte(hl, swapped_value);
        self.registers.flags.set_z_flag_from_u8(swapped_value);
        self.registers.flags.n = false;
        self.registers.flags.set_h_flag(false);
        self.registers.flags.set_c_flag(false);
        self.increment_16_clock_cycles();
    }

    /// Rotates a 8-bit value to the left, updating the CPU flags accordingly.
    /// C flag always receives the contents of bit 7.
    /// Z flag is set if the result is 0; and flags N and H are reset.
    ///
    /// Returns the rotated value.
    /// If `rotate_through_c_flag` is true, the previous value of the C flag is placed in bit 0.
    /// If false, bit 7 is placed in bit 0.
    fn rotate_left_and_update_flags(&mut self, mut value: u8, rotate_through_c_flag: bool, set_z_flag: bool) -> u8 {
        let bit7 = value >> 7;
        value <<= 1;

        if rotate_through_c_flag {
            if self.registers.flags.c {
                value |= 0b00000001;
            }
        } else if bit7 == 1 {
            value |= 0b00000001;
        }

        self.registers.flags.set_c_flag(bit7 == 1);
        self.registers.flags.set_n_flag(false);
        self.registers.flags.set_h_flag(false);

        // Some rotate right instructions set the Z flag based on the result, and some do not. This is handled here.
        if set_z_flag {
            self.registers.flags.set_z_flag_from_u8(value);
        } else {
            self.registers.flags.set_z_flag(false);
        }

        value
    }

    /// Rotates a 8-bit value to the right, updating the CPU flags accordingly.
    /// C flag always receives the contents of bit 0.
    /// Z flag is set if the result is 0; and flags N and H are reset.

    /// Returns the rotated value.
    /// If `rotate_through_c_flag` is true, the previous value of the C flag is placed in bit 7.
    /// If false, bit 0 is placed in bit 7.
    fn rotate_right_and_update_flags(&mut self, mut value: u8, rotate_through_c_flag: bool, set_z_flag: bool) -> u8 {
        let bit0 = value & 0b00000001;
        value >>= 1;

        if rotate_through_c_flag {
            if self.registers.flags.c {
                value |= 0b10000000;
            }
        } else if bit0 == 1 {
            value |= 0b10000000;
        }

        self.registers.flags.set_c_flag(bit0 == 1);
        self.registers.flags.set_n_flag(false);
        self.registers.flags.set_h_flag(false);

        // Some rotate right instructions set the Z flag based on the result, and some do not. This is handled here.
        if set_z_flag {
            self.registers.flags.set_z_flag_from_u8(value);
        } else {
            self.registers.flags.set_z_flag(false);
        }

        value
    }
}
