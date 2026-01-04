#![no_std]
#![no_main]
#![feature(stdarch_arm_hints)]
#![feature(stdarch_arm_neon_intrinsics)]

mod port;
mod sim;
mod watchdog;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let (wdog, sim, pin) = unsafe {
        (
            watchdog::Watchdog::new(),
            sim::Sim::new(),
            port::Port::new(port::PortName::C).pin(5),
        )
    };
    wdog.disable();
    sim.enable_clock(sim::Clock::PortC);

    let mut gpio = pin.make_gpio();
    gpio.output();
    gpio.high();

    loop {}
}

// Language items
#[panic_handler]
fn teensy_panic(_info: &core::panic::PanicInfo) -> ! {
    // TODO: print panic info
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
