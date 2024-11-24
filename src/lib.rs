#![no_std]

pub mod include;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "system" fn driver_entry() -> u32 {
    0 /* STATUS_SUCCESS */
}
