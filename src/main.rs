#![no_std]
#![no_main]
#![feature(stdarch_arm_hints)]
#![feature(stdarch_arm_neon_intrinsics)]

mod port;
mod sim;
mod uart;
mod watchdog;

use core::fmt::Write;

static mut UART: Option<uart::Uart> = None;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let (wdog, sim) = unsafe {
        (watchdog::Watchdog::new(), sim::Sim::new())
    };
    wdog.disable();

    // Enable clocks for Port B (UART pins), Port C (LED), and UART0
    sim.enable_clock(sim::Clock::PortB);
    sim.enable_clock(sim::Clock::PortC);
    sim.enable_clock(sim::Clock::Uart0);

    // UART0 TX on Port B pin 17.
    // Baud rate (11, 12) = 115200 at the default ~20.97 MHz FEI core clock.
    unsafe {
        let tx = port::Port::new(port::PortName::B).pin(17).make_tx();
        UART = Some(uart::Uart::new(tx, 11, 12));
    }

    let mut gpio = unsafe {
        port::Port::new(port::PortName::C).pin(5).make_gpio()
    };
    gpio.output();
    gpio.high();

    loop {
        unsafe {
            if let Some(u) = (&raw mut UART).as_mut().and_then(|o| o.as_mut()) {
                writeln!(u, "boot").ok();
            }
        }
        // ~500 ms spin delay at the default ~20.97 MHz core clock
        for _ in 0..1_000_000u32 {
            unsafe { core::arch::arm::__nop() };
        }
    }
}

// Language items
#[panic_handler]
fn teensy_panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        if let Some(u) = (&raw mut UART).as_mut().and_then(|o| o.as_mut()) {
            write!(u, "panic: ").ok();
            write!(u, "{}", info.message()).ok();
            writeln!(u).ok();
        }
    }
    loop {}
}

// static data
unsafe extern "C" {
    fn _stack_top();
}

#[unsafe(link_section = ".vectors")]
#[unsafe(no_mangle)]
pub static _VECTORS: [unsafe extern "C" fn(); 2] = [_stack_top, main];

#[unsafe(link_section = ".flashconfig")]
#[unsafe(no_mangle)]
pub static _FLASHCONFIG: [u8; 16] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xDE, 0xF9, 0xFF, 0xFF,
];
