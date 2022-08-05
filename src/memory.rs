pub struct Memory {
    data: Box<[u32]>,
}

impl Memory {
    pub fn new(start: &[u32]) -> Memory {
        // For technical reasons (not enough RAM), the memory is only 4 GB instead of 16 GB
        // Only addresses 0x00000000 to 0x3fffffff are indexable
        let mut data = vec![0; 1 << 30].into_boxed_slice();

        for (i, &val) in start.iter().enumerate() {
            data[i] = val;
        }
        Memory { data }
    }

    pub fn read(&self, address: u32) -> u32 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u32, value: u32) {
        self.data[address as usize] = value;
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
