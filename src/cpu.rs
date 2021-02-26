use crate::mmu::MMU;

pub struct Z80 {
    memory_unit: MMU,
    global_m: u8,
    global_t: u8,
    last_m: u8,
    last_t: u8,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    pc: usize,
    sp: usize
}

impl Z80 {
    pub fn new(mut memory_unit: MMU) -> Z80 {
        return Z80 {
            memory_unit,
            global_m: 0,
            global_t: 0,
            last_m: 0,
            last_t: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0
        }
    }

    pub fn reset(&mut self) {
        
    }

    pub fn run(&mut self) -> u8 {
        if let Some(op) = self.memory_unit.get_b(self.pc) {
            self.pc += 1;
            isa_map[op as usize](self);
        }

        return self.a;
    }

    pub fn test(&self) -> u16 {
        return self.memory_unit.get_w(5).unwrap();
    }
}

static isa_map: [fn(&mut Z80); 1] = [
    |cpu: &mut Z80| { cpu.a += 1; }
];