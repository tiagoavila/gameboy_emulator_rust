/// The maximum address is 65,535 (decimal) = 0xFFFF (hexadecimal).
/// The memory can address 65,536 locations, which equals 64 KB.
pub const MEMORY_SIZE: usize = 0xFFFF + 1; // 0xFFFF - u16::MAX