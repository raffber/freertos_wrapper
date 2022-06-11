#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m_rt::entry;
use freertos::run_scheduler;

#[entry]
fn main() -> ! {
    run_scheduler();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
