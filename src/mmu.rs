/*
the 8kb internal ram is accessable starting at 0xc000
but also mirrored starting at 0xe000

could possible xor with 0xdfff to normalize both accesses
to go to the same memory
*/

#[macro_export]
macro_rules! mem_access_b {
    ($func:expr, $address:expr) => {
        {
        let result = $func.get_b($address);
        match result {
            None => panic!("Failed memory access at {:?}", stringify!($address)),
            Some(r) => r
        }
        }
    };
    ($func:expr, $address:expr, $val:expr) => {
        {
        let result = $func.set_b($address, $val);
        match result {
            None => panic!("Falied memory access at {:?}", stringify!($address)),
            Some(r) => r
        }
        }
    };
}

#[macro_export]
macro_rules! mem_access_w {
    ($func:expr, $address:expr) => {
        {
        let result = $func.get_w($address);
        match result {
            None => panic!("Failed memory access at {:?}", stringify!($address)),
            Some(r) => r
        }
        }
    };
    ($func:expr, $address:expr, $val:expr) => {
        {
        let result = $func.set_w($address, $val);
        match result {
            None => panic!("Failed memory access at {:?}", stringify!($address)),
            Some(r) => r
        }
        }
    }
}

pub struct MMU {
    mem: Vec<u8>
}

impl MMU {
    pub fn new() -> MMU {
        return MMU {
            mem: vec![0; 0xffff]
        };
    }

    pub fn set_b(&mut self, address: usize, value: u8) -> Option<u8> {
        if address >= self.mem.len() { return None; }
        self.mem[address] = value;
        return Some(value);
    }

    pub fn set_w(&mut self, address: usize, value: u16) -> Option<u16> {
        if address >= self.mem.len() - 1 { return None; }
        let first_byte = (value & 0xff) as u8;
        let second_byte = (value >> 8) as u8;
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
        let mut value: u16 = (self.mem[address] as u16);
        value |= (self.mem[address + 1] as u16) << 8;
        return Some(value);
    }
}