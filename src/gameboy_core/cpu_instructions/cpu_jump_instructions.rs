use crate::gameboy_core::cpu::Cpu;

/// Trait for CPU jump instructions
pub trait CpuJumpInstructions {
    fn jp_imm16(&mut self);
    fn jp_cc_imm16(&mut self, opcode: u8);
    fn jr_imm8(&mut self);
    fn jr_cc_imm8(&mut self, opcode: u8);
    fn jp_hl(&mut self);
}

impl CpuJumpInstructions for Cpu {
    /// Loads the 16-bit immediate value to the program counter (PC).
    fn jp_imm16(&mut self) {
        self.increment_4_cycles_and_update_timers();
        let imm16 = self.get_imm16();
        self.increment_4_cycles_and_update_timers();
        self.increment_4_cycles_and_update_timers();
        self.registers.pc = imm16;
        self.increment_4_cycles_and_update_timers();
    }

    /// Loads operand nn in the PC if condition cc and the flag status match.
    /// The subsequent instruction starts at address nn.
    /// If condition cc and the flag status do not match, the contents of the PC are incremented, and the
    /// instruction following the current JP instruction is executed.
    fn jp_cc_imm16(&mut self, opcode: u8) {
        if self.check_cc_condition(opcode) {
            self.jp_imm16();
        } else {
            self.increment_4_cycles_and_update_timers();
            self.registers.increment_pc_twice();
            self.increment_4_cycles_and_update_timers();
            self.increment_4_cycles_and_update_timers();
        }
    }

    /// Jumps to the address by adding the signed 8-bit immediate value to the PC.
    /// The jump range is -128 to +127 bytes from the current position.
    /// The below logic uses 2's complement to handle negative offsets. When a number is parsed to i8 or i16, Rust automatically
    /// interprets it as a signed number in 2's complement form.
    /// Example: 0xF6 as u8 = 246
    ///          0xF6 as i8 = -10 (two's complement interpretation).
    fn jr_imm8(&mut self) {
        self.increment_4_cycles_and_update_timers();
        // Read the signed offset (PC is already at opcode + 1)
        let imm8 = self.get_imm8() as i8; // Parse to i8 to handle
        self.increment_4_cycles_and_update_timers();
        self.registers.increment_pc(); // Move past the offset byte

        // Add the signed offset to PC
        // We need to convert i8 to i16 first to handle negative numbers correctly
        self.registers.pc = (self.registers.pc as i16).wrapping_add(imm8 as i16) as u16;

        self.increment_4_cycles_and_update_timers();
    }

    /// If condition cc and the flag status match, jumps -127 to +129 steps from the current address.
    /// If cc and the flag status do not match, the instruction following the current JP instruction is executed.
    /// Note: JR cc uses bits 4-3 for the condition, different from JP cc which uses bits 5-3
    fn jr_cc_imm8(&mut self, opcode: u8) {
        // Extract condition from bits 4-3 (for JR cc instructions)
        let condition = (opcode & 0b00011000) >> 3;
        let condition_met = match condition {
            0 => !self.registers.flags.z, // NZ
            1 => self.registers.flags.z,  // Z
            2 => !self.registers.flags.c, // NC
            3 => self.registers.flags.c,  // C
            _ => false,
        };

        if condition_met {
            self.jr_imm8();
        } else {
            self.increment_4_cycles_and_update_timers();
            self.registers.increment_pc(); // Move past the offset byte
            self.increment_4_cycles_and_update_timers();
        }
    }

    /// Loads the contents of register pair HL in program counter PC.
    fn jp_hl(&mut self) {
        self.increment_4_cycles_and_update_timers();
        self.registers.pc = self.registers.get_hl();
    }
}
