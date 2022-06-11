use chlorine::c_void;

use crate::ports::{TickType, UBaseType};

#[allow(non_snake_case)]
#[repr(C)]
pub struct StaticListItem {
    xDummy2: TickType,
    pvDummy3: [*mut c_void; 4],
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct StaticTask {
    pxDummy1: *mut c_void,

    xDummy3: [StaticListItem; 2],

    uxDummy5: UBaseType,
    pxDummy6: *mut c_void,

    ucDummy7: [u8; 16], // task name

    pxDummy8: *mut c_void, // end of stack address
    // UBaseType_t uxDummy9; if portCRITICAL_NESTING_IN_TCB
    #[cfg(feature = "trace_facility")]
    uxDummy10: [UBaseType; 2],

    #[cfg(feature = "mutex")]
    uxDummy12: [UBaseType; 2],

    pxDummy14: *mut c_void, // task tag
    pvDummy15: *mut c_void, // thread local storage
    #[cfg(feature = "runtime-stats")]
    ulDummy16: u32,
    #[cfg(feature = "task_notifications")]
    ulDummy18: [u32; 3],
    #[cfg(feature = "task_notifications")]
    ucDummy19: [u32; 3],
    #[cfg(feature = "dynamic_memory")]
    uxDummy20: u8,
}
