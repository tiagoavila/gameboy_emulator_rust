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
        self.flags_register.n = false;
        self.flags_register.set_c_flag(carry);
        self.flags_register.set_h_flag(h_flag);
    }

    /// Adds the signed 8-bit immediate value to the stack pointer SP and stores the result in SP.
    fn add_sp_imm8(&mut self) {
        let imm8 = self.get_imm8() as u16;
        let (result, carry) = self.registers.sp.overflowing_add(imm8);
        let h_flag = FlagsRegister::calculate_h_flag_on_add_u16_numbers(self.registers.sp, imm8);

        self.registers.sp = result;
        self.flags_register.n = false;
        self.flags_register.z = false;
        self.flags_register.set_c_flag(carry);
        self.flags_register.set_h_flag(h_flag);
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
