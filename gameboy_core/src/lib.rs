mod cpu;
mod constants;
mod cpu_components;
mod cpu_test;
pub mod prelude {
    pub use crate::cpu::*;
    pub use crate::constants::*;
    pub use crate::cpu_components::*;
}