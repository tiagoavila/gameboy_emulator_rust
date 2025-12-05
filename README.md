# gameboy_emulator_rust

![Gameboy Emulator Logo](files/images/logo.png)

## Introduction

This project is a Gameboy emulator written in Rust. The goal is to accurately emulate the original Nintendo Gameboy hardware, allowing you to run classic Gameboy games on your computer. The emulator aims to be simple, educational, and a fun way to learn about emulation and low-level systems programming in Rust.

## Testing with Blargg's Test ROMs

This emulator is tested against Blargg's comprehensive CPU instruction test suite. The following individual tests are included:

- `01-special.gb` - Special operations ✅
- `02-interrupts.gb` - Interrupt handling ⏳
- `03-op sp,hl.gb` - Stack pointer and HL register operations ✅
- `04-op r,imm.gb` - Register and immediate value operations ✅
- `05-op rp.gb` - Register pair operations ✅ 
- `06-ld r,r.gb` - Register to register load operations ✅ 
- `07-jr,jp,call,ret,rst.gb` - Jump and call instructions ⏳
- `08-misc instrs.gb` - Miscellaneous instructions ✅ 
- `09-op r,r.gb` - Register to register operations ⏳
- `10-bit ops.gb` - Bit operations ⏳
- `11-op a,(hl).gb` - Accumulator and indirect HL operations ⏳
- `cpu_instrs.gb` - Complete CPU instruction test ⏳

