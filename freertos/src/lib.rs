#![no_std]

mod ffi;
mod ports;

use chlorine::{c_char, c_int, c_void};
use ffi::StaticTask;
use ports::StackType;

extern "C" {
    fn vTaskStartScheduler();
}

pub fn run_scheduler() -> ! {
    unsafe {
        vTaskStartScheduler();
    }
    loop {}
}

#[no_mangle]
extern "C" fn rust_rtos_panic(_line: c_int, _file: *const c_char) {
    loop {}
}

#[no_mangle]
extern "C" fn vApplicationGetIdleTaskMemory(
    _ppx_idle_task_tcbbuffer: *mut *mut StaticTask,
    _ppx_idle_task_stack_buffer: *mut *mut StackType,
    _pul_idle_task_stack_size: *mut u32,
) {
    todo!()
}
