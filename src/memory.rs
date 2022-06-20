const MEMORY_SIZE: usize = 1 << (8 * 2);

pub struct Memory {
    data: [u8; MEMORY_SIZE],
}

pub fn array_to_idx(address: [u8; 2]) -> usize {
    ((address[0] as usize) << 8) + address[1] as usize
}

impl Memory {
    pub fn new(start: Vec<u8>) -> Memory {
        let mut data = [0; MEMORY_SIZE];
        for (i, &val) in start.iter().enumerate() {
            data[i] = val;
        }
        Memory { data }
    }

    pub fn read(&self, address: [u8; 2]) -> u8 {
        self.data[array_to_idx(address)]
    }

    pub fn read2(&self, address: [u8; 2]) -> [u8; 2] {
        let addr = array_to_idx(address);
        if addr + 1 == MEMORY_SIZE {
            [self.data[addr], self.data[0]]
        } else {
            [self.data[addr], self.data[addr + 1]]
        }
    }

    pub fn write(&mut self, address: [u8; 2], value: u8) {
        self.data[array_to_idx(address)] = value;
    }

    pub fn read_after(&self, address: u16, offset: u16) -> String {
        let mut result = String::new();
        for a in address..(address + offset) {
            result.push_str(&format!("{:02x} ", self.data[a as usize]));
            if a % 4 == 3 {
                result.push_str(" ");
            }
        }
        result
    }
}
