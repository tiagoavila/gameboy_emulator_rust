use crate::gameboy_core::cpu_instructions::cpu_helpers::CpuAddOperation;

/// Trait for 16-bit transfer instruction operations
pub trait Cpu16BitTransferInstructions {
    fn ld_r16_imm16(&mut self, opcode: u8);
    fn ld_sp_hl(&mut self);
    fn push_r16_onto_memory_stack(&mut self, opcode: u8);
    fn pop_r16_from_memory_stack(&mut self, opcode: u8);
    fn ld_hl_sp_imm8(&mut self);
    fn ld_imm16_sp(&mut self);
}

impl Cpu16BitTransferInstructions for crate::gameboy_core::cpu::Cpu {
    /// Loads 2 bytes of immediate data to 16-bit register, where it can be the registers BC, DE, HL or SP.
    /// BC = 0b00, DE = 0b01, HL = 0b10, SP = 0b11
    fn ld_r16_imm16(&mut self, opcode: u8) {
        let destination_register = Self::get_16bit_destination_register(opcode);
        let value = self.get_imm16();

        match destination_register {
            0b00 => self.registers.set_bc(value),
            0b01 => self.registers.set_de(value),
            0b10 => self.registers.set_hl(value),
            0b11 => self.registers.sp = value,
            _ => (),
        };

        self.registers.increment_pc_twice();
    }

    /// Loads the contents of register pair HL in stack pointer SP.
    fn ld_sp_hl(&mut self) {
        self.registers.sp = self.registers.get_hl();
    }

    /// Pushes the contents of register pair qq (a 16-bit register) onto the memory stack. First 1 is subtracted from SP and the
    /// contents of the higher portion of qq are placed on the stack. The contents of the lower portion of qq are
    /// then placed on the stack. The contents of SP are automatically decremented by 2.
    /// FF80h-FFFEh: Can be used as CPU work RAM and/or stack RAM.
    fn push_r16_onto_memory_stack(&mut self, opcode: u8) {
        let source_register = Self::get_16bit_destination_register(opcode);
        let value = match source_register {
            0b00 => self.registers.get_bc(),
            0b01 => self.registers.get_de(),
            0b10 => self.registers.get_hl(),
            0b11 => self.registers.get_af(),
            _ => 0,
        };

        self.push_value_to_sp(value);
    }

    /// Pops contents from the memory stack and into register pair qq.
    /// First the contents of memory specified by the contents of SP are loaded in the lower portion of qq.
    /// Next, the contents of SP are incremented by 1 and the contents of the memory they specify are loaded in the upper portion of qq.
    /// The contents of SP are automatically incremented by 2.
    fn pop_r16_from_memory_stack(&mut self, opcode: u8) {
        let value = self.pop_value_from_sp();

        let destination_register = Self::get_16bit_destination_register(opcode);
        match destination_register {
            0b00 => self.registers.set_bc(value),
            0b01 => self.registers.set_de(value),
            0b10 => self.registers.set_hl(value),
            0b11 => self.registers.set_af(value),

            _ => (),
        }
    }

    /// Adds the signed 8-bit immediate value to the stack pointer SP and stores the result in HL.
    /// The Z flag is reset. The N flag is reset.
    /// H flag is set if there is a carry from bit 3 and C flag is set if there is a carry from bit 7.
    fn ld_hl_sp_imm8(&mut self) {
        let imm8 = self.get_imm8();
        let sp = self.registers.sp;
        let (result, c_flag, h_flag) = sp.add_u8_as_signed(imm8);
        self.registers.set_hl(result);
        self.registers.flags.n = false;
        self.registers.flags.z = false;
        self.registers.flags.set_c_flag(c_flag);
        self.registers.flags.set_h_flag(h_flag);
        self.registers.increment_pc();
    }

    /// Stores the lower byte of SP at address nn specified by the 16-bit immediate operand nn and the upper byte of SP at address nn + 1.
    fn ld_imm16_sp(&mut self) {
        let imm16 = self.get_imm16();
        let sp_lower_byte = (self.registers.sp & 0b011111111) as u8;
        self.memory_bus.write_byte(imm16, sp_lower_byte);

        let sp_higher_byte = (self.registers.sp >> 8) as u8;
        self.memory_bus.write_byte(imm16 + 1, sp_higher_byte);

        self.registers.increment_pc_twice();
    }
}
