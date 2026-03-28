
#[derive(Clone, Copy)]
pub enum PortName {
    B,
    C,
}

#[repr(C)]
pub struct Port {
    pcr: [u32; 32],
    gpclr: u32,
    gpchr: u32,
    reserved_0: [u8; 24],
    isfr: u32,
}

pub struct Pin {
    port: *mut Port,
    pin: usize,
}

pub struct Tx {
    _pin: Pin,
}

#[repr(C)]
struct GpioBitband {
    pdor: [u32; 32],
    psor: [u32; 32],
    pcor: [u32; 32],
    ptor: [u32; 32],
    pdir: [u32; 32],
    pddr: [u32; 32],
}

pub struct Gpio {
    gpio: *mut GpioBitband,
    pin: usize,
}

impl Port {
    pub fn name(&self) -> PortName {
        let addr = (self as *const Port) as u32;
        match addr {
            0x4004_A000 => PortName::B,
            0x4004_B000 => PortName::C,
            _ => unreachable!(),
        }
    }

    pub unsafe fn new(name: PortName) -> &'static mut Port {
        unsafe {
            &mut *match name {
                PortName::B => 0x4004_A000 as *mut Port,
                PortName::C => 0x4004_B000 as *mut Port,
            }
        }
    }

    pub unsafe fn set_pin_mode(&mut self, p: usize, mut mode: u32) {
        unsafe {
            let mut pcr = core::ptr::read_volatile(&self.pcr[p]);
            pcr &= 0xFFFFF8FF;
            mode &= 0x00000007;
            mode <<= 8;
            pcr |= mode;
            core::ptr::write_volatile(&mut self.pcr[p], pcr);
        }
    }

    pub unsafe fn pin(&mut self, p: usize) -> Pin {
        Pin { port: self, pin: p }
    }
}

impl Pin {
    pub fn make_gpio(self) -> Gpio {
        unsafe {
            let port = &mut *self.port;
            port.set_pin_mode(self.pin, 1);
            Gpio::new(port.name(), self.pin)
        }
    }

    /// Configure pin as UART0 TX. Only valid for Port B pin 17.
    pub fn make_tx(self) -> Tx {
        unsafe {
            let port = &mut *self.port;
            match (port.name(), self.pin) {
                (PortName::B, 17) => {
                    port.set_pin_mode(self.pin, 3);
                    Tx { _pin: self }
                }
                _ => panic!("Invalid UART TX pin"),
            }
        }
    }
}

impl Gpio {
    pub unsafe fn new(port: PortName, pin: usize) -> Gpio {
        let gpio = match port {
            PortName::B => 0x43FE_0800 as *mut GpioBitband,
            PortName::C => 0x43FE_1000 as *mut GpioBitband,
        };

        Gpio { gpio, pin }
    }

    pub fn output(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.gpio).pddr[self.pin], 1);
        }
    }

    pub fn high(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.gpio).psor[self.pin], 1);
        }
    }
}
