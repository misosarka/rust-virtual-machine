use super::instructions;
use super::memory::Memory;

const STACK_START: u32 = 0x10000000;
const STACK_LIMIT: u32 = 0x20000000 - 0x10;

pub struct CPU {
    ip: u32,
    reg: u32,
    ar: u32,
    sp: u32,
    fp: u32,
    flag: bool,
    memory: Memory,
}

impl CPU {
    pub fn new(start: &[u32]) -> CPU {
        CPU {
            ip: 0,
            reg: 0,
            ar: 0,
            sp: STACK_START,
            fp: STACK_START,
            flag: false,
            memory: Memory::new(start),
        }
    }

    fn read_next(&mut self) -> u32 {
        let result = self.memory.read(self.ip);
        self.ip = self.ip.wrapping_add(1);
        result
    }

    fn jump_if(&mut self, condition: bool) {
        let address = self.read_next();
        if condition {
            self.ip = address;
        }
        self.flag = false;
    }

    fn push(&mut self, value: u32) {
        self.memory.write(self.sp, value);
        self.sp = self.sp.wrapping_add(1);
        if self.sp > STACK_LIMIT {
            panic!("Error: stack overflow");
        }
    }

    fn pull(&mut self) -> u32 {
        self.sp = self.sp.wrapping_sub(1);
        self.memory.read(self.sp)
    }

    pub fn execute(&mut self) -> bool {
        match self.read_next() {
            instructions::END => {
                return false;
            }

            instructions::MOV_LIT_REG => {
                self.reg = self.read_next();
            }
            instructions::MOV_LIT_AR => {
                self.ar = self.read_next();
            }
            instructions::PSH_LIT => {
                let value = self.read_next();
                self.push(value);
            }

            instructions::MOV_MEM_REG => {
                let address = self.read_next();
                self.reg = self.memory.read(address);
            }
            instructions::MOV_MEM_AR => {
                let address = self.read_next();
                self.ar = self.memory.read(address);
            }

            instructions::MOV_REG_MEM => {
                let address = self.read_next();
                self.memory.write(address, self.reg);
            }
            instructions::MOV_REG_AR => {
                self.ar = self.reg;
            }
            instructions::MOV_REG_AAR => {
                self.memory.write(self.ar, self.reg);
            }
            instructions::PSH_REG => {
                self.push(self.reg);
            }

            instructions::MOV_AR_REG => {
                self.reg = self.ar;
            }
            instructions::PSH_AR => {
                self.push(self.ar);
            }

            instructions::MOV_AAR_REG => {
                self.reg = self.memory.read(self.ar);
            }
            instructions::MOV_AAR_AR => {
                self.ar = self.memory.read(self.ar);
            }

            instructions::MOV_SP_REG => {
                self.reg = self.sp;
            }
            instructions::MOV_SP_AR => {
                self.ar = self.sp;
            }
            instructions::PSH_SP => {
                self.push(self.sp);
            }

            instructions::PUL_REG => {
                self.reg = self.pull();
            }
            instructions::PUL_AR => {
                self.ar = self.pull();
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
                let address = self.read_next();
                let value = self.memory.read(address);
                (self.reg, self.flag) = self.reg.overflowing_add(value);
            }
            instructions::SUB_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                (self.reg, self.flag) = self.reg.overflowing_sub(value);
            }
            instructions::SBF_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                (self.reg, self.flag) = value.overflowing_sub(self.reg);
            }
            instructions::MUL_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                (self.reg, self.flag) = self.reg.overflowing_mul(value);
            }
            instructions::AND_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                self.reg &= value;
                self.flag = false;
            }
            instructions::ORB_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                self.reg |= value;
                self.flag = false;
            }
            instructions::XOR_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
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
            instructions::JMA => self.ip = self.ar,

            instructions::CAL => {
                let addr = self.read_next();
                self.push(self.fp);
                self.push(self.ip);
                self.fp = self.sp;
                self.ip = addr;
            }
            instructions::RET => {
                self.sp = self.fp;
                self.ip = self.pull();
                self.fp = self.pull();
            }

            x => panic!("Error: invalid instruction: {x}"),
        };
        true
    }

    pub fn print_info(&self, address: usize, offset: usize) {
        print!("ip: {:#06x}   ", self.ip);
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
