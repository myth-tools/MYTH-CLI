use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};

/// Global Sovereign Mission Abort signal.
/// Synchronizes termination across CLI signal handlers and TUI key events.
/// Uses SeqCst ordering to guarantee immediate cross-thread visibility.
pub static MISSION_ABORT: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

pub fn is_aborted() -> bool {
    MISSION_ABORT.load(Ordering::SeqCst)
}

pub fn abort_mission() {
    MISSION_ABORT.store(true, Ordering::SeqCst);
}

pub fn reset_mission_signal() {
    MISSION_ABORT.store(false, Ordering::SeqCst);
}
