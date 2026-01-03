#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn main() {
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
