elf:
    cargo build --release

hex:
    @just elf
    arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabi/release/oongli oongli.hex

flash:
    @just hex
    teensy_loader_cli -w -mmcu=mk20dx256 oongli.hex -v


bootstrap:
    rustup component add --target thumbv7em-none-eabi rust-std
    cargo init --bin --name oongli .

