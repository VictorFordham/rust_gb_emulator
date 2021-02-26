pub struct MMU {
    mem: Vec<u8>
}

impl MMU {
    pub fn new() -> MMU {
        return MMU {
            mem: vec![0; 0xffff]
        };
    }

    pub fn set_b(&mut self, address: usize, value: u8) -> Option<u8>  {
        if address >= self.mem.len() { return None; }
        self.mem[address] = value;
        return Some(value);
    }

    pub fn set_w(&mut self, address: usize, value: u16) -> Option<u16> {
        if address >= self.mem.len() - 1 { return None; }
        let first_byte = (value >> 8) as u8;
        let second_byte = (value & 0xff) as u8;
        self.mem[address] = first_byte;
        self.mem[address + 1] = second_byte;
        return Some(value);
    }

    pub fn get_b(&self, address: usize) -> Option<u8> {
        if address >= self.mem.len() { return None; }
        return Some(self.mem[address]);
    }

    pub fn get_w(&self, address: usize) -> Option<u16> {
        if address >= self.mem.len() - 1 { return None; }
        let mut value: u16 = (self.mem[address] as u16) << 8;
        value |= self.mem[address + 1] as u16;
        return Some(value);
    }
}