//! Shared code for all Teensy 4 demos
//!
//! This includes the panic handler.

#![no_std]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("PANIC! {}", info);
    teensy4_panic::sos()
}
