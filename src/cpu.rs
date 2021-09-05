use crate::opcodes;
use crate::state::State;

static OPCODE_TABLE: [fn(&mut State, u16); 16] = [
    op_table_0,
    opcodes::op_1XXX,
    opcodes::op_2XXX,
    opcodes::op_3XYY,
    opcodes::op_4XYY,
    opcodes::op_5XY0,
    opcodes::op_6XYY,
    opcodes::op_7XYY,
    op_table_8,
    opcodes::op_9XY0,
    opcodes::op_AXXX,
    opcodes::op_BXXX,
    opcodes::op_CXYY,
    opcodes::op_DXYZ,
    op_table_e,
    op_table_f,
];

static OPCODE_TABLE_0: [fn(&mut State, u16); 16] = [
    opcodes::op_00E0,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    opcodes::op_00EE,
    op_none,
];

static OPCODE_TABLE_8: [fn(&mut State, u16); 16] = [
    opcodes::op_8XY0,
    opcodes::op_8XY1,
    opcodes::op_8XY2,
    opcodes::op_8XY3,
    opcodes::op_8XY4,
    opcodes::op_8XY5,
    opcodes::op_8XY6,
    opcodes::op_8XY7,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    opcodes::op_8XYE,
    op_none,
];

static OPCODE_TABLE_E: [fn(&mut State, u16); 16] = [
    op_none,
    opcodes::op_EXA1,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    opcodes::op_EX9E,
    op_none,
];

static OPCODE_TABLE_F: [fn(&mut State, u16); 102] = [
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    opcodes::op_FX07,
    op_none,
    op_none,
    opcodes::op_FX0A,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    opcodes::op_FX15,
    op_none,
    op_none,
    opcodes::op_FX18,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    opcodes::op_FX1E,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    opcodes::op_FX29,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    opcodes::op_FX33,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    opcodes::op_FX55,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    op_none,
    opcodes::op_FX65,
];

fn op_none(_state: &mut State, opcode: u16) {
    // TODO: what should we do here?
    panic!("Invalid opcode: {:#06x}", opcode);
}

fn op_table_0(state: &mut State, opcode: u16) {
    let index = (opcode & 0x000F) as usize;
    OPCODE_TABLE_0[index](state, opcode);
}

fn op_table_8(state: &mut State, opcode: u16) {
    let index = (opcode & 0x000F) as usize;
    OPCODE_TABLE_8[index](state, opcode);
}

fn op_table_e(state: &mut State, opcode: u16) {
    let index = (opcode & 0x000F) as usize;
    OPCODE_TABLE_E[index](state, opcode);
}

fn op_table_f(state: &mut State, opcode: u16) {
    let index = (opcode & 0x00FF) as usize;
    if index > OPCODE_TABLE_F.len() {
        op_none(state, opcode);
    } else {
        OPCODE_TABLE_F[index](state, opcode);
    }
}

fn run_opcode(state: &mut State, opcode: u16) {
    let index = ((opcode & 0xF000) >> 12) as usize;
    OPCODE_TABLE[index](state, opcode);
}

pub fn run_cycle(state: &mut State, cycles_per_clock: u8) {
    for _ in 0..cycles_per_clock {
        let opcode = ((state.memory[state.pc as usize] as u16) << 8) | state.memory[(state.pc.wrapping_add(1) % 4096) as usize] as u16;
        state.pc = state.pc.wrapping_add(2) % 4096;
        run_opcode(state, opcode);
    }
    if state.delay_timer > 0 {
        state.delay_timer -= 1;
    }
    if state.sound_timer > 0 {
        state.sound_timer -= 1;
    }
}
