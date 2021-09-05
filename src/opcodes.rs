#![allow(non_snake_case)]

use crate::init::FONT_SET_START_ADDRESS;
use crate::state::State;

static SCREEN_WIDTH: u8 = 64;
static SCREEN_HEIGHT: u8 = 32;

pub fn op_00E0(state: &mut State, _opcode: u16) {
    state.video.iter_mut().for_each(|row| *row = 0);
}

pub fn op_00EE(state: &mut State, _opcode: u16) {
    state.sp = state.sp.wrapping_sub(1) % 16;
    state.pc = state.stack[state.sp as usize];
}

pub fn op_1XXX(state: &mut State, opcode: u16) {
    let address = opcode & 0x0FFF;
    state.pc = address;
}

pub fn op_2XXX(state: &mut State, opcode: u16) {
    let address = opcode & 0x0FFF;
    state.stack[state.sp as usize] = state.pc;
    state.sp = state.sp.wrapping_add(1) % 16;
    state.pc = address;
}

pub fn op_3XYY(state: &mut State, opcode: u16) {
    let register = ((opcode & 0x0F00) >> 8) as usize;
    let byte = (opcode & 0x00FF) as u8;
    if state.registers[register] == byte {
        state.pc = state.pc.wrapping_add(2) % 4096;
    }
}

pub fn op_4XYY(state: &mut State, opcode: u16) {
    let register = ((opcode & 0x0F00) >> 8) as usize;
    let byte = (opcode & 0x00FF) as u8;
    if state.registers[register] != byte {
        state.pc = state.pc.wrapping_add(2) % 4096;
    }
}

pub fn op_5XY0(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    let register2 = ((opcode & 0x00F0) >> 4) as usize;
    if state.registers[register1] == state.registers[register2] {
        state.pc = state.pc.wrapping_add(2) % 4096;
    }
}

pub fn op_6XYY(state: &mut State, opcode: u16) {
    let register = ((opcode & 0x0F00) >> 8) as usize;
    let byte = (opcode & 0x00FF) as u8;
    state.registers[register] = byte;
}

pub fn op_7XYY(state: &mut State, opcode: u16) {
    let register = ((opcode & 0x0F00) >> 8) as usize;
    let byte = (opcode & 0x00FF) as u8;
    state.registers[register] = state.registers[register].wrapping_add(byte);
}

pub fn op_8XY0(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    let register2 = ((opcode & 0x00F0) >> 4) as usize;
    state.registers[register1] = state.registers[register2];
}

pub fn op_8XY1(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    let register2 = ((opcode & 0x00F0) >> 4) as usize;
    state.registers[register1] |= state.registers[register2];
}

pub fn op_8XY2(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    let register2 = ((opcode & 0x00F0) >> 4) as usize;
    state.registers[register1] &= state.registers[register2];
}

pub fn op_8XY3(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    let register2 = ((opcode & 0x00F0) >> 4) as usize;
    state.registers[register1] ^= state.registers[register2];
}

pub fn op_8XY4(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    let register2 = ((opcode & 0x00F0) >> 4) as usize;
    let sum = (state.registers[register1] as u16).wrapping_add(state.registers[register2] as u16);
    state.registers[15] = if sum > 255 { 1 } else { 0 };
    state.registers[register1 as usize] = sum as u8;
}

pub fn op_8XY5(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    let register2 = ((opcode & 0x00F0) >> 4) as usize;
    state.registers[15] = if state.registers[register1] > state.registers[register2] { 1 } else { 0 };
    state.registers[register1] = state.registers[register1].wrapping_sub(state.registers[register2]);
}

pub fn op_8XY6(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    state.registers[15] = state.registers[register1] & 1;
    state.registers[register1] >>= 1;
}

pub fn op_8XY7(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    let register2 = ((opcode & 0x00F0) >> 4) as usize;
    state.registers[15] = if state.registers[register2] > state.registers[register1] { 1 } else { 0 };
    state.registers[register1] = state.registers[register2].wrapping_sub(state.registers[register1]);
}

