use crate::Port;

pub fn cortex_m4f() -> Port {
    Port {
        target_triplet: "arm-none-eabi".to_string(),
        sources: vec!["port.c".to_string()],
        directory: "GCC/ARM_CM4F".to_string(),
    }
}
