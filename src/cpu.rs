use super::instructions;
use super::memory::{array_to_idx, Memory, ADDR_SIZE};

pub struct CPU {
    ip: [u8; ADDR_SIZE],
    reg: u8,
    flag: bool,
    memory: Memory,
}

impl CPU {
    pub fn new(start: Vec<u8>) -> CPU {
        CPU {
            ip: [0; ADDR_SIZE],
            reg: 0,
            flag: false,
            memory: Memory::new(start),
        }
    }

    fn increment_ip(&mut self) {
        let mut overflow;
        for i in (0..ADDR_SIZE).rev() {
            (self.ip[i], overflow) = self.ip[i].overflowing_add(1);
            if !overflow {
                break;
            }
        }
        /*
        (self.ip[1], overflow) = self.ip[1].overflowing_add(1);
        if overflow {
            self.ip[0] = self.ip[0].wrapping_add(1);
        } */
    }

    fn read_next(&mut self) -> u8 {
        let result = self.memory.read(self.ip);
        self.increment_ip();
        result
    }

    fn read_addr(&mut self) -> [u8; ADDR_SIZE] {
        let result = self.memory.read_addr(self.ip);
        for _ in 0..ADDR_SIZE {
            self.increment_ip();
        }
        result
    }

    fn read_at(&self, address: [u8; ADDR_SIZE]) -> u8 {
        self.memory.read(address)
    }

    fn write_at(&mut self, address: [u8; ADDR_SIZE], value: u8) {
        self.memory.write(address, value);
    }

    fn jump_if(&mut self, condition: bool) {
        let address = self.read_addr();
        if condition {
            self.ip = address;
        }
        self.flag = false;
    }

    pub fn execute(&mut self) -> bool {
        match self.read_next() {
            instructions::END => {
                return false;
            }
            instructions::GET => {
                let address = self.read_addr();
                self.reg = self.read_at(address);
                self.flag = false;
            }
            instructions::SET => {
                let address = self.read_addr();
                self.write_at(address, self.reg);
                self.flag = false;
            }
            instructions::VAL => {
                self.reg = self.read_next();
                self.flag = false;
            }
            instructions::ADD_LIT => {
                let value = self.read_next();
                (self.reg, self.flag) = self.reg.overflowing_add(value);
            }
            instructions::SUB_LIT => {
                let value = self.read_next();
                (self.reg, self.flag) = self.reg.overflowing_sub(value);
            }
            instructions::SBF_LIT => {
                let value = self.read_next();
                (self.reg, self.flag) = value.overflowing_sub(self.reg);
            }
            instructions::MUL_LIT => {
                let value = self.read_next();
                (self.reg, self.flag) = self.reg.overflowing_mul(value);
            }
            instructions::AND_LIT => {
                let value = self.read_next();
                self.reg &= value;
                self.flag = false;
            }
            instructions::ORB_LIT => {
                let value = self.read_next();
                self.reg |= value;
                self.flag = false;
            }
            instructions::XOR_LIT => {
                let value = self.read_next();
                self.reg ^= value;
                self.flag = false;
            }
            instructions::ADD_MEM => {
                let address = self.read_addr();
                let value = self.read_at(address);
                (self.reg, self.flag) = self.reg.overflowing_add(value);
            }
            instructions::SUB_MEM => {
                let address = self.read_addr();
                let value = self.read_at(address);
                (self.reg, self.flag) = self.reg.overflowing_sub(value);
            }
            instructions::SBF_MEM => {
                let address = self.read_addr();
                let value = self.read_at(address);
                (self.reg, self.flag) = value.overflowing_sub(self.reg);
            }
            instructions::MUL_MEM => {
                let address = self.read_addr();
                let value = self.read_at(address);
                (self.reg, self.flag) = self.reg.overflowing_mul(value);
            }
            instructions::AND_MEM => {
                let address = self.read_addr();
                let value = self.read_at(address);
                self.reg &= value;
                self.flag = false;
            }
            instructions::ORB_MEM => {
                let address = self.read_addr();
                let value = self.read_at(address);
                self.reg |= value;
                self.flag = false;
            }
            instructions::XOR_MEM => {
                let address = self.read_addr();
                let value = self.read_at(address);
                self.reg ^= value;
                self.flag = false;
            }
            instructions::INC => {
                (self.reg, self.flag) = self.reg.overflowing_add(1);
            }
            instructions::DEC => {
                (self.reg, self.flag) = self.reg.overflowing_sub(1);
            }
            instructions::NOT => {
                self.reg = !self.reg;
            }
            instructions::JMP => self.jump_if(true),
            instructions::JFL => self.jump_if(self.flag),
            instructions::JNF => self.jump_if(!self.flag),
            instructions::JZE => self.jump_if(self.reg == 0),
            instructions::JNZ => self.jump_if(self.reg != 0),
            _ => (),
        };
        true
    }

    pub fn print_info(&self, address: usize, offset: usize) {
        print!("ip: {:#06x}   ", array_to_idx(self.ip));
        print!("reg: {:02x}   ", self.reg);
        print!("flag: {}   ", self.flag as u8);
        print!(
            "memory at {:#06x}: {}",
            address,
            self.memory.read_after(address, offset)
        );
        println!();
    }
}
