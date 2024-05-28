# pi-osdev-rust

## Building elf
cargo rustc --release

## Copy elf to image for raspberry pi
rust-objcopy --strip-all -O binary target/aarch64-unknown-none-softfloat/release/kernel kernel8.img

## View Assembly
cargo objdump --release -d | less

## Running in QEMU
qemu-system-aarch64 -M raspi3ap -serial stdio -kernel kernel8.img