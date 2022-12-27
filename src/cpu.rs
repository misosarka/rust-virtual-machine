use super::instructions;
use super::memory::Memory;

const STACK_START: u32 = 0x10000000;
const STACK_LIMIT: u32 = 0x20000000 - 0x10;

pub(crate) struct Cpu {
    ip: u32,
    reg: u32,
    ar: u32,
    sp: u32,
    fp: u32,
    flag: bool,
    memory: Memory,
}

impl Cpu {
    pub fn new(start: &[u8]) -> Cpu {
        Cpu {
            ip: 0,
            reg: 0,
            ar: 0,
            sp: STACK_START,
            fp: STACK_START,
            flag: false,
            memory: Memory::new(start),
        }
    }

    fn read_next_instruction(&mut self) -> u8 {
        let result = self.memory.read_instruction(self.ip);
        self.ip = self.ip.wrapping_add(1);
        result
    }

    fn read_next(&mut self) -> u32 {
        let result = self.memory.read(self.ip);
        self.ip = self.ip.wrapping_add(4);
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
        self.sp = self.sp.wrapping_add(4);
        if self.sp > STACK_LIMIT {
            panic!("Error: stack overflow");
        }
    }

    fn pull(&mut self) -> u32 {
        self.sp = self.sp.wrapping_sub(4);
        self.memory.read(self.sp)
    }

    fn add(target: &mut u32, other: u32, flag: &mut bool) {
        (*target, *flag) = target.overflowing_add(other);
    }

    fn sub(target: &mut u32, other: u32, flag: &mut bool) {
        (*target, *flag) = target.overflowing_sub(other);
    }

    fn mul(target: &mut u32, other: u32, flag: &mut bool) {
        (*target, *flag) = target.overflowing_mul(other);
    }

    fn and(target: &mut u32, other: u32, flag: &mut bool) {
        *target &= other;
        *flag = false;
    }

    fn or(target: &mut u32, other: u32, flag: &mut bool) {
        *target |= other;
        *flag = false;
    }

    fn xor(target: &mut u32, other: u32, flag: &mut bool) {
        *target ^= other;
        *flag = false;
    }

    fn shl(target: &mut u32, other: u32, flag: &mut bool) {
        if other >= 32 {
            *target = 0;
            *flag = true;
        } else {
            *flag = target.leading_zeros() < other;
            *target = target.wrapping_shl(other);
        }
    }

    fn shr(target: &mut u32, other: u32, flag: &mut bool) {
        *flag = false;
        if other >= 32 {
            *target = 0;
        } else {
            *target = target.wrapping_shr(other);
        }
    }

