use crate::gameboy_core::cpu::Cpu;


/// Trait for CPU jump instructions
pub trait CpuJumpInstructions {
    fn jp_imm16(&mut self);
    fn jp_cc_imm16(&mut self, opcode: u8);
    fn jr_imm8(&mut self);
    fn jr_cc_imm8(&mut self, opcode: u8);
}

impl CpuJumpInstructions for Cpu {
    /// Loads the 16-bit immediate value to the program counter (PC).
    fn jp_imm16(&mut self) {
        let imm16 = self.get_imm16();
        self.registers.pc = imm16;
    }

    /// Loads operand nn in the PC if condition cc and the flag status match.
    /// The subsequent instruction starts at address nn.
    /// If condition cc and the flag status do not match, the contents of the PC are incremented, and the
    /// instruction following the current JP instruction is executed.
    fn jp_cc_imm16(&mut self, opcode: u8) {
        if self.check_cc_condition(opcode) {
            self.jp_imm16();
        } else {
            self.registers.increment_pc_twice();
        }
    }

    /// Jumps to the address by adding the signed 8-bit immediate value to the PC.
    /// The jump range is -128 to +127 bytes from the current position.
    /// The below logic uses 2's complement to handle negative offsets. When a number is parsed to i8 or i16, Rust automatically
    /// interprets it as a signed number in 2's complement form.
    /// Example: 0xF6 as u8 = 246
    ///          0xF6 as i8 = -10 (two's complement interpretation).
    fn jr_imm8(&mut self) {
        // Read the signed offset (PC is already at opcode + 1)
        let imm8 = self.get_imm8() as i8; // Parse to i8 to handle
        self.registers.increment_pc(); // Move past the offset byte

        // Add the signed offset to PC
        // We need to convert i8 to i16 first to handle negative numbers correctly
        self.registers.pc = (self.registers.pc as i16).wrapping_add(imm8 as i16) as u16;

        // Cycles: 12 (3 machine cycles)
        self.cycles += 12;
    }

    /// If condition cc and the flag status match, jumps -127 to +129 steps from the current address.
    /// If cc and the flag status do not match, the instruction following the current JP instruction is executed.
    fn jr_cc_imm8(&mut self, opcode: u8) {
        if self.check_cc_condition(opcode) {
            self.jr_imm8();
        }
    }
}
