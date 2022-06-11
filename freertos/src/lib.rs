#![no_std]

extern "C" {
    fn vTaskStartScheduler();
}

pub fn run_scheduler() -> ! {
    unsafe {
        vTaskStartScheduler();
    }
    loop {}
}
