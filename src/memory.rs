pub struct Memory {
    data: Box<[u8]>,
}

impl Memory {
    pub fn new(start: &[u8]) -> Memory {
        let mut data = vec![0; 1 << 32].into_boxed_slice();

        for (i, &val) in start.iter().enumerate() {
            data[i] = val;
        }
        Memory { data }
    }

    pub fn read_instruction(&self, address: u32) -> u8 {
        self.data[address as usize]
    }

    pub fn read(&self, address: u32) -> u32 {
        u32::from_be_bytes([
            self.data[address as usize],
            self.data[address.wrapping_add(1) as usize],
            self.data[address.wrapping_add(2) as usize],
            self.data[address.wrapping_add(3) as usize],
        ])
    }

    pub fn write(&mut self, address: u32, value: u32) {
        let arr = value.to_be_bytes();
        self.data[address as usize] = arr[0];
        self.data[address.wrapping_add(1) as usize] = arr[1];
        self.data[address.wrapping_add(2) as usize] = arr[2];
        self.data[address.wrapping_add(3) as usize] = arr[3];
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
