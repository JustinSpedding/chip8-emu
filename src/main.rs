mod cpu;
mod init;
mod opcodes;
mod state;

fn main() {
    let mut state = init::init_state();
    println!("Hello, world!");
}
