pub use core::sync::atomic;

pub mod spin_lock;
pub use spin_lock::SpinLock;

pub mod rw_spin_lock;
pub use rw_spin_lock::RwSpinLock;

