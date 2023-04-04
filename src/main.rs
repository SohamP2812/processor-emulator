use computer::Computer;

mod computer;

fn main() {
    let computer = Computer::new();
    run(computer, 1);
}

fn run(mut computer: Computer, speed: u16) {
    computer.memory.write(0, 0b0000_0_000);
    computer.memory.write(1, 0b00000110);
    computer.memory.write(2, 0b0110_0_000);
    computer.memory.write(3, 0b00000001);
    computer.run(10);
    print!("{}", computer.cpu.general_registers[0].value);
}   