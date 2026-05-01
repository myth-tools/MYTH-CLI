//! Animation system — zero-allocation frame-driven state machine.
//!
//! All animation values are derived from a monotonic frame counter,
//! ensuring zero heap allocation and deterministic behavior.

/// Zero-allocation animation state.
/// All visual effects are computed from the monotonic `frame` counter.
pub struct AnimationState {
    /// Monotonic frame counter (wraps at u64::MAX, effectively never).
    frame: u64,
}

/// Premium Cylinder Spinner — 3D-simulated Braille rotation.
const SPINNER_CHARS: &[char] = &['⢹', '⢺', '⢼', '⣸', '⣇', '⡧', '⡗', '⡏'];

/// Premium Pulsing Heartbeat — High-fidelity state transitions.
const HEARTBEAT_CHARS: &[char] = &['◆', '◈', '◇', ' '];

/// High-fidelity scanning bar with variable block density.
const SCAN_BAR: &[&str] = &[
    "░▒▓█▓▒░",
    "▒▓█▓▒░ ",
    "▓█▓▒░  ",
    "█▓▒░   ",
    "▓▒░    ",
    "▒░     ",
    "░      ",
    "       ",
    "      ░",
    "     ░▒",
    "    ░▒▓",
    "   ░▒▓█",
    "  ░▒▓█▓",
    " ░▒▓█▓▒",
];

impl AnimationState {
    /// Create a new animation state starting at frame 0.
    pub fn new() -> Self {
        Self { frame: 0 }
    }

    /// Advance the animation by one frame. Call once per render tick.
    pub fn tick(&mut self) {
        self.frame = self.frame.wrapping_add(1);
    }

    /// Current frame number.
    pub fn frame(&self) -> u64 {
        self.frame
    }

    /// Spinner character for thinking/processing indicators.
    /// Rotates every 3 frames for smooth ~20fps appearance at 60fps tick.
    pub fn spinner_char(&self) -> char {
        let idx = (self.frame / 3) as usize % SPINNER_CHARS.len();
        SPINNER_CHARS[idx]
    }

    /// Heartbeat symbol — faster bio-rhythmic pulse (~10 frames/state).
    pub fn heartbeat_symbol(&self) -> char {
        let idx = (self.frame / 10) as usize % HEARTBEAT_CHARS.len();
        HEARTBEAT_CHARS[idx]
    }

    /// Pulse brightness as 0.0–1.0 "breathing" wave.
    /// Asymmetric: Inhales for 1/3 of the period, exhales for 2/3.
    /// Period is ~120 frames (~2s at 60fps).
    pub fn pulse_brightness(&self) -> f32 {
        let period = 120.0;
        let phase = (self.frame % period as u64) as f32 / period;

        // Asymmetric wave: faster rise, slower fall
        if phase < 0.33 {
            // Inhale (Rise): 0.0 -> 1.0 in 1/3 time
            (phase / 0.33 * std::f32::consts::PI / 2.0).sin()
        } else {
            // Exhale (Fall): 1.0 -> 0.0 in 2/3 time
            ((phase - 0.33) / 0.67 * std::f32::consts::PI / 2.0).cos()
        }
    }

    /// Scanning bar string for activity indication.
    /// Cycles through the scanning pattern every 4 frames.
    pub fn scan_bar(&self) -> &'static str {
        let idx = (self.frame / 4) as usize % SCAN_BAR.len();
        SCAN_BAR[idx]
    }

    /// Returns true for N frames out of every M frames.
    /// Useful for blink effects. E.g., `blink(10, 40)` = on for 10, off for 30.
    pub fn blink(&self, on_frames: u64, period: u64) -> bool {
        (self.frame % period) < on_frames
    }
}

impl Default for AnimationState {
    fn default() -> Self {
        Self::new()
    }
}
