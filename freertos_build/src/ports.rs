use crate::Port;

pub fn cortex_m4f() -> Port {
    Port {
        target_triplet: "thumbv7em-none-eabihf".to_string(),
        sources: vec!["port.c".into()],
        directory: "ARM_CM4F".into(),
    }
}
