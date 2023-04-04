use std::u8;

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
    pub special_registers: [SpecialRegister; 3]
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            general_registers: [GeneralRegister {value: 0}; 6],
            special_registers: [SpecialRegister {value: 0}; 3]
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
        for i in 0..2 {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let instruction = self.memory.read(self.cpu.special_registers[0].value);
        self.increment_pc();
        self.execute_instruction(instruction);
    }

    pub fn execute_instruction(&mut self, instruction: u8) {
        let opcode = self.get_opcode(&instruction);

        match opcode {
            0x0 /* MOV */ => {
                let operand = self.memory.read(self.cpu.special_registers[0].value);
                self.increment_pc();

                if instruction & 0x8 != 0 { 
                    self.cpu.general_registers[(instruction & 0x7) as usize].value = self.cpu.general_registers[(operand & 0x7) as usize].value;
                } else {
                    self.cpu.general_registers[(instruction & 0x7) as usize].value = operand;
                }
            },
            0x1 /* LDR */ => {
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
            0x2 /* STR */ => {
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
            0x3 /* LHL */ => {
                let low_byte = self.memory.read(self.cpu.special_registers[0].value);
                self.increment_pc();
                let high_byte = self.memory.read(self.cpu.special_registers[0].value);
                self.increment_pc();

                self.cpu.general_registers[4].value = low_byte;
                self.cpu.general_registers[5].value = high_byte;
            },
            0x4 /* JMP */ => {
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
            0x5 /* JZ */ => {
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
            0x6 /* ADD */ => {
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
            0x7 /* ADC */ => {
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
            0x8 /* CMP */ => {
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
            0x9 /* SUB */ => {
                let operand = self.memory.read(self.cpu.special_registers[0].value);
                self.increment_pc();

                if instruction & 0x8 != 0 { 
                    self.cpu.general_registers[(instruction & 0x7) as usize].value -= self.cpu.general_registers[(operand & 0x7) as usize].value;
                } else {
                    self.cpu.general_registers[(instruction & 0x7) as usize].value -= operand;
                }
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
}