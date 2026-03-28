use bit_field::BitField;
use volatile::Volatile;

use super::port::Tx;

#[repr(C)]
struct UartRegs {
    bdh: Volatile<u8>,
    bdl: Volatile<u8>,
    c1: Volatile<u8>,
    c2: Volatile<u8>,
    s1: Volatile<u8>,
    s2: Volatile<u8>,
    c3: Volatile<u8>,
    d: Volatile<u8>,
    ma1: Volatile<u8>,
    ma2: Volatile<u8>,
    c4: Volatile<u8>,
}

pub struct Uart {
    reg: &'static mut UartRegs,
    _tx: Tx,
}

impl Uart {
    /// Create a UART0 instance.
    ///
    /// `sbr` and `brfa` are the baud rate divider and fractional adjust.
    /// At the default ~20.97 MHz core clock (FEI/slow IRC), use `(11, 12)`
    /// for 115200 baud.
    pub unsafe fn new(tx: Tx, sbr: u16, brfa: u8) -> Uart {
        let reg = unsafe { &mut *(0x4006_A000 as *mut UartRegs) };

        reg.c4.update(|c4| {
            c4.set_bits(0..5, brfa);
        });
        reg.bdh.update(|bdh| {
            bdh.set_bits(0..5, sbr.get_bits(8..13) as u8);
        });
        reg.bdl.write(sbr.get_bits(0..8) as u8);

        // Enable transmitter
        reg.c2.update(|c2| {
            c2.set_bit(3, true);
        });

        Uart { reg, _tx: tx }
    }
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            // Wait for TX Data Register Empty (TDRE, bit 7 of S1)
            while !self.reg.s1.read().get_bit(7) {}
            self.reg.d.write(b);
        }
        // Wait for Transmission Complete (TC, bit 6 of S1)
        while !self.reg.s1.read().get_bit(6) {}
        Ok(())
    }
}
