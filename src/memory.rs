pub const ADDR_SIZE: usize = 4;
const MEMORY_SIZE: usize = 1 << (8 * ADDR_SIZE);

pub type Addr = [u8; ADDR_SIZE];

pub struct Memory {
    data: Vec<u8>,
}

pub fn array_to_idx(address: Addr) -> usize {
    let mut total = 0usize;
    for &byte in address.iter() {
        total <<= 8;
        total += byte as usize;
    }
    //((address[0] as usize) << 8) + address[1] as usize
    total
}

impl Memory {
    pub fn new(start: Vec<u8>) -> Memory {
        let mut data = vec![0; MEMORY_SIZE];

        for (i, &val) in start.iter().enumerate() {
            data[i] = val;
        }
        Memory { data }
    }

    pub fn read(&self, address: Addr) -> u8 {
        self.data[array_to_idx(address)]
    }

    pub fn read_addr(&self, address: Addr) -> Addr {
        let addr = array_to_idx(address);
        let mut result: Addr = [0; ADDR_SIZE];
        for i in 0..ADDR_SIZE {
            result[i] = self.data[(addr + i) % MEMORY_SIZE];
        }
        /*
        if addr + 1 == MEMORY_SIZE {
            [self.data[addr], self.data[0]]
        } else {
            [self.data[addr], self.data[addr + 1]]
        } */
        result
    }

    pub fn write(&mut self, address: Addr, value: u8) {
        self.data[array_to_idx(address)] = value;
    }

    pub fn read_after(&self, address: usize, offset: usize) -> String {
        let mut result = String::new();
        for a in address..(address + offset) {
            result.push_str(&format!("{:02x} ", self.data[a]));
            if a % 4 == 3 {
                result.push_str(" ");
            }
        }
        result
    }
}
