use crate::gameboy_core::{cpu_components::MemoryBus, interrupts::InterruptType};

pub struct Timer {
    /// Number of cycles executed since last increment for the DIV register
    pub cycles_executed_div: u16,
    /// Number of cycles executed since last increment for the TIMA register
    pub cycles_executed_tima: u16,
    pub tima_overflowed: bool,
}

pub enum InterruptRequested {
    Yes,
    No,
}

impl Timer {
    pub(crate) fn new() -> Self {
        Self {
            cycles_executed_div: 0,
            cycles_executed_tima: 0,
            tima_overflowed: false,
        }
    }

    /// Update the DIV and TIMA registers based on the number of cycles executed since the last instruction.
    /// Sets the IF register timer interrupt flag if TIMA overflows.
    pub fn update(&mut self, cycles_before: u64, cycles_after: u64, memory: &mut MemoryBus) {
        let cycles_of_last_instruction: u8 = (cycles_after - cycles_before) as u8;
        self.update_div(cycles_of_last_instruction, memory);
        self.update_tima(cycles_of_last_instruction, memory);
    }

    /// The Divider Register (DIV) increments at a rate of 16384 Hz.
    /// Therefore, it increments every 256 CPU cycles, because the CPU runs at 4.194304 MHz.
    /// The math is 4,194,304 Hz / 16,384 Hz = 256 cycles.
    /// Update the DIV register based on the number of cycles executed since the last instruction.
    /// If total cycles exceed 256, increment DIV and reset the cycle counter.
    fn update_div(&mut self, cycles_of_last_instruction: u8, memory: &mut MemoryBus) {
        let total_cycles = self.cycles_executed_div + cycles_of_last_instruction as u16;

        if total_cycles >= 256 {
            let mut div = memory.get_div_register();
            div = div.wrapping_add(1);
            self.cycles_executed_div = total_cycles - 256;
            memory.set_div_register(div);
        } else {
            self.cycles_executed_div = total_cycles;
        }
    }

    /// Increment the TIMA register every n cycles (where n is determined by bits 1-0 of the TAC register).
    /// If TIMA overflows (since it's an u8 register it means going from 0xFF to 0x00), it is reset to the value specified in TMA register
    /// and an interrupt is requested.
    ///
    /// *When TIMA overflows, the value from TMA is copied, and the timer flag is set in IF, **but one M-cycle later (4 T-cycles).**
    /// This means that TIMA is equal to $00 for the M-cycle after it overflows.*
    fn update_tima(&mut self, cycles_of_last_instruction: u8, memory: &mut MemoryBus) {
        if self.tima_overflowed {
            let tma = memory.get_tma_register();
            memory.set_tima_register(tma);
            memory.update_timer_flag_in_if_register(InterruptType::Timer, true);
            self.tima_overflowed = false;
            return;
        }

        let tac = memory.get_tac_register();
        let timer_enabled = (tac & 0b00000100) != 0;
        if !timer_enabled {
            return;
        }

        let input_clock_select = tac & 0b00000011;
        let tima_increment_threshold = Self::get_tima_increment_threshould(input_clock_select);

        let total_cycles = self.cycles_executed_tima + cycles_of_last_instruction as u16;

        if total_cycles >= tima_increment_threshold {
            let mut tima = memory.get_tima_register();

            self.cycles_executed_tima = total_cycles - tima_increment_threshold;

            let (increment_result, tima_overflowed) = tima.overflowing_add(1);

            if tima_overflowed {
                tima = 0;
                self.tima_overflowed = true;
            } else {
                tima = increment_result;
            }

            memory.set_tima_register(tima);
        } else {
            self.cycles_executed_tima = total_cycles;
        }
    }

    /// Get the threshold of cycles for TIMA increment based on the TAC input clock select bits.
    /// The thresholds are returned in terms of Master Clock cycles (4.194304 MHz).
    /// - 00: 4096 Hz -> 1024 cycles
    /// - 01: 262144 Hz -> 16 cycles
    /// - 10: 65536 Hz -> 64 cycles
    /// - 11: 16384 Hz -> 256 cycles
    fn get_tima_increment_threshould(input_clock_select: u8) -> u16 {
        match input_clock_select {
            0b00 => 1024, // 4096 Hz - 4,194,304 Hz / 4,096 Hz = 1024 cycles
            0b01 => 16,   // 262144 Hz - 4,194,304 Hz / 262,144 Hz = 16 cycles
            0b10 => 64,   // 65536 Hz - 4,194,304 Hz / 65,536 Hz = 64 cycles
            0b11 => 256,  // 16384 Hz - 4,194,304 Hz / 16,384 Hz = 256 cycles
            _ => 256,     // Default case (should not occur)
        }
    }
}
