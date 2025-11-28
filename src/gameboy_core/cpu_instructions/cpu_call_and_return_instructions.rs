use crate::gameboy_core::cpu::Cpu;

/// Trait for CPU call and return instructions
pub trait CpuCallAndReturnInstructions {
    fn ret(&mut self);
    fn call_cc_imm16(&mut self, opcode: u8);
    fn rst(&mut self, opcode: u8);
    fn ret_cc(&mut self, opcode: u8);
}

impl CpuCallAndReturnInstructions for Cpu {
    /// Pops from the memory stack the PC value pushed when the subroutine was called, returning control to the source program.
    /// In this case, the contents of the address specified by the SP are loaded in the lower-order byte of the PC,
    /// and the content of the SP is incremented by 1. The contents of the address specified by the new SP
    /// value are then loaded in the higher-order byte of the PC, and the SP is again incremented by 1. (The
    /// value of SP is 2 larger than before instruction execution.)
    fn ret(&mut self) {
        self.registers.pc = self.pop_value_from_sp();
    }

    /// If condition cc matches the flag, the PC value is pushed onto the stack and the PC is loaded with the 16-bit immediate value.
    /// Conditions:
    ///     00 - NZ (Z flag is reset)
    ///     01 - Z  (Z flag is set)
    ///     10 - NC (C flag is reset)
    ///     11 - C  (C flag is set)
    fn call_cc_imm16(&mut self, opcode: u8) {
        if self.check_cc_condition(opcode) {
            self.push_value_to_sp(self.registers.pc);
            self.registers.pc = self.get_imm16();
        }
    }

    /// Pushes the current value of the PC to the memory stack and loads to the PC the page 0 memory addresses provided by operand t.
    /// Then next instruction is fetched from the address specified by the new content of PC.
    /// With the push, the content of the SP is decremented by 1, and the higher-order byte of the PC is loaded
    /// in the memory address specified by the new SP value. The value of the SP is then again decremented
    /// by 1, and the lower-order byte of the PC is loaded in the memory address specified by that value of the SP.
    /// The RST instruction can be used to jump to 1 of 8 addresses.
    fn rst(&mut self, opcode: u8) {
        self.push_value_to_sp(self.registers.pc);
        self.registers.pc = match (opcode & 0b00111000) >> 3 {
            0 => 0x0,
            1 => 0x0008,
            2 => 0x0010,
            3 => 0x0018,
            4 => 0x0020,
            5 => 0x0028,
            6 => 0x0030,
            7 => 0x0038,
            _ => 0x0,
        }
    }

    /// If condition cc matches the flag, pops from the memory stack the PC value pushed when the subroutine was called.
    fn ret_cc(&mut self, opcode: u8) {
        if self.check_cc_condition(opcode) {
            self.registers.pc = self.pop_value_from_sp();
        }
    }

}
