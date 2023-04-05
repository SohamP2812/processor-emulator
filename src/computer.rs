use std::u8;

pub const GENERAL_REGISTER_NAMES: &[&str] = &["A", "B", "C", "D", "L", "H"]; 
pub const SPECIAL_REGISTER_NAMES: &[&str] = &["PC", "SP", "F"]; 
const FLAG_NAMES: &[&str] = &["ZERO", "CARRY"]; 
const STATUS_NAMES: &[&str] = &["HALT"]; 

#[derive(Copy, Clone)]
pub struct GeneralRegister {
    pub value: u8
}

#[derive(Copy, Clone)]
pub struct SpecialRegister {
    pub value: u16
}

pub struct CPU {
    pub general_registers: [GeneralRegister; 6],
    pub special_registers: [SpecialRegister; 4]
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            general_registers: [GeneralRegister {value: 0}; 6],
            special_registers: [SpecialRegister {value: 0}; 4]
        }
    }

    pub fn hl(&self) -> u16 {
        (self.general_registers[4].value as u16) | ((self.general_registers[5].value as u16) << 0x8)
    }
}

pub struct Memory {
    pub memory: [u8; 0xFFFF]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; 0xFFFF],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
}

pub struct Computer {
    pub cpu: CPU,
    pub memory: Memory
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            cpu: CPU::new(),
            memory: Memory::new()
        }
    }

    pub fn run(&mut self, speed: u16) {
        self.cpu.special_registers[3].value &= !(1 << 0);

        loop {
            self.step();

            if self.halted() {
                break;
            }
        }
    }

    pub fn step(&mut self) {
        if self.halted() {
            return;
        }

        let instruction = self.memory.read(self.cpu.special_registers[0].value);
        self.increment_pc();
        self.execute_instruction(instruction);
    }

    pub fn execute_instruction(&mut self, instruction: u8) {
        let opcode = self.get_opcode(&instruction);

        match opcode {
            0x0 /* NOP */ => {
                
            },
            0x1 /* MOV */ => {
                let operand = self.memory.read(self.cpu.special_registers[0].value);
                self.increment_pc();

                if instruction & 0x8 != 0 { 
                    self.cpu.general_registers[(instruction & 0x7) as usize].value = self.cpu.general_registers[(operand & 0x7) as usize].value;
                } else {
                    self.cpu.general_registers[(instruction & 0x7) as usize].value = operand;
                }
            },
            0x2 /* LDR */ => {
                let address: u16;

                if instruction & 0x8 != 0 {
                    address = self.cpu.hl();
                } else {
                    let low_byte = self.memory.read(self.cpu.special_registers[0].value);
                    self.increment_pc();
                    let high_byte = self.memory.read(self.cpu.special_registers[0].value);
                    self.increment_pc();
                    address = (low_byte as u16) | ((high_byte as u16) << 0x8);
                }

                self.cpu.general_registers[(instruction & 0x7) as usize].value = self.memory.read(address);
            },
            0x3 /* STR */ => {
                let address: u16;

                if instruction & 0x8 != 0 {
                    address = self.cpu.hl();
                } else {
                    let low_byte = self.memory.read(self.cpu.special_registers[0].value);
                    self.increment_pc();
                    let high_byte = self.memory.read(self.cpu.special_registers[0].value);
                    self.increment_pc();
                    address = (low_byte as u16) | ((high_byte as u16) << 0x8);
                }

                self.memory.write(address, self.cpu.general_registers[(instruction & 0x7) as usize].value);
            },
            0x4 /* LHL */ => {
                let low_byte = self.memory.read(self.cpu.special_registers[0].value);
                self.increment_pc();
                let high_byte = self.memory.read(self.cpu.special_registers[0].value);
                self.increment_pc();

                self.cpu.general_registers[4].value = low_byte;
                self.cpu.general_registers[5].value = high_byte;
            },
            0x5 /* JMP */ => {
                let address: u16;

                if instruction & 0x8 != 0 {
                    address = self.cpu.hl();
                } else {
                    let low_byte = self.memory.read(self.cpu.special_registers[0].value);
                    self.increment_pc();
                    let high_byte = self.memory.read(self.cpu.special_registers[0].value);
                    self.increment_pc();
                    address = (low_byte as u16) | ((high_byte as u16) << 0x8);
                }

                self.cpu.special_registers[0].value = address;
            },
            0x6 /* JZ */ => {
                if self.cpu.special_registers[2].value & (1 << 0) == 0 {
                    let address: u16;

                    if instruction & 0x8 != 0 {
                        address = self.cpu.hl();
                    } else {
                        let low_byte = self.memory.read(self.cpu.special_registers[0].value);
                        self.increment_pc();
                        let high_byte = self.memory.read(self.cpu.special_registers[0].value);
                        self.increment_pc();
                        address = (low_byte as u16) | ((high_byte as u16) << 0x8);
                    }

                    self.cpu.special_registers[0].value = address;
                }
            },
            0x7 /* ADD */ => {
                let operand = self.memory.read(self.cpu.special_registers[0].value);
                self.increment_pc();
                let result;

                if instruction & 0x8 != 0 { 
                    result = self.cpu.general_registers[(instruction & 0x7) as usize].value.overflowing_add(self.cpu.general_registers[(operand & 0x7) as usize].value);
                } else {
                    result = self.cpu.general_registers[(instruction & 0x7) as usize].value.overflowing_add(operand);
                }

                self.cpu.general_registers[(instruction & 0x7) as usize].value = result.0;

                if result.1 {
                    self.cpu.special_registers[2].value |= 1 << 1;
                } else {
                    self.cpu.special_registers[2].value &= !(1 << 1);
                }
            },
            0x8 /* ADC */ => {
                let operand = self.memory.read(self.cpu.special_registers[0].value);
                self.increment_pc();
                let mut result;

                if instruction & 0x8 != 0 { 
                    result = self.cpu.general_registers[(instruction & 0x7) as usize].value.overflowing_add(self.cpu.general_registers[(operand & 0x7) as usize].value);
                } else {
                    result = self.cpu.general_registers[(instruction & 0x7) as usize].value.overflowing_add(operand);
                }
                
                if self.cpu.special_registers[2].value & (1 << 1) != 0 {
                    result = result.0.overflowing_add(1);
                } 

                self.cpu.general_registers[(instruction & 0x7) as usize].value = result.0;

                if result.1 {
                    self.cpu.special_registers[2].value |= 1 << 1;
                } else {
                    self.cpu.special_registers[2].value &= !(1 << 1);
                }
            }, 
            0x9 /* CMP */ => {
                let operand = self.memory.read(self.cpu.special_registers[0].value);
                self.increment_pc();

                let comparison;

                if instruction & 0x8 != 0 { 
                    comparison = self.cpu.general_registers[(instruction & 0x7) as usize].value - self.cpu.general_registers[(operand & 0x7) as usize].value;
                } else {
                    comparison = self.cpu.general_registers[(instruction & 0x7) as usize].value - operand;
                }

                if comparison != 0 {
                    self.cpu.special_registers[2].value &= !(1 << 0);
                } else {
                    self.cpu.special_registers[2].value |= 1 << 0;
                }
            },
            0xA /* SUB */ => {
                let operand = self.memory.read(self.cpu.special_registers[0].value);
                self.increment_pc();

                if instruction & 0x8 != 0 { 
                    self.cpu.general_registers[(instruction & 0x7) as usize].value -= self.cpu.general_registers[(operand & 0x7) as usize].value;
                } else {
                    self.cpu.general_registers[(instruction & 0x7) as usize].value -= operand;
                }
            },
            0xF /* HLT  */ => {
                self.cpu.special_registers[3].value |= 1 << 0;
            }
            _ => {}
        }
    }

    fn get_opcode(&self, instruction: &u8) -> u8 {
        (instruction & 0xF0) >> 4
    }

    fn increment_pc(&mut self) {
        self.cpu.special_registers[0].value += 1;
    }

    fn halted(&self) -> bool {
        return (self.cpu.special_registers[3].value & (1 << 0)) != 0;
    }

    pub fn load(&mut self, start_addr: u16, data: Vec<u8>) {
        for i in 0..data.len() {
            self.memory.write(start_addr + i as u16, data[i]);
        }
    }

    pub fn dump(&self) {
        println!("PC: {:#06X} SP: {:#06X}", self.cpu.special_registers[0].value, self.cpu.special_registers[1].value);

        println!();

        for i in 0..6 {
            println!("{}: {:#04X}", GENERAL_REGISTER_NAMES[i], self.cpu.general_registers[i].value);
        }

        println!();

        for i in 0..2 {
            let flag_set = self.cpu.special_registers[2].value & (1 << i) != 0;
            println!("{}: {}", FLAG_NAMES[i], if flag_set { "true" } else { "false" });
        }

        println!();

        for i in 0..1 {
            let status_set = self.cpu.special_registers[3].value & (1 << i) != 0;
            println!("{}: {}", STATUS_NAMES[i], if status_set { "true" } else { "false" });
        }
    }

    pub fn mem_dump(&self, start_addr: u16, bytes: usize) {
        for i in 0..bytes {
            print!("{:#04X} ", self.memory.read(start_addr + i as u16));
        }

        println!();
    }
}