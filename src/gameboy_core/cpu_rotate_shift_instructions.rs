/// Trait for rotate and shift instruction operations
pub trait CpuRotateShiftInstructions {
    fn rlca(&mut self);
    fn rla(&mut self);
    fn rrca(&mut self);
    fn rra(&mut self);
    fn rotate_left_and_update_flags(&mut self, value: u8) -> u8;
    fn rotate_left_and_update_flags_u16(&mut self, value: u16) -> u16;
    fn rotate_right_and_update_flags(&mut self, value: u8) -> u8;
    fn rlc_r8(&mut self, cb_opcode: u8);
    fn rlc_hl(&mut self);
}

impl CpuRotateShiftInstructions for crate::gameboy_core::cpu::Cpu {
    /// Rotates the contents of register A to the left.
    /// That is, the contents of bit 0 are copied to bit 1 and the previous contents of bit 1 (the contents before the copy operation)
    /// are copied to bit 2. The same operation is repeated in sequence for the rest of the register.
    /// The contents of bit 7 are placed in CY.
    fn rlca(&mut self) {
        let rotated_value = self.rotate_left_and_update_flags(self.registers.a);
        self.registers.a = rotated_value;
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

    /// Rotates the contents of a 8-bit register to the left.
    fn rlc_r8(&mut self, cb_opcode: u8) {
        let register = Self::get_source_register(cb_opcode);
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
}