use crate::gameboy_core::cpu::Cpu;

pub trait Cpu8BitArithmeticLogicalInstructions {
    fn add_a_r(&mut self, opcode: u8);
    fn add_a_n(&mut self);
    fn add_a_hl(&mut self);
    fn adc_a_r(&mut self, opcode: u8);
    fn adc_a_imm8(&mut self);
    fn adc_a_hl(&mut self);
    fn adc_a_value(&mut self, value: u8);
    fn sub_a_r(&mut self, opcode: u8);
    fn sub_a_imm8(&mut self);
    fn sub_a_hl(&mut self);
    fn sub_a_value(&mut self, value: u8);
    fn sbc_a_r(&mut self, opcode: u8);
    fn sbc_a_imm8(&mut self);
    fn sbc_a_hl(&mut self);
    fn sbc_a_value(&mut self, value: u8);
    fn and_a_r(&mut self, opcode: u8);
    fn and_a_imm8(&mut self);
    fn and_a_hl(&mut self);
    fn and_a_value(&mut self, value: u8);
    fn or_a_r(&mut self, opcode: u8);
    fn or_a_imm8(&mut self);
    fn or_a_hl(&mut self);
    fn or_a_value(&mut self, value: u8);
    fn xor_a_r(&mut self, opcode: u8);
    fn xor_a_imm8(&mut self);
    fn xor_a_hl(&mut self);
    fn xor_a_value(&mut self, value: u8);
    fn cp_a_r(&mut self, opcode: u8);
    fn cp_a_imm8(&mut self);
    fn cp_a_hl(&mut self);
    fn cp_a_value(&mut self, value: u8);
    fn inc_r(&mut self, opcode: u8);
    fn inc_hl(&mut self);
    fn dec_r(&mut self, opcode: u8);
    fn dec_hl(&mut self);
}

