mod cpu;
mod init;
mod opcodes;
mod state;

fn main() {
    let rom_path = "placeholder";
    let mut state = init::init_state(&rom_path);
    println!("Hello, world!");
}
