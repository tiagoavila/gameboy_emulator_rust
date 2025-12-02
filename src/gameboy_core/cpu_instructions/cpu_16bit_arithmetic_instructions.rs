use crate::gameboy_core::cpu_components::FlagsRegister;

/// Trait for 16-bit arithmetic instruction operations
pub trait Cpu16BitArithmeticInstructions {
    fn add_hl_r16(&mut self, opcode: u8);
    fn add_sp_imm8(&mut self);
    fn inc_r16(&mut self, opcode: u8);
    fn dec_r16(&mut self, opcode: u8);
}

impl Cpu16BitArithmeticInstructions for crate::gameboy_core::cpu::Cpu {
    /// Adds the contents of a 16-bit register to the contents of register pair HL and stores the results in HL.
    /// The 16-bit register can be BC, DE, HL or SP.
    fn add_hl_r16(&mut self, opcode: u8) {
        let source_register = Self::get_16bit_destination_register(opcode);
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
        self.registers.flags.n = false;
        self.registers.flags.set_c_flag(carry);
        self.registers.flags.set_h_flag(h_flag);
    }

    /// Adds the signed 8-bit immediate value to the stack pointer SP and stores the result in SP.
    fn add_sp_imm8(&mut self) {
        let imm8 = self.get_imm8(); // u8 (e.g., 0xFF)
        let sp_val = self.registers.sp;

        // --- 1. Calculate Flags (C and H) ---
        // For ADD SP, n (Opcode E8h), the Carry (C) and Half Carry (H) flags are calculated
        // based on the unsigned addition of the low byte of SP and the immediate operand (imm8),
        // checking for carries out of bit 7 (C) and bit 3 (H), respectively [1, 2].
        let lower_sp = (sp_val & 0x00FF) as u8;
        let (_flag_result, c_carry) = lower_sp.overflowing_add(imm8);
        let h_carry = FlagsRegister::calculate_h_flag_on_add(lower_sp, imm8);

        // --- 2. Calculate SP result (16-bit signed arithmetic) ---
        // Convert the 8-bit unsigned immediate value (u8: 0xFF) into a signed 8-bit integer (i8: -1).
        let imm8_signed = imm8 as i8;

        // Sign-extend the offset to a 16-bit signed integer (i16: 0xFFFF).
        let offset_signed: i16 = imm8_signed as i16;

        // Convert the resulting 16-bit signed offset to its unsigned representation (u16)
        // to allow safe wrapping addition with sp_val (u16).
        let offset_u16 = offset_signed as u16;

        // Perform the 16-bit addition. The Game Boy SP wraps around 16 bits.
        let result = sp_val.wrapping_add(offset_u16);

        // --- 3. Update registers and flags ---
        self.registers.sp = result;

        // Z and N flags are reset for this instruction [1, 2].
        self.registers.flags.n = false;
        self.registers.flags.z = false;

        self.registers.flags.set_c_flag(c_carry);
        self.registers.flags.set_h_flag(h_carry);

        self.registers.increment_pc();
    } 

    /// Increments the contents of a 16-bit register by 1. The 16-bit register can be BC, DE, HL or SP.
    fn inc_r16(&mut self, opcode: u8) {
        let source_register = Self::get_16bit_destination_register(opcode);
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
        let source_register = Self::get_16bit_destination_register(opcode);
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
}
