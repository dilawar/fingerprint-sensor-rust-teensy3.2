# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Bare-metal Rust firmware for a fingerprint sensor on the Teensy 3.2 (Freescale Kinetis MK20DX256, ARM Cortex-M4). The crate is named `oongli`. It implements hardware abstraction layers directly against memory-mapped registers — no HAL crates, no RTOS.

## Build Commands

```sh
just elf       # cargo build --release (cross-compiles to thumbv7em-none-eabi)
just hex       # produces oongli.hex (Intel HEX via arm-none-eabi-objcopy)
just flash     # builds hex and flashes via teensy_loader_cli
just check     # cargo clippy --all-targets -- -D warnings
just fix       # auto-apply clippy fixes
just bootstrap # one-time dev setup: installs Rust target, udev rules, teensy_loader_cli, binutils
```

The `teensy/` subdirectory is an older, more feature-complete implementation with its own `Makefile` (`make` / `make flash`).

## Architecture

**Two parallel implementations exist:**

- `src/` — the active, simplified implementation using Rust edition 2024 nightly features
- `teensy/` — the older, richer implementation; useful as reference for UART, OSC, MCG drivers

Both follow the same pattern: each peripheral is a module that maps a raw pointer to a fixed hardware address and exposes safe-ish methods wrapping `volatile` reads/writes.

**Startup sequence** (`src/main.rs`):
1. Disable watchdog (`watchdog::disable()`)
2. Enable SIM clock gate for Port C (`sim::enable_port_c_clock()`)
3. Configure Port C pin 5 as GPIO output
4. Blink LED in loop

**Peripheral modules:**
- `watchdog` — unlock and disable the COP watchdog (required before any other init)
- `sim` — System Integration Module; enables peripheral clocks and sets core/bus/flash clock dividers
- `port` — pin mux and GPIO direction control

The `teensy/` version additionally has `osc` (crystal oscillator), `mcg` (PLL/clock mode state machine), and `uart` (implements `core::fmt::Write` for panic output).

## Key Technical Details

- **Target:** `thumbv7em-none-eabi` (set in `.cargo/config.toml`)
- **Linker script:** `layout.ld` — places `.vectors` at 0x0, flash config at 0x400, then `.text`/`.rodata` in 256KB FLASH; 64KB RAM from 0x1FFF8000
- **No `std`**, no allocator — `#![no_std]`, `#![no_main]`
- **Dependencies:** `volatile = "0.2"` for register access; `bit_field = "0.7"` for bitfield manipulation
- **Clock target (teensy/):** 72MHz core via 16MHz crystal + PLL (ratio 27/6), bus 36MHz, flash 24MHz
- Flash security bytes (FSEC=0xDE, FOPT=0xF9) are embedded in the binary at offset 0x400