impl Cpu8BitArithmeticLogicalInstructions for Cpu {
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
        let h_flag = crate::gameboy_core::cpu_components::FlagsRegister::calculate_h_flag_on_add(self.registers.a, value);
        self.registers.a = result;
        self.registers.flags.n = false;
        self.registers.flags.set_c_flag(carry);
        self.registers.flags.set_z_flag_from_u8(result);
        self.registers.flags.set_h_flag(h_flag);
        self.increment_4_clock_cycles();
    }

    /// Adds 8-bit immediate operand n to the contents of register A and stores the results in register A.
    /// Example: When A = 3Ch,
    /// ADD A. FFh ; A ← 3Bh, Z ← 0, H ← 1, N ← 0, CY ← 1
    fn add_a_n(&mut self) {
        let value = self.get_imm8();
        let (result, carry) = self.registers.a.overflowing_add(value);
        let h_flag = crate::gameboy_core::cpu_components::FlagsRegister::calculate_h_flag_on_add(self.registers.a, value);

        self.registers.a = result;
        self.registers.flags.n = false;
        self.registers.flags.set_c_flag(carry);
        self.registers.flags.set_z_flag_from_u8(result);
        self.registers.flags.set_h_flag(h_flag);
        self.registers.increment_pc();
        self.increment_8_clock_cycles();
    }

    /// Adds the contents of memory specified by the contents of register pair HL to the contents of register A and stores the results in register A.
    /// Example: When A = 3Ch and (HL) = 12h,
    /// ADD A, (HL) ; A ← 4Eh, Z ← 0, H ← 0, N ← 0, CY ← 0
    fn add_a_hl(&mut self) {
        let value = self.get_memory_value_at_hl();
        let (result, carry) = self.registers.a.overflowing_add(value);
        let h_flag = crate::gameboy_core::cpu_components::FlagsRegister::calculate_h_flag_on_add(self.registers.a, value);

        self.registers.a = result;
        self.registers.flags.n = false;
        self.registers.flags.set_c_flag(carry);
        self.registers.flags.set_z_flag_from_u8(result);
        self.registers.flags.set_h_flag(h_flag);
        self.increment_8_clock_cycles();
    }

    /// Adds the contents of register r and CY to the contents of register A and stores the results in register A.
    fn adc_a_r(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        self.adc_a_value(value);
        self.increment_4_clock_cycles();
    }

    /// Adds the contents of the immediate byte and CY to the contents of register A and stores the results in register A.
    fn adc_a_imm8(&mut self) {
        self.adc_a_value(self.get_imm8());
        self.registers.increment_pc();
        self.increment_8_clock_cycles();
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
        let cy = self.registers.flags.get_c_flag_u8();

        let (temp_result, temp_carry) = value.overflowing_add(cy);
        let mut h_flag: bool = crate::gameboy_core::cpu_components::FlagsRegister::calculate_h_flag_on_add(value, cy);

        let (final_result, final_carry) = self.registers.a.overflowing_add(temp_result);
        h_flag |= crate::gameboy_core::cpu_components::FlagsRegister::calculate_h_flag_on_add(self.registers.a, temp_result);

        self.registers.a = final_result;
        self.registers.flags.n = false;
        self.registers.flags.set_c_flag(temp_carry | final_carry);
        self.registers.flags.set_z_flag_from_u8(final_result);
        self.registers.flags.set_h_flag(h_flag);
    }

    /// Subtracts the contents of register r from the contents of register A and stores the results in register A.
    fn sub_a_r(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        self.sub_a_value(value);
        self.increment_4_clock_cycles();
    }

    /// Subtracts the 8-bit immediate operand n from the contents of register A and stores the results in register A.
    fn sub_a_imm8(&mut self) {
        let value = self.get_imm8();
        self.sub_a_value(value);
        self.registers.increment_pc();
        self.increment_8_clock_cycles();
    }

    /// Subtracts the contents of memory specified by the contents of register pair HL from the contents of register A and stores the results in register A.
    fn sub_a_hl(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.sub_a_value(value);
        self.increment_8_clock_cycles();
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
        self.registers.flags.n = true;
        self.registers.flags.set_c_flag(carry);
        self.registers.flags.set_z_flag_from_u8(result);
        self.registers.flags.set_h_flag(half_carry);
    }

    /// Subtracts the contents of register r and CY from the contents of register A and stores the results in register A.
    fn sbc_a_r(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        self.sbc_a_value(value);
        self.increment_4_clock_cycles();
    }

    /// Subtracts the 8-bit immediate operand n and CY from the contents of register A and stores the results in register A.
    fn sbc_a_imm8(&mut self) {
        let value = self.get_imm8();
        self.sbc_a_value(value);
        self.registers.increment_pc();
        self.increment_8_clock_cycles();
    }

    /// Subtracts the contents of memory specified by the contents of register pair HL and CY from the contents of register A and stores the results in register A.
    fn sbc_a_hl(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.sbc_a_value(value);
        self.increment_8_clock_cycles();
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

        if self.registers.flags.c {
            half_carry |= (result & 0x0F) < 1;
            carry |= result < 1;
            let (result_c_flag, _) = result.overflowing_sub(1);
            result = result_c_flag;
        }

        self.registers.a = result;
        self.registers.flags.n = true;
        self.registers.flags.set_c_flag(carry);
        self.registers.flags.set_z_flag_from_u8(result);
        self.registers.flags.set_h_flag(half_carry);
    }

    /// Takes the logical-AND for each bit of the contents of register r and register A, and stores the results in register A.
    fn and_a_r(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        self.and_a_value(value);
        self.increment_4_clock_cycles();
    }

    /// Takes the logical-AND for each bit of the contents of immediate operand and register A, and stores the results in register A.
    fn and_a_imm8(&mut self) {
        let value = self.get_imm8();
        self.and_a_value(value);
        self.registers.increment_pc();
        self.increment_8_clock_cycles();
    }

    /// Takes the logical-AND for each bit of the contents of memory specified by the contents of register pair HL and register A, and stores the results in register A.
    fn and_a_hl(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.and_a_value(value);
        self.increment_8_clock_cycles();
    }

    /// Takes the logical-AND for each bit of the contents of operand s and register A, and stores the results in register A.
    fn and_a_value(&mut self, value: u8) {
        self.registers.a &= value;
        self.registers.flags.set_h_flag(true);
        self.registers.flags.set_z_flag_from_u8(self.registers.a);
        self.registers.flags.n = false;
        self.registers.flags.c = false;
    }

    /// Takes the logical-OR for each bit of the contents of register r and register A, and stores the results in register A.
    fn or_a_r(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        self.or_a_value(value);
        self.increment_4_clock_cycles();
    }

    /// Takes the logical-OR for each bit of the contents of immediate operand and register A, and stores the results in register A.
    fn or_a_imm8(&mut self) {
        let value = self.get_imm8();
        self.or_a_value(value);
        self.registers.increment_pc();
        self.increment_8_clock_cycles();
    }

    /// Takes the logical-OR for each bit of the contents of memory specified by the contents of register pair HL and register A, and stores the results in register A.
    fn or_a_hl(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.or_a_value(value);
        self.increment_8_clock_cycles();
    }

    /// Takes the logical-OR for each bit of the contents of operand s and register A, and stores the results in register A.
    fn or_a_value(&mut self, value: u8) {
        self.registers.a |= value;
        self.registers.flags.set_h_flag(false);
        self.registers.flags.set_z_flag_from_u8(self.registers.a);
        self.registers.flags.n = false;
        self.registers.flags.c = false;
    }

    /// Takes the logical exclusive-OR for each bit of the contents of register r and register A, and stores the results in register A.
    fn xor_a_r(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        self.xor_a_value(value);
        self.increment_4_clock_cycles();
    }

    /// Takes the logical exclusive-OR for each bit of the contents of immediate operand and register A, and stores the results in register A.
    fn xor_a_imm8(&mut self) {
        let value = self.get_imm8();
        self.xor_a_value(value);
        self.registers.increment_pc();
        self.increment_8_clock_cycles();
    }

    /// Takes the logical exclusive-OR for each bit of the contents of memory specified by the contents of register pair HL and register A, and stores the results in register A.
    fn xor_a_hl(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.xor_a_value(value);
        self.increment_8_clock_cycles();
    }

    /// Takes the logical exclusive-OR for each bit of the contents of operand s and register A, and stores the results in register A.
    fn xor_a_value(&mut self, value: u8) {
        self.registers.a ^= value;
        self.registers.flags.set_h_flag(false);
        self.registers.flags.set_z_flag_from_u8(self.registers.a);
        self.registers.flags.n = false;
        self.registers.flags.c = false;
    }

    /// Compares the contents of register r and register A and sets the flag if they are equal.
    fn cp_a_r(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        self.cp_a_value(value);
        self.increment_4_clock_cycles();
    }

    /// Compares the contents of 8-bit immediate operand n and register A and sets the flag if they are equal.
    fn cp_a_imm8(&mut self) {
        let value = self.get_imm8();
        self.cp_a_value(value);
        self.registers.increment_pc();
        self.increment_8_clock_cycles();
    }

    /// Compares the contents of memory specified by the contents of register pair HL and register A and sets the flag if they are equal.
    fn cp_a_hl(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.cp_a_value(value);
        self.increment_8_clock_cycles();
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
        let half_carry = crate::gameboy_core::cpu_components::FlagsRegister::calculate_h_flag_on_sub(self.registers.a, value);

        // Carry flag (C): Set if no borrow occurred (A < B)
        let carry = self.registers.a < value;

        self.registers.flags.n = true;
        self.registers.flags.set_c_flag(carry);
        self.registers.flags.set_z_flag_from_u8(result);
        self.registers.flags.set_h_flag(half_carry);
    }

    /// Increments the contents of register r by 1.
    fn inc_r(&mut self, opcode: u8) {
        let destination_register = Self::get_destination_register(opcode);
        let value = self.registers.get_8bit_register_value(destination_register);

        let (result, _carry) = value.overflowing_add(1);
        let h_flag = crate::gameboy_core::cpu_components::FlagsRegister::calculate_h_flag_on_add(value, 1);
        self.registers.flags.n = false;
        self.registers.flags.set_z_flag_from_u8(result);
        self.registers.flags.set_h_flag(h_flag);

        self.registers
            .set_8bit_register_value(destination_register, result);
        self.increment_4_clock_cycles();
    }

    /// Increments by 1 the contents of memory specified by register pair HL.
    fn inc_hl(&mut self) {
        let value = self.get_memory_value_at_hl();

        let (result, _carry) = value.overflowing_add(1);
        let h_flag = crate::gameboy_core::cpu_components::FlagsRegister::calculate_h_flag_on_add(value, 1);
        self.registers.flags.n = false;
        self.registers.flags.set_z_flag_from_u8(result);
        self.registers.flags.set_h_flag(h_flag);

        self.write_memory_value_at_hl(result);
        self.increment_8_clock_cycles();
    }

    /// Subtract 1 from the contents of register r.
    fn dec_r(&mut self, opcode: u8) {
        let destination_register = Self::get_destination_register(opcode);
        let value = self.registers.get_8bit_register_value(destination_register);

        let (result, _carry) = value.overflowing_sub(1);
        let h_flag = crate::gameboy_core::cpu_components::FlagsRegister::calculate_h_flag_on_sub(value, 1);
        self.registers.flags.n = true;
        self.registers.flags.set_z_flag_from_u8(result);
        self.registers.flags.set_h_flag(h_flag);

        self.registers
            .set_8bit_register_value(destination_register, result);
        self.increment_4_clock_cycles();
    }

    /// Decrements by 1 the contents of memory specified by register pair HL.
    fn dec_hl(&mut self) {
        let value = self.get_memory_value_at_hl();

        let (result, _carry) = value.overflowing_sub(1);
        let h_flag = crate::gameboy_core::cpu_components::FlagsRegister::calculate_h_flag_on_sub(value, 1);
        self.registers.flags.n = true;
        self.registers.flags.set_z_flag_from_u8(result);
        self.registers.flags.set_h_flag(h_flag);

        self.write_memory_value_at_hl(result);
        self.increment_12_clock_cycles();
    }
}