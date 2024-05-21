# pi-osdev-rust

## Building elf
RUSTFLAGS="-C target-cpu=cortex-a76 -C link-arg=--library-path=src/ -C link-arg=--script=linker.ld" cargo rustc --target=aarch64-unknown-none-softfloat --release

## Copy elf to image for raspberry pi
rust-objcopy --strip-all -O binary target/aarch64-unknown-none-softfloat/release/kernel kernel8.img

## View Assembly
RUSTFLAGS="-C target-cpu=cortex-a76 -C link-arg=--library-path=/home/pizza-man/Desktop/pi-osdev-rust/kernel/src -C link-arg=--script=linker.ld" cargo objdump --release -- -d | less

## Running in QEMU
qemu-system-aarch64 -M raspi3ap -serial stdio -kernel kernel8.img