    pub fn execute(&mut self) -> bool {
        match self.read_next_instruction() {
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

            instructions::MOV_FP_REG => {
                self.reg = self.fp;
            }
            instructions::MOV_FP_AR => {
                self.ar = self.fp;
            }
            instructions::PSH_FP => {
                self.push(self.fp);
            }

            instructions::PUL_REG => {
                self.reg = self.pull();
            }
            instructions::PUL_AR => {
                self.ar = self.pull();
            }

            instructions::ADD_LIT => {
                let value = self.read_next();
                Self::add(&mut self.reg, value, &mut self.flag);
            }
            instructions::SUB_LIT => {
                let value = self.read_next();
                Self::sub(&mut self.reg, value, &mut self.flag);
            }
            instructions::MUL_LIT => {
                let value = self.read_next();
                Self::mul(&mut self.reg, value, &mut self.flag);
            }
            instructions::AND_LIT => {
                let value = self.read_next();
                Self::and(&mut self.reg, value, &mut self.flag);
            }
            instructions::ORB_LIT => {
                let value = self.read_next();
                Self::or(&mut self.reg, value, &mut self.flag);
            }
            instructions::XOR_LIT => {
                let value = self.read_next();
                Self::xor(&mut self.reg, value, &mut self.flag);
            }
            instructions::SHL_LIT => {
                let value = self.read_next();
                Self::shl(&mut self.reg, value, &mut self.flag);
            }
            instructions::SHR_LIT => {
                let value = self.read_next();
                Self::shr(&mut self.reg, value, &mut self.flag);
            }

            instructions::ADD_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                Self::add(&mut self.reg, value, &mut self.flag);
            }
            instructions::SUB_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                Self::sub(&mut self.reg, value, &mut self.flag);
            }
            instructions::MUL_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                Self::mul(&mut self.reg, value, &mut self.flag);
            }
            instructions::AND_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                Self::and(&mut self.reg, value, &mut self.flag);
            }
            instructions::ORB_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                Self::or(&mut self.reg, value, &mut self.flag);
            }
            instructions::XOR_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                Self::xor(&mut self.reg, value, &mut self.flag);
            }
            instructions::SHL_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                Self::shl(&mut self.reg, value, &mut self.flag);
            }
            instructions::SHR_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                Self::shr(&mut self.reg, value, &mut self.flag);
            }

            instructions::ADD_AAR => {
                let value = self.memory.read(self.ar);
                Self::add(&mut self.reg, value, &mut self.flag);
            }
            instructions::SUB_AAR => {
                let value = self.memory.read(self.ar);
                Self::sub(&mut self.reg, value, &mut self.flag);
            }
            instructions::MUL_AAR => {
                let value = self.memory.read(self.ar);
                Self::mul(&mut self.reg, value, &mut self.flag);
            }
            instructions::AND_AAR => {
                let value = self.memory.read(self.ar);
                Self::and(&mut self.reg, value, &mut self.flag);
            }
            instructions::ORB_AAR => {
                let value = self.memory.read(self.ar);
                Self::or(&mut self.reg, value, &mut self.flag);
            }
            instructions::XOR_AAR => {
                let value = self.memory.read(self.ar);
                Self::xor(&mut self.reg, value, &mut self.flag);
            }
            instructions::SHL_AAR => {
                let value = self.memory.read(self.ar);
                Self::shl(&mut self.reg, value, &mut self.flag);
            }
            instructions::SHR_AAR => {
                let value = self.memory.read(self.ar);
                Self::shr(&mut self.reg, value, &mut self.flag);
            }

            instructions::ADD_PUL => {
                let value = self.pull();
                Self::add(&mut self.reg, value, &mut self.flag);
            }
            instructions::SUB_PUL => {
                let value = self.pull();
                Self::sub(&mut self.reg, value, &mut self.flag);
            }
            instructions::MUL_PUL => {
                let value = self.pull();
                Self::mul(&mut self.reg, value, &mut self.flag);
            }
            instructions::AND_PUL => {
                let value = self.pull();
                Self::and(&mut self.reg, value, &mut self.flag);
            }
            instructions::ORB_PUL => {
                let value = self.pull();
                Self::or(&mut self.reg, value, &mut self.flag);
            }
            instructions::XOR_PUL => {
                let value = self.pull();
                Self::xor(&mut self.reg, value, &mut self.flag);
            }
            instructions::SHL_PUL => {
                let value = self.pull();
                Self::shl(&mut self.reg, value, &mut self.flag);
            }
            instructions::SHR_PUL => {
                let value = self.pull();
                Self::shr(&mut self.reg, value, &mut self.flag);
            }

            instructions::AAD_LIT => {
                let value = self.read_next();
                Self::add(&mut self.ar, value, &mut self.flag);
            }
            instructions::ASB_LIT => {
                let value = self.read_next();
                Self::sub(&mut self.ar, value, &mut self.flag);
            }
            instructions::AML_LIT => {
                let value = self.read_next();
                Self::mul(&mut self.ar, value, &mut self.flag);
            }
            instructions::AAN_LIT => {
                let value = self.read_next();
                Self::and(&mut self.ar, value, &mut self.flag);
            }
            instructions::AOR_LIT => {
                let value = self.read_next();
                Self::or(&mut self.ar, value, &mut self.flag);
            }
            instructions::AXR_LIT => {
                let value = self.read_next();
                Self::xor(&mut self.ar, value, &mut self.flag);
            }
            instructions::ASL_LIT => {
                let value = self.read_next();
                Self::shl(&mut self.ar, value, &mut self.flag);
            }
            instructions::ASR_LIT => {
                let value = self.read_next();
                Self::shr(&mut self.ar, value, &mut self.flag);
            }

            instructions::AAD_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                Self::add(&mut self.ar, value, &mut self.flag);
            }
            instructions::ASB_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                Self::sub(&mut self.ar, value, &mut self.flag);
            }
            instructions::AML_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                Self::mul(&mut self.ar, value, &mut self.flag);
            }
            instructions::AAN_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                Self::and(&mut self.ar, value, &mut self.flag);
            }
            instructions::AOR_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                Self::or(&mut self.ar, value, &mut self.flag);
            }
            instructions::AXR_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                Self::xor(&mut self.ar, value, &mut self.flag);
            }
            instructions::ASL_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                Self::shl(&mut self.ar, value, &mut self.flag);
            }
            instructions::ASR_MEM => {
                let address = self.read_next();
                let value = self.memory.read(address);
                Self::shr(&mut self.ar, value, &mut self.flag);
            }

            instructions::AAD_REG => {
                Self::add(&mut self.ar, self.reg, &mut self.flag);
            }
            instructions::ASB_REG => {
                Self::sub(&mut self.ar, self.reg, &mut self.flag);
            }
            instructions::AML_REG => {
                Self::mul(&mut self.ar, self.reg, &mut self.flag);
            }
            instructions::AAN_REG => {
                Self::and(&mut self.ar, self.reg, &mut self.flag);
            }
            instructions::AOR_REG => {
                Self::or(&mut self.ar, self.reg, &mut self.flag);
            }
            instructions::AXR_REG => {
                Self::xor(&mut self.ar, self.reg, &mut self.flag);
            }
            instructions::ASL_REG => {
                Self::shl(&mut self.ar, self.reg, &mut self.flag);
            }
            instructions::ASR_REG => {
                Self::shr(&mut self.ar, self.reg, &mut self.flag);
            }

            instructions::AAD_PUL => {
                let value = self.pull();
                Self::add(&mut self.ar, value, &mut self.flag);
            }
            instructions::ASB_PUL => {
                let value = self.pull();
                Self::sub(&mut self.ar, value, &mut self.flag);
            }
            instructions::AML_PUL => {
                let value = self.pull();
                Self::mul(&mut self.ar, value, &mut self.flag);
            }
            instructions::AAN_PUL => {
                let value = self.pull();
                Self::and(&mut self.ar, value, &mut self.flag);
            }
            instructions::AOR_PUL => {
                let value = self.pull();
                Self::or(&mut self.ar, value, &mut self.flag);
            }
            instructions::AXR_PUL => {
                let value = self.pull();
                Self::xor(&mut self.ar, value, &mut self.flag);
            }
            instructions::ASL_PUL => {
                let value = self.pull();
                Self::shl(&mut self.ar, value, &mut self.flag);
            }
            instructions::ASR_PUL => {
                let value = self.pull();
                Self::shr(&mut self.ar, value, &mut self.flag);
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

    pub fn print_info(&self) {
        print!("I: {:04x}   ", self.ip);
        print!("RFA: {:08x} ", self.reg);
        print!("{} ", if self.flag { "F" } else { " " });
        print!("{:08x}   ", self.ar);
        print!("SF: {:08x} {:08x}   ", self.sp, self.fp);
        print!("frame: {}", self.memory.read_after(self.fp as usize, 0x10));
        println!();
    }
}
