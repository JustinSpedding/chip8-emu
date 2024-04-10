#[derive(Debug, Clone, Copy)]
pub struct State {
    pub registers: [u8; 16],
    pub memory: [u8; 4096],
    pub stack: [u16; 16],
    pub keypad: [bool; 16],
    pub video: [u64; 32],
    pub index: u16,
    pub pc: u16,
    pub sp: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
}
