use std::u8;

#[derive(Copy, Clone)]
pub struct GeneralRegister {
    pub value: u8
}

pub struct SpecialRegister {
    pub value: u16
}

pub struct CPU {
    pub general_registers: [GeneralRegister; 6],
    pub special_registers: [SpecialRegister; 1]
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            general_registers: [GeneralRegister {value: 0}; 6],
            special_registers: [SpecialRegister {value: 0}; 1]
        }
    }

    pub fn hl(&self) -> u16 {
        (self.general_registers[4].value as u16) | ((self.general_registers[5].value as u16) << 0x8)
    }
}

pub struct Memory {
    pub memory: [u8; 0xFF]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; 0xFF],
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

    pub fn step(&mut self) {
        let data = self.memory.read(self.cpu.special_registers[0].value);
        self.increment_pc();
    }

    fn execute_instruction(&mut self, instruction: u8) {
        let opcode = self.get_opcode(&instruction);

        match opcode {
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