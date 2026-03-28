use std::io::Read;
use std::time::{Duration, Instant};

/// Flash the firmware and assert the board prints "boot\n" over UART.
///
/// Requires:
///   - `teensy_loader_cli` on PATH
///   - Teensy 3.2 connected (in bootloader or auto-reset mode)
///   - Serial adapter on TEENSY_PORT (default /dev/ttyACM0) at 115200 8N1
///
/// Run via: `just test` (or `just test port=/dev/ttyUSB0` for a different port)
#[test]
fn prints_boot() {
    let port_name =
        std::env::var("TEENSY_PORT").unwrap_or_else(|_| "/dev/ttyACM0".to_string());
    let hex_path = concat!(env!("CARGO_MANIFEST_DIR"), "/oongli.hex");

    // Flash the firmware; -w waits for the bootloader, then flashes and resets.
    let status = std::process::Command::new("teensy_loader_cli")
        .args(["-s", "-v", "-w", "-mmcu=mk20dx256", hex_path])
        .status()
        .expect("teensy_loader_cli not found — run `just bootstrap` first");
    assert!(status.success(), "teensy_loader_cli exited with {status}");

    // Poll until the serial port enumerates after the post-flash reset (up to 10 s).
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut port = loop {
        match serialport::new(&port_name, 115_200)
            .timeout(Duration::from_secs(3))
            .open()
        {
            Ok(p) => break p,
            Err(_) if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => panic!("serial port {port_name} not available: {e}"),
        }
    };

    // Read byte-by-byte until we collect a full "boot\n" line (up to 5 s).
    // The firmware prints this in a loop, so we will always catch at least one.
    let mut buf = Vec::<u8>::new();
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut byte = [0u8; 1];
    loop {
        match port.read(&mut byte) {
            Ok(1) => {
                buf.push(byte[0]);
                if buf.ends_with(b"boot\n") {
                    break;
                }
            }
            Ok(_) | Err(_) => {}
        }
        assert!(
            Instant::now() < deadline,
            "timed out waiting for 'boot\\n'; received so far: {:?}",
            String::from_utf8_lossy(&buf)
        );
    }

    let line: &[u8] = &buf[buf.len() - 5..]; // last 5 bytes = "boot\n"
    assert_eq!(line, b"boot\n");
}
