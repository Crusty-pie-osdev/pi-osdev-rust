use core::arch::global_asm;

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;

// Assembly counterpart to this file.
global_asm!(
    include_str!("boot.S"),
    CONST_CORE_ID_MASK = const 0b11
);

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// The Rust entry of the `kernel` binary.
///
/// The function is called from the assembly `_start` function.
#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    crate::kernel_init()
}
