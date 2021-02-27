use crate::mmu::MMU;
use crate::{ mem_access_w, mem_access_b };

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
            self.pc = self.pc.wrapping_add(1);
            isa_map[op as usize](self);
        }

        return self.a;
    }

    pub fn test(&self) -> u16 {
        return mem_access_w!(self.memory_unit, 5);
    }
}

static isa_map: [fn(&mut Z80); 256] = [

    //00
    |cpu: &mut Z80| { cpu.last_m = 1; cpu.last_t = 4; }, //NOP
    |cpu: &mut Z80| { //LDBCnn
        cpu.c = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.b = mem_access_b!(cpu.memory_unit, cpu.pc.wrapping_add(1));
        cpu.pc = cpu.pc.wrapping_add(2); cpu.last_m = 3; cpu.last_t = 12;
    },
    |cpu: &mut Z80| { //LDBCmA
        let mut address: usize = cpu.b as usize;
        address = (address << 8) + cpu.c as usize;
        mem_access_b!(cpu.memory_unit, address, cpu.a);
        cpu.last_m = 2; cpu.last_t = 8;
    },
    |cpu: &mut Z80| { //INCBC
        cpu.c = cpu.c.wrapping_add(1); if cpu.c == 0 { cpu.b = cpu.b.wrapping_add(1); }
        cpu.last_m = 1; cpu.last_t = 4;
    },
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},

    //10
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},

    //20
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},

    //30
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},

    //40
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},

    //50
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},

    //60
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},

    //70
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},

    //80
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},

    //90
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},

    //a0
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},

    //b0
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},

    //c0
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},

    //d0
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},

    //e0
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},

    //f0
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {},
    |cpu: &mut Z80| {}
];