pub fn op_8XYE(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    state.registers[15] = (state.registers[register1] & 0x80) >> 7;
    state.registers[register1] <<= 1;
}

pub fn op_9XY0(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    let register2 = ((opcode & 0x00F0) >> 4) as usize;
    if state.registers[register1] != state.registers[register2] {
        state.pc = state.pc.wrapping_add(2) % 4096;
    }
}

pub fn op_AXXX(state: &mut State, opcode: u16) {
    let address = opcode & 0x0FFF;
    state.index = address;
}

pub fn op_BXXX(state: &mut State, opcode: u16) {
    let address = opcode & 0x0FFF;
    state.pc = (state.registers[0] as u16).wrapping_add(address) % 4096;
}

pub fn op_CXYY(state: &mut State, opcode: u16) {
    let register = ((opcode & 0x0F00) >> 8) as usize;
    let byte = (opcode & 0x00FF) as u8;
    let random_byte: u8 = rand::random();
    state.registers[register] = byte & random_byte;
}

pub fn op_DXYZ(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    let register2 = ((opcode & 0x00F0) >> 4) as usize;
    let height = (opcode & 0x000F) as usize;
    let x_pos = (state.registers[register1] % SCREEN_WIDTH) as u32;
    let y_pos = (state.registers[register2] % SCREEN_HEIGHT) as usize;

    state.registers[15] = 0;

    for row in 0..height {
        let sprite_row = state.memory[((state.index as usize).wrapping_add(row)) % 4096];
        let bits_to_flip = (sprite_row as u64).rotate_right(x_pos + 8);
        let row_index = (row + y_pos) % SCREEN_HEIGHT as usize;
        state.video[row_index] ^= bits_to_flip;
        if state.video[row_index] & bits_to_flip != bits_to_flip {
            state.registers[15] = 1;
        }
    }
}

pub fn op_EX9E(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    if state.keypad[state.registers[register1] as usize] {
        state.pc = state.pc.wrapping_add(2) % 4096;
    }
}

pub fn op_EXA1(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    if !state.keypad[state.registers[register1] as usize] {
        state.pc = state.pc.wrapping_add(2) % 4096;
    }
}

pub fn op_FX07(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    state.registers[register1] = state.delay_timer;
}

pub fn op_FX0A(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    let pressed_key = state.keypad.iter().position(|key| *key);
    match pressed_key {
        Some(key) => state.registers[register1] = key as u8,
        None => state.pc = state.pc.wrapping_sub(2) % 4096,
    }
}

pub fn op_FX15(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    state.delay_timer = state.registers[register1];
}

pub fn op_FX18(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    state.sound_timer = state.registers[register1];
}

pub fn op_FX1E(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    state.index = state.index.wrapping_add(state.registers[register1] as u16) % 4096;
}

pub fn op_FX29(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    state.index = (FONT_SET_START_ADDRESS as u16).wrapping_add(5 * state.registers[register1] as u16) % 4096;
}

pub fn op_FX33(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    let mut num = state.registers[register1];
    state.memory[((state.index.wrapping_add(2)) % 4096) as usize] = num % 10;
    num /= 10;
    state.memory[((state.index.wrapping_add(1)) % 4096) as usize] = num % 10;
    num /= 10;
    state.memory[state.index as usize] = num % 10;
}

pub fn op_FX55(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    let index = state.index as usize;
    for i in 0..register1 + 1 {
        state.memory[index.wrapping_add(i) % 4096] = state.registers[i];
    }
}

pub fn op_FX65(state: &mut State, opcode: u16) {
    let register1 = ((opcode & 0x0F00) >> 8) as usize;
    let index = state.index as usize;
    for i in 0..register1 + 1 {
        state.registers[i] = state.memory[index.wrapping_add(i) % 4096];
    }
}
