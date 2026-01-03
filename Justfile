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
    @just install_teensy_udev_rules
    @just install_teensy_loader_cli
    sudo apt install -y binutils-arm-none-eabi || echo "installation failed"
    cargo init --bin --name oongli .

install_teensy_loader_cli:
    #!/usr/bin/env bash
    sudo apt -y install libusb-dev || echo "installation failed"
    rm -rf /tmp/teensy_loader_cli
    cd /tmp/ && git clone https://github.com/PaulStoffregen/teensy_loader_cli
    cd /tmp/teensy_loader_cli && make && mv ./teensy_loader_cli ~/.local/bin/
    ~/.local/bin/teensy_loader_cli --list-mcus

install_teensy_udev_rules:
    cd /tmp && wget https://www.pjrc.com/teensy/00-teensy.rules
    sudo cp /tmp/00-teensy.rules /etc/udev/rules.d/00-teensy.rules
    echo "recoonect your board"
