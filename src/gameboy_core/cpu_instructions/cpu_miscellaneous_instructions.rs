
pub trait CpuMiscellaneousInstructions {
    fn ei(&mut self);
    fn nop(&mut self);
    fn di(&mut self);
    fn ccf(&mut self);
}
impl CpuMiscellaneousInstructions for crate::gameboy_core::cpu::Cpu {
    /// No Operation - Do nothing for one CPU cycle.
    fn nop(&mut self) {
        return;
    }

    /// This instruction disables interrupts but not immediately. Interrupts are disabled after instruction after DI is executed.
    fn di(&mut self) {
        self.di_instruction_pending = true;
    }
    
    fn ei(&mut self) {
        self.ei_instruction_pending = true;
    }

    /// Flips the carry flag CY. H and N flags are reset.
    fn ccf(&mut self) {
        self.registers.flags.c = !self.registers.flags.c;
        self.registers.flags.h = false;
        self.registers.flags.n = false;
    }
}