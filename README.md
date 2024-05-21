# pi-osdev-rust

RUSTFLAGS="-C target-cpu=cortex-a76 -C link-arg=--library-path=src/ -C link-arg=--script=linker.ld" cargo rustc --target=aarch64-unknown-none-softfloat --release

rust-objcopy --strip-all -O binary target/aarch64-unknown-none-softfloat/release/kernel kernel8.img