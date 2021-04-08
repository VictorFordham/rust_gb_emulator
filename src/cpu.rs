use crate::mmu::MMU;
use crate::{ mem_access_w, mem_access_b };


const ZERO_FLAG: u8 = 0x80;
const SUB_FLAG: u8 = 0x40;
const HCARRY_FLAG: u8 = 0x20;
const CARRY_FLAG: u8 = 0x10;


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
    sp: usize,
    halt: bool,
    ime: bool
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
            sp: 0,
            halt: false,
            ime: true
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

static undefined = |x: u16| { panic!("Hit undefined instruction at {:?}", stringify!(x - 1)); };

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
    |cpu: &mut Z80| { //INCr_b
        cpu.b = cpu.b.wrapping_add(1);
        cpu.f = 0;
        if cpu.b == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    },
    |cpu: &mut Z80| {
        cpu.b = cpu.b.wrapping_sub(1);
        cpu.f = 0;
        if cpu.b == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //DECr_b
    |cpu: &mut Z80| {
        cpu.b = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDrn_b
    |cpu: &mut Z80| {
        let ci = (cpu.a & 0x80 != 0) as u8;
        let co = (cpu.a & 0x80 != 0) as u8 * 0x10;
        cpu.a = ci.wrapping_add(cpu.a << 1);
        cpu.f = co.wrapping_add(cpu.f & 0xef);
        cpu.last_m = 1; cpu.last_t = 4;
    }, //RLCA
    |cpu: &mut Z80| { //the guy never implmented this, just my guess at what it would be
        let address = mem_access_w!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(2);
        mem_access_w!(cpu.memory_unit, address, cpu.sp);
        cpu.last_m = 5; cpu.last_t = 20;
    }, //LDmmSP
    |cpu: &mut Z80| {
        let hl = (cpu.h as u16 << 8) + cpu.l as u16;
        let bc = (cpu.b as u16 << 8) + cpu.c as u16;
        let (i, b) hl.overflowing_add(bc);
        if b {
            cpu.f |= CARRY_FLAG;
        } else {
            cpu.f &= (0xff - CARRY_FLAG);
        }
        cpu.h = hl >> 8 as u8;
        cpu.l = hl & 0xff as u8;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //ADDHLBC
    |cpu: &mut Z80| {
        let address = (cpu.b as u16 << 8) + cpu.c as u16;
        cpu.a = mem_access_b!(cpu.memory_unit, address);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDABCm
    |cpu: &mut Z80| {
        cpu.c = cpu.c.wrapping_sub(1); if cpu.c == 0xff { cpu.b = cpu.b.wrapping_sub(1); }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //DECBC
    |cpu: &mut Z80| {
        cpu.c = cpu.wrapping_add(1);
        cpu.f = 0;
        if cpu.c == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //INCr_c
    |cpu: &mut Z80| {
        cpu.c = cpu.c.wrapping_sub(1);
        cpu.f = 0;
        if cpu.c == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //DECr_c
    |cpu: &mut Z80| {
        cpu.c = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDrn_c
    |cpu: &mut Z80| {
        let ci = (cpu.a & 1 != 0) as u8 * 0x80;
        let co = (cpu.a & 1 != 0) as u8 * 0x10;
        cpu.a = ci.wrapping_add(cpu.a >> 1);
        cpu.f = (cpu.f & (0xff - CARRY_FLAG)) + co;
        cpu.last_m = 1; cpu.last_t = 4;
    }, //RRCA

    //10
    |cpu: &mut Z80| {
        cpu.last_m = 2; cpu.last_t = 8;
        let offset = mem_access_b!(cpu.memory_unit, cpu.pc) as i8;
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.b = cpu.b.wrapping_sub(1);
        if cpu.b != 0 {
            cpu.pc = (cpu.pc as i16).wrapping_add(offset as i16) as u16;
            cpu.last_m += 1;
            cpu.last_t += 4;
        }
    }, //DJNZn
    |cpu: &mut Z80| {
        cpu.e = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.d = mem_access_b!(cpu.memory_unit, cpu.pc.wrapping_add(1));
        cpu.pc = cpu.pc.wrapping_add(2);
        cpu.last_m = 3; cpu.last_t = 12;
    }, //LDDEnn
    |cpu: &mut Z80| {
        let mut address: usize = cpu.d as usize;
        address = (address << 8) + cpu.e as usize;
        mem_access_b!(cpu.memory_unit, address, cpu.a);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDDEmA
    |cpu: &mut Z80| {
        cpu.e = cpu.e.wrapping_add(1);
        if cpu.e == 0 { cpu.d = cpu.d.wrapping_add(1); }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //INCDE
    |cpu: &mut Z80| {
        cpu.d = cpu.d.wrapping_add(1);
        cpu.f = 0;
        if cpu.d == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //INCr_d
    |cpu: &mut Z80| {
        cpu.d = cpu.d.wrapping_sub(1);
        cpu.f = 0;
        if cpu.d == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //DECr_d
    |cpu: &mut Z80| {
        cpu.d = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDrn_d
    |cpu: &mut Z80| {
        let ci = (cpu.f & CARRY_FLAG != 0) as u8;
        let co = (cpu.a & 0x80 != 0) as u8 * 0x10;
        cpu.a = ci.wrapping_add(cpu.a << 1);
        cpu.f = co.wrapping_add(cpu.f & 0xef);
        cpu.last_m = 1; cpu.last_t = 4;
    }, //RLA
    |cpu: &mut Z80| {
        let offset = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc += 1;
        cpu.pc = (cpu.pc as i16 + (offset as i8) as i16) as u16;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //JRn
    |cpu: &mut Z80| {
        let hl: u16 = (cpu.h as u16 << 8) + cpu.l as u16;
        let de: u16 = (cpu.d as u16 << 8) + cpu.e as u16;
        let (i, b) = hl.overflowing_add(de);
        cpu.h = (i >> 8) as u8;
        cpu.l = (i & 0xff) as u8;
        if b { cpu.f |= CARRY_FLAG; } else { cpu.f &= 0xff - CARRY_FLAG; }
        cpu.last_m = 3; cpu.last_t = 12;
    }, //ADDHLDE
    |cpu: &mut Z80| {
        let mut address: usize = cpu.d as usize;
        address = (address << 8) + cpu.e as usize;
        cpu.a = mem_access_b!(cpu.memory_unit, address);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDADEm
    |cpu: &mut Z80| {
        cpu.e = cpu.e.wrapping_sub(1);
        if cpu.e == 0xff { cpu.d = cpu.d.wrapping_sub(1); }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //DECDE
    |cpu: &mut Z80| {
        cpu.e = cpu.e.wrapping_add(1);
        cpu.f = 0;
        if cpu.e == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //INCr_e
    |cpu: &mut Z80| {
        cpu.e = cpu.e.wrapping_sub(1);
        cpu.f = 0;
        if cpu.e == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //DECr_e
    |cpu: &mut Z80| {
        cpu.e = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDrn_e
    |cpu: &mut Z80| {
        let ci = (cpu.f & CARRY_FLAG != 0) as u8 * 0x80;
        let co = (cpu.a & 1 != = 0) as u8 0x10;
        cpu.a = ci.wrapping_add(cpu.a >> 1);
        cpu.f = co.wrapping_add(cpu.f & 0xef);
        cpu.last_m = 1; cpu.last_t = 4;
    }, //RRA

    //20
    |cpu: &mut Z80| {
        cpu.last_m = 2; cpu.last_t = 8;
        let offset = mem_access_b!(cpu.memory_unit, cpu.pc) as i8;
        cpu.pc = cpu.pc.wrapping_add(1);
        if cpu.f & ZERO_FLAG == 0 {
            cpu.pc = (cpu.pc as i16 + offset as i16) as u16;
            cpu.last_m += 1; cpu.last_t += 4;
        }
    }, //JRNZn
    |cpu: &mut Z80| {
        cpu.l = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.h = mem_access_b!(cpu.memory_unit, cpu.pc.wrapping_add(1));
        cpu.pc = cpu.pc.wrapping_add(2);
        cpu.last_m = 3; cpu.last_t = 12;
    }, //LDHLnn
    |cpu: &mut Z80| {
        let mut address: u16 =(cpu.h as u16 << 8) + cpu.l as u16;
        mem_access_b!(cpu.memory_unit, address, cpu.a);
        address = address.wrapping_add(1);
        cpu.h = (address >> 8) as u8;
        cpu.l = (address & 0xff) as u8;
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDHLIA
    |cpu: &mut Z80| {
        cpu.l = cpu.l.wrapping_add(1);
        if cpu.l == 0 { cpu.h = cpu.h.wrapping_add(1); }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //INCHL
    |cpu: &mut Z80| {
        cpu.h = cpu.h.wrapping_add(1);
        cpu.f = 0;
        if cpu.h == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //INCr_h
    |cpu: &mut Z80| {
        cpu.h = cpu.h.wrapping_sub(1);
        cpu.f = 0;
        if cpu.h == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //DECr_h
    |cpu: &mut Z80| {
        cpu.h = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDrn_h
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {
        cpu.last_m = 2; cpu.last_t = 8;
        let offset = mem_access_b!(cpu.memory_unit, cpu.pc) as i8;
        cpu.pc = cpu.pc.wrapping_add(1);
        if cpu.f & ZERO_FLAG != 0 {
            cpu.pc = (cpu.pc as i16 + offset as i16) as u16;
            cpu.last_m += 1; cpu.last_t += 4;
        }
    }, //JRZn
    |cpu: &mut Z80| {
        let hl: u16 = (cpu.h as u16 << 8) + cpu.l as u16;
        let (i, b) = hl.overflowing_add(hl);
        if b {
            cpu.f |= CARRY_FLAG;
        } else {
            cpu.f &= 0xff - CARRY_FLAG;
        }
        cpu.h = (hl >> 8) as u8;
        cpu.l = (hl & 0xff) as u8;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //ADDHLHL
    |cpu: &mut Z80| {
        let mut address: u16 = (cpu.h as u16 << 8) + cpu.l as u16;
        cpu.a = mem_access_b!(cpu.memory_unit, address);
        address = address.wrapping_add(1);
        cpu.h = (address >> 8) as u8;
        cpu.l = (address & 0xff) as u8;
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDAHLI
    |cpu: &mut Z80| {
        cpu.l = cpu.l.wrapping_sub(1);
        if cpu.l == 0xff { cpu.h = cpu.h.wrapping_sub(1); }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //DECHL
    |cpu: &mut Z80| {
        cpu.l = cpu.l.wrapping_add(1);
        cpu.f = 0;
        if cpu.l == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //INCr_l
    |cpu: &mut Z80| {
        cpu.l = cpu.l.wrapping_sub(1);
        cpu.f = 0;
        if cpu.l == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //DECr_l
    |cpu: &mut Z80| {
        cpu.l = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDrn_l
    |cpu: &mut Z80| {
        cpu.a = ~cpu.a;
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //CPL

    //30
    |cpu: &mut Z80| {}, //JRNCn
    |cpu: &mut Z80| {}, //LDSPnn
    |cpu: &mut Z80| {}, //LDHLDA
    |cpu: &mut Z80| { cpu.sp = cpu.sp.wrapping_add(1); cpu.last_m = 1; cpu.last_t = 4; }, //INCSP
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        let val = mem_access_b!(cpu.memory_unit, address).wrapping_add(1);
        mem_access_b!(cpu.memory_unit, address, val);
        cpu.f = 0;
        if val == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 3; cpu.last_t = 12;
    }, //INCHLm
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        let val = mem_access_b!(cpu.memory_unit, address).wrapping_sub(1);
        mem_access_b!(cpu.memory_unit, address, val);
        cpu.f = 0;
        if val == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 3; cpu.last_t = 12;
    }, //DECHLm
    |cpu: &mut Z80| {}, //LDHLmn
    |cpu: &mut Z80| {}, //SCF
    |cpu: &mut Z80| {}, //JRCn
    |cpu: &mut Z80| {}, //ADDHLSP
    |cpu: &mut Z80| {}, //LDAHLD
    |cpu: &mut Z80| { cpu.sp = cpu.sp.wrapping_sub(1); cpu.last_m = 1; cpu.last_t = 4; }, //DECSP
    |cpu: &mut Z80| {
        cpu.a = cpu.a.wrapping_add(1);
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //INCr_a
    |cpu: &mut Z80| {
        cpu.a = cpu.a.wrapping_sub(1);
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //DECr_a
    |cpu: &mut Z80| {}, //LDrn_a
    |cpu: &mut Z80| {}, //CCF

    //40
    |cpu: &mut Z80| { cpu.b = cpu.b; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_bb
    |cpu: &mut Z80| { cpu.b = cpu.c; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_bc
    |cpu: &mut Z80| { cpu.b = cpu.d; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_bd
    |cpu: &mut Z80| { cpu.b = cpu.e; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_be
    |cpu: &mut Z80| { cpu.b = cpu.h; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_bh
    |cpu: &mut Z80| { cpu.b = cpu.l; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_bl
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        cpu.b = mem_access_b!(cpu.memory_unit, address);
        cpu.last_m = 2; cpu.last_t = 8; 
    }, //LDrHLm_b
    |cpu: &mut Z80| { cpu.b = cpu.a; cpu.last_m = 1; cpu.last_t = 4; }, //Ldrr_ba
    |cpu: &mut Z80| { cpu.c = cpu.b; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_cb
    |cpu: &mut Z80| { cpu.c = cpu.c; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_cc
    |cpu: &mut Z80| { cpu.c = cpu.d; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_cd
    |cpu: &mut Z80| { cpu.c = cpu.e; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ce
    |cpu: &mut Z80| { cpu.c = cpu.h; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ch
    |cpu: &mut Z80| { cpu.c = cpu.l; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_cl
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        cpu.c = mem_access_b!(cpu.memory_unit, address);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDrHLm_c
    |cpu: &mut Z80| { cpu.c = cpu.a; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ca

    //50
    |cpu: &mut Z80| { cpu.d = cpu.b; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_db
    |cpu: &mut Z80| { cpu.d = cpu.c; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_dc
    |cpu: &mut Z80| { cpu.d = cpu.d; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_dd
    |cpu: &mut Z80| { cpu.d = cpu.e; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_de
    |cpu: &mut Z80| { cpu.d = cpu.h; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_dh
    |cpu: &mut Z80| { cpu.d = cpu.l; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_dl
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        cpu.d = mem_access_b!(cpu.memory_unit, address);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDrHLm_d
    |cpu: &mut Z80| { cpu.d = cpu.a; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_da
    |cpu: &mut Z80| { cpu.e = cpu.b; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_eb
    |cpu: &mut Z80| { cpu.e = cpu.c; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ec
    |cpu: &mut Z80| { cpu.e = cpu.d; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ed
    |cpu: &mut Z80| { cpu.e = cpu.e; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ee
    |cpu: &mut Z80| { cpu.e = cpu.h; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_eh
    |cpu: &mut Z80| { cpu.e = cpu.l; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_el
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        cpu.e = mem_access_b!(cpu.memory_unit, address);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDrHLm_e
    |cpu: &mut Z80| { cpu.e = cpu.a; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ea

    //60
    |cpu: &mut Z80| { cpu.h = cpu.b; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_hb
    |cpu: &mut Z80| { cpu.h = cpu.c; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_hc
    |cpu: &mut Z80| { cpu.h = cpu.d; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_hd
    |cpu: &mut Z80| { cpu.h = cpu.e; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_he
    |cpu: &mut Z80| { cpu.h = cpu.h; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_hh
    |cpu: &mut Z80| { cpu.h = cpu.l; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_hl
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        cpu.h = mem_access_b!(cpu.memory_unit, address);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDrHLm_h
    |cpu: &mut Z80| { cpu.h = cpu.a; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ha
    |cpu: &mut Z80| { cpu.l = cpu.b; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_lb
    |cpu: &mut Z80| { cpu.l = cpu.c; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_lc
    |cpu: &mut Z80| { cpu.l = cpu.d; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ld
    |cpu: &mut Z80| { cpu.l = cpu.e; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_le
    |cpu: &mut Z80| { cpu.l = cpu.h; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_lh
    |cpu: &mut Z80| { cpu.l = cpu.l; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ll
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        cpu.h = mem_access_b!(cpu.memory_unit, address);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDrHLm_l
    |cpu: &mut Z80| { cpu.l = cpu.a; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_la

    //70
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        mem_access_b!(cpu.memory_unit, address, cpu.b);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDHLmr_b
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        mem_access_b!(cpu.memory_unit, address, cpu.c);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDHLmr_c
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        mem_access_b!(cpu.memory_unit, address, cpu.d);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDHLmr_d
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        mem_access_b!(cpu.memory_unit, address, cpu.e);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDHLmr_e
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        mem_access_b!(cpu.memory_unit, address, cpu.h);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDHLmr_h
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        mem_access_b!(cpu.memory_unit, address, cpu.l);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDHLmr_l
    |cpu: &mut Z80| { cpu.halt = true; cpu.last_m = 1; cpu.last_t = 4; }, //halt
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        mem_access_b!(cpu.memory_unit, address, cpu.a);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDHLmr_a
    |cpu: &mut Z80| { cpu.a = cpu.b; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ab
    |cpu: &mut Z80| { cpu.a = cpu.c; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ac
    |cpu: &mut Z80| { cpu.a = cpu.d; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ad
    |cpu: &mut Z80| { cpu.a = cpu.e; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ae
    |cpu: &mut Z80| { cpu.a = cpu.h; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ah
    |cpu: &mut Z80| { cpu.a = cpu.l; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_al
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        cpu.a = mem_access_b!(cpu.memory_unit, address);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDrHLm_a
    |cpu: &mut Z80| { cpu.a = cpu.a; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_aa

    //80
    |cpu: &mut Z80| {
        let (val, b) = cpu.a.overflowing_add(cpu.b);
        cpu.a = val;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ADDr_b
    |cpu: &mut Z80| {
        let (val, b) = cpu.a.overflowing_add(cpu.c);
        cpu.a = val;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ADDr_c
    |cpu: &mut Z80| {
        let (val, b) = cpu.a.overflowing_add(cpu.d);
        cpu.a = val;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ADDr_d
    |cpu: &mut Z80| {
        let (val, b) = cpu.a.overflowing_add(cpu.e);
        cpu.a = val;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ADDr_e
    |cpu: &mut Z80| {
        let (val, b) = cpu.a.overflowing_add(cpu.h);
        cpu.a = val;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ADDr_h
    |cpu: &mut Z80| {
        let (val, b) = cpu.a.overflowing_add(cpu.l);
        cpu.a = val;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ADDr_l
    |cpu: &mut Z80| {}, //ADDHL
    |cpu: &mut Z80| {
        let (val, b) = cpu.a.overfowing_add(cpu.a);
        cpu.a = val;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ADDr_a
    |cpu: &mut Z80| {
        let (mut val, b) = cpu.a.overflowing_add(cpu.b);
        if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_add(1); }
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b || cpu.a > val { cpu.f |= CARRY_FLAG; }
        cpu.a = val;
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ADCr_b
    |cpu: &mut Z80| {
        let (mut val, b) = cpu.a.overflowing_add(cpu.c);
        if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_add(1); }
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b || cpu.a > val { cpu.f |= CARRY_FLAG; }
        cpu.a = val;
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ADCr_c
    |cpu: &mut Z80| {
        let (mut val, b) = cpu.a.overflowing_add(cpu.d);
        if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_add(1); }
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b || cpu.a > val { cpu.f |= CARRY_FLAG; }
        cpu.a = val;
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ADCr_d
    |cpu: &mut Z80| {
        let (mut val, b) = cpu.a.overflowing_add(cpu.e);
        if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_add(1); }
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b || cpu.a > val { cpu.f |= CARRY_FLAG; }
        cpu.a = val;
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ADCr_e
    |cpu: &mut Z80| {
        let (mut val, b) = cpu.a.overflowing_add(cpu.h);
        if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_add(1); }
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b || cpu.a > val { cpu.f |= CARRY_FLAG; }
        cpu.a = val;
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ADCr_h
    |cpu: &mut Z80| {
        let (mut val, b) = cpu.a.overflowing_add(cpu.l);
        if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_add(1); }
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b || cpu.a > val { cpu.f |= CARRY_FLAG; }
        cpu.a = val;
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ADCr_l
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        let i = mem_access_b!(cpu.memory_unit, address);
        let (mut val, b) = cpu.a.overflowing_add(i);
        if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_add(1); }
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b || cpu.a > val { cpu.f |= CARRY_FLAG; }
        cpu.a = val;
        cpu.last_m = 2; cpu.last_t = 8;
    }, //ADCHL
    |cpu: &mut Z80| {
        let (mut val, b) = cpu.a.overflowing_add(cpu.a);
        if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_add(1); }
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b || cpu.a > val { cpu.f |= CARRY_FLAG; }
        cpu.a = val;
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ADCr_a

    //90
    |cpu: &mut Z80| {
        let (val, b) = cpu.a.overflowing_sub(cpu.b);
        cpu.a = val;
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //SUBr_b
    |cpu: &mut Z80| {
        let (val, b) = cpu.a.overflowing_sub(cpu.c);
        cpu.a = val;
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //SUBr_c
    |cpu: &mut Z80| {
        let (val, b) = cpu.a.overflowing_sub(cpu.d);
        cpu.a = val;
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //SUBr_d
    |cpu: &mut Z80| {
        let (val, b) = cpu.a.overflowing_sub(cpu.e);
        cpu.a = val;
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //SUBr_e
    |cpu: &mut Z80| {
        let (val, b) = cpu.a.overflowing_sub(cpu.h);
        cpu.a = val;
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //SUBr_h
    |cpu: &mut Z80| {
        let (val, b) = cpu.a.overflowing_sub(cpu.l);
        cpu.a = val;
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //SUBr_l
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        let val = mem_access_b!(cpu.memory_unit, address);
        let (i, b) = cpu.a.overflowing_sub(val);
        cpu.a = i;
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //SUBHL
    |cpu: &mut Z80| {
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //SUBr_a
    |cpu: &mut Z80| {
        let (mut val, b) = cpu.a.overflowing_sub(cpu.b);
        if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_sub(1); }
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b || cpu.a < val { cpu.f |= CARRY_FLAG; }
        cpu.a = val;
        cpu.last_m = 1; cpu.last_t = 4;
    }, //SBCr_b
    |cpu: &mut Z80| {
        let (mut val, b) = cpu.a.overflowing_sub(cpu.c);
        if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_sub(1); }
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b || cpu.a < val { cpu.f |= CARRY_FLAG; }
        cpu.a = val;
        cpu.last_m = 1; cpu.last_t = 4;
    }, //SBCr_c
    |cpu: &mut Z80| {
        let (mut val, b) = cpu.a.overflowing_sub(cpu.d);
        if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_sub(1); }
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b || cpu.a < val { cpu.f |= CARRY_FLAG; }
        cpu.a = val;
        cpu.last_m = 1; cpu.last_t = 4;
    }, //SBCr_d
    |cpu: &mut Z80| {
        let (mut val, b) = cpu.a.overflowing_sub(cpu.e);
        if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_sub(1); }
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b || cpu.a < val { cpu.f |= CARRY_FLAG; }
        cpu.a = val;
        cpu.last_m = 1; cpu.last_t = 4;
    }, //SBCr_e
    |cpu: &mut Z80| {
        let (mut val, b) = cpu.a.overflowing_sub(cpu.h);
        if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_sub(1); }
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b || cpu.a < val { cpu.f |= CARRY_FLAG; }
        cpu.a = val;
        cpu.last_m = 1; cpu.last_t = 4;
    }, //SBCr_h
    |cpu: &mut Z80| {
        let (mut val, b) = cpu.a.overflowing_sub(cpu.l);
        if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_sub(1); }
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b || cpu.a < val { cpu.f |= CARRY_FLAG; }
        cpu.a = val;
        cpu.last_m = 1; cpu.last_t = 4;
    }, //SBCr_l
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        let i = mem_access_b!(cpu.memory_unit, address);
        let (mut val, b) = cpu.a.overflowing_sub(i);
        if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_sub(1); }
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b || cpu.a < val { cpu.f |= CARRY_FLAG; }
        cpu.a = val;
        cpu.last_m = 1; cpu.last_t = 4;
    }, //SBCHL
    |cpu: &mut Z80| {
        let (mut val, b) = cpu.a.overflowing_sub(cpu.a);
        if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_sub(1); }
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b || cpu.a < val { cpu.f |= CARRY_FLAG; }
        cpu.a = val;
        cpu.last_m = 1; cpu.last_t = 4;
    }, //SBCr_a

    //a0
    |cpu: &mut Z80| {
        cpu.a &= cpu.b;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ANDr_b
    |cpu: &mut Z80| {
        cpu.a &= cpu.c;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ANDr_c
    |cpu: &mut Z80| {
        cpu.a &= cpu.d;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ANDr_d
    |cpu: &mut Z80| {
        cpu.a &= cpu.e;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ANDr_e
    |cpu: &mut Z80| {
        cpu.a &= cpu.h;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ANDr_h
    |cpu: &mut Z80| {
        cpu.a &= cpu.l;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ANDr_l
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        cpu.a &= mem_access_b!(cpu.memory_unit, address);
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //ANDHL
    |cpu: &mut Z80| {
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ANDr_a
    |cpu: &mut Z80| {
        cpu.a ^= cpu.b;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //XORr_b
    |cpu: &mut Z80| {
        cpu.a ^= cpu.c;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |- ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //XORr_c
    |cpu: &mut Z80| {
        cpu.a ^= cpu.d;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //XORr_d
    |cpu: &mut Z80| {
        cpu.a ^= cpu.e;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //XORr_e
    |cpu: &mut Z80| {
        cpu.a ^= cpu.h;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //XORr_h
    |cpu: &mut Z80| {
        cpu.a ^= cpu.l;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //XORr_l
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        cpu.a ^= mem_access_b!(cpu.memory_unit, address);
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //XORHL
    |cpu: &mut Z80| { cpu.a ^= cpu.a; cpu.f = ZERO_FLAG; cpu.last_m = 1; cpu.last_t = 4; }, //XORr_a

    //b0
    |cpu: &mut Z80| {
        cpu.a |= cpu.b;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ORr_b
    |cpu: &mut Z80| {
        cpu.a |= cpu.c;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ORr_c
    |cpu: &mut Z80| {
        cpu.a |= cpu.d;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ORr_d
    |cpu: &mut Z80| {
        cpu.a |= cpu.e;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ORr_e
    |cpu: &mut Z80| {
        cpu.a |= cpu.h;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ORr_h
    |cpu: &mut Z80| {
        cpu.a |= cpu.l;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ORr_l
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        cpu.a |= mem_access_b!(cpu.memory_unit, address);
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //ORHL
    |cpu: &mut Z80| {
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //ORr_a
    |cpu: &mut Z80| {
        let (i, b) = cpu.a.overflowing_sub(cpu.b);
        cpu.f = SUB_FLAG;
        if i == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //CPr_b
    |cpu: &mut Z80| {
        let (i, b) = cpu.a.overflowing_sub(cpu.c);
        cpu.f = SUB_FLAG;
        if i == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //CPr_c
    |cpu: &mut Z80| {
        let (i, b) = cpu.a.overflowing_sub(cpu.d);
        cpu.f = SUB_FLAG;
        if i == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //CPr_d
    |cpu: &mut Z80| {
        let (i, b) = cpu.a.overflowing_sub(cpu.e);
        cpu.f = SUB_FLAG;
        if i == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //CPr_e
    |cpu: &mut Z80| {
        let (i, b) = cpu.a.overflowing_sub(cpu.h);
        cpu.f = SUB_FLAG;
        if i == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //CPr_h
    |cpu: &mut Z80| {
        let (i, b) = cpu.a.overflowing_sub(cpu.l);
        cpu.f = SUB_FLAG;
        if i == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //CPr_l
    |cpu: &mut Z80| {
        let mut address = cpu.h as u16 << 8;
        address += cpu.l as u16;
        let val = mem_access_b!(cpu.memory_unit, address);
        let (i, b) = cpu.a.overflowing_sub(val);
        cpu.f = SUB_FLAG;
        if i == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //CPHL
    |cpu: &mut Z80| {
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //CPr_a

    //c0
    |cpu: &mut Z80| {
        cpu.last_m = 1; cpu.last_t = 4;
        if cpu.f & ZERO_FLAG == 0 {
            cpu.pc = mem_access_w!(cpu.memory_unit, cpu.sp);
            cpu.sp = cpu.sp.wrapping_add(2);
            cpu.last_m += 2; cpu.last_t += 8;
        }
    }, //RETNZ
    |cpu: &mut Z80| {
        let value = mem_access_w!(cpu.memory_unit, cpu.sp);
        cpu.sp = cpu.sp.wrapping_add(2);
        cpu.b = value >> 8 as u8;
        cpu.c = value & 0xff as u8;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //POPBC
    |cpu: &mut Z80| {
        cpu.last_m = 3; cpu.last_t = 12;
        if (cpu.f & ZERO_FLAG) == 0 {
            cpu.pc = mem_access_w!(cpu.memory_unit, cpu.pc);
            cpu.last_m += 1; cpu.last_t += 4;
        } else { cpu.pc = cpu.pc.wrapping_add(2); }
    }, //JPNZnn
    |cpu: &mut Z80| {
        cpu.pc = mem_access_w!(cpu.memory_unit, cpu.pc);
        cpu.last_m = 3; cpu.last_t = 12;
    }, //JPnn
    |cpu: &mut Z80| {
        cpu.last_m = 3; cpu.last_t = 12;
        if cpu.f & ZERO_FLAG == 0 {
            cpu.sp = cpu.sp.wrapping_sub(2);
            mem_access_w!(cpu.memory_unit, cpu.sp, cpu.pc.wrapping_add(2));
            cpu.pc = mem_access_w!(cpu.memory_unit, cpu.pc);
            cpu.last_m += 2; cpu.last_t += 8;
        } else { cpu.pc = cpu.pc.wrapping_add(2); }
    }, //CALLNZnn
    |cpu: &mut Z80| {
        cpu.sp = cpu.sp.wrapping_sub(2);
        let value = (cpu.b as u16 << 8) + (cpu.c as u16);
        mem_access_w!(cpu.memory_unit, cpu.sp, value);
        cpu.last_m = 3; cpu.last_t = 12;
    }, //PUSHBC
    |cpu: &mut Z80| {
        let val = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let (i, b) = cpu.a.overflowing_add(val);
        cpu.a = i; cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //ADDn
    |cpu: &mut Z80| {
        cpu.sp = cpu.sp.wrapping_sub(2);
        mem_access_w!(cpu.memory_unit, cpu.sp, cpu.pc);
        cpu.pc = 0x00;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //RST00
    |cpu: &mut Z80| {
        cpu.last_m = 1; cpu.last_t = 4;
        if cpu.f & ZERO_FLAG == ZERO_FLAG {
            cpu.pc = mem_access_w!(cpu.memory_unit, cpu.sp);
            cpu.sp = cpu.sp.wrapping_add(2);
            cpu.last_m += 2; cpu.last_t += 8;
        }
    }, //RETZ
    |cpu: &mut Z80| {
        cpu.pc = mem_access_w!(cpu.memory_unit, cpu.sp); cpu.sp += 2;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //RET
    |cpu: &mut Z80| {
        cpu.last_m = 3; cpu.last_t = 12;
        if (cpu.f & ZERO_FLAG) == ZERO_FLAG {
            cpu.pc = mem_access_w!(cpu.memory_unit, cpu.pc);
            cpu.last_m += 1; cpu.last_t += 4;
        } else { cpu.pc = cpu.pc.wrapping_add(2); }
    }, //JPZnn
    |cpu: &mut Z80| {}, //MAPcb
    |cpu: &mut Z80| {
        cpu.last_m = 3; cpu.last_t = 12;
        if cpu.f & ZERO_FLAG == ZERO_FLAG {
            cpu.sp = cpu.sp.wrapping_sub(2);
            mem_access_w!(cpu.memory_unit, cpu.sp, cpu.pc.wrapping_add(2);
            cpu.pc = mem_access_w!(cpu.memory_unit, cpu.pc);
            cpu.last_m += 2; cpu.last+t += 8;
        } else { cpu.pc += 2; }
    }, //CALLZnn
    |cpu: &mut Z80| {
        cpu.sp = cpu.sp.wrapping_sub(2);
        mem_access_w!(cpu.memory_unit, cpu.sp, cpu.pc.wrapping_add(2));
        cpu.pc = mem_access_w!(cpu.memory_unit, cpu.pc);
        cpu.last_m = 5; cpu.last_t = 20;
    }, //CALLnn
    |cpu: &mut Z80| {}, //ADCn
    |cpu: &mut Z80| {
        cpu.sp = cpu.sp.wrapping_sub(2);
        mem_access_w!(cpu.memory_unit, cpu.sp, cpu.pc);
        cpu.pc = 0x08;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //RST08

    //d0
    |cpu: &mut Z80| {
        cpu.last_m = 1; cpu.last_t = 4;
        if cpu.f & CARRY_FLAG == 0 {
            cpu.pc = mem_access_w!(cpu.memory_unit, cpu.sp);
            cpu.sp = cpu.sp.wrapping_add(2);
            cpu.last_m += 2; cpu.last_t += 8;
        }
    }, //RETNC
    |cpu: &mut Z80| {
        let value = mem_access_w!(cpu.memory_unit, cpu.sp);
        cpu.sp = cpu.sp.wrapping_add(2);
        cpu.d = value >> 8 as u8;
        cpu.e = value & 0xff as u8;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //POPDE
    |cpu: &mut Z80| {
        cpu.last_m = 3; cpu.last_t = 12;
        if cpu.f & CARRY_FLAG == 0 {
            cpu.pc = mem_access_w!(cpu.memory_unit, cpu.pc);
            cpu.last_m += 1; cpu.last_t += 4;
        } else { cpu.pc = cpu.pc.wrapping_add(2); }
    }, //JPNCnn
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {
        cpu.last_m = 3; cpu.last_t = 12;
        if cpu.f & CARRY_FLAG == 0 {
            cpu.pc = mem_access_w!(cpu.memory_unit, cpu.sp);
            cpu.sp = cpu.sp.wrapping_add(2);
            cpu.last_m += 2; cpu.last_t += 8;
        }
    }, //CALLNCnn
    |cpu: &mut Z80| {
        cpu.sp = cpu.sp.wrapping_sub(2);
        let value = (cpu.d as u16 << 8) + (cpu.e as u16);
        mem_access_w!(cpu.memory_unit, cpu.sp, value);
        cpu.last_m = 3; cpu.last_t = 12;
    }, //PUSHDE
    |cpu: &mut Z80| {
        let val = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let (i, b) = cpu.a.overflowing_sub(val);
        cpu.a = i; cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //SUBn
    |cpu: &mut Z80| {
        cpu.sp = cpu.sp.wrapping_sub(2);
        mem_access_w!(cpu.memory_unit, cpu.sp, cpu.pc);
        cpu.pc = 0x10;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //RST10
    |cpu: &mut Z80| {
        cpu.last_m = 1; cpu.last_t = 4;
        if cpu.f & CARRY_FLAG == CARRY_FLAG {
            cpu.pc = mem_access_w!(cpu.memory_unit, cpu.sp);
            cpu.sp = cpu.sp.wrapping_add(2);
            cpu.last_m += 2; cpu.last_t += 8;
        }
    }, //RETC
    |cpu: &mut Z80| {
        cpu.ime = true;
        cpu.pc = mem_access_w!(cpu.memory_unit, cpu.sp);
        cpu.sp = cpu.sp.wrapping_add(2);
        cpu.last_m = 3; cpu.last_t = 12;
    }, //RETI
    |cpu: &mut Z80| {
        cpu.last_m = 3; cpu.last_t = 12;
        if cpu.f & CARRY_FLAG == CARRY_FLAG {
            cpu.pc = mem_access_w!(cpu.memory_unit, cpu.pc);
            cpu.last_m += 1; cpu.last_t += 4;
        } else { cpu.pc = cpu.pc.wrapping_add(2); }
    }, //JPCnn
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {
        cpu.last_m = 3; cpu.last_t = 12;
        if cpu.f & CARRY_FLAG == CARRY_FLAG {
            cpu.sp = cpu.sp.wrapping_sub(2);
            mem_access_w!(cpu.memory_unit, cpu.sp, cpu.pc.wrapping_add(2));
            cpu.pc = mem_access_w!(cpu.memory_unit, cpu.pc);
            cpu.last_m += 2; cpu.last_t += 8;
        } else { cpu.pc = cpu.pc.wrapping_add(2); }
    }, //CALLCnn
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {}, //SBCn
    |cpu: &mut Z80| {
        cpu.sp = cpu.sp.wrapping_sub(2);
        mem_access_w!(cpu.memory_unit, cpu.sp, cpu.pc);
        cpu.pc = 0x18;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //RST18

    //e0
    |cpu: &mut Z80| {}, //LDIOnA
    |cpu: &mut Z80| {
        let value = mem_access_w!(cpu.memory_unit, cpu.sp);
        cpu.sp = cpu.sp.wrapping_add(2);
        cpu.h = value >> 8 as u8;
        cpu.l = value & 0xff as u8;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //POPHL
    |cpu: &mut Z80| {}, //LDIOCA
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {
        cpu.sp = cpu.sp.wrapping_sub(2);
        let value = (cpu.h as u16 << 8) + (cpu.l as u16);
        mem_access_w!(cpu.memory_unit, cpu.sp, value);
        cpu.last_m = 3; cpu.last_t = 12;
    }, //PUSHHL
    |cpu: &mut Z80| {
        cpu.a &= mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //ANDn
    |cpu: &mut Z80| {
        cpu.sp = cpu.sp.wrapping_sub(2);
        mem_access_w!(cpu.memory_unit, cpu.sp, cpu.pc);
        cpu.pc = 0x20;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //RST20
    |cpu: &mut Z80| {}, //ADDSPn
    |cpu: &mut Z80| {}, //JPHL
    |cpu: &mut Z80| {}, //LDmmA
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {}, //ORn
    |cpu: &mut Z80| {
        cpu.sp = cpu.sp.wrapping_sub(2);
        mem_access_w!(cpu.memory_unit, cpu.sp, cpu.pc);
        cpu.pc = 0x28;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //RST28

    //f0
    |cpu: &mut Z80| {}, //LDAIOn
    |cpu: &mut Z80| {
        let value = mem_access_w!(cpu.memory_unit, cpu.sp);
        cpu.sp = cpu.sp.wrapping_add(2);
        cpu.a = value >> 8 as u8;
        cpu.f = value & 0xff as u8;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //POPAF
    |cpu: &mut Z80| {}, //LDAIOC
    |cpu: &mut Z80| { cpu.ime = false; cpu.last_m = 1; cpu.last_t = 4; }, //DI
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {
        cpu.sp = cpu.sp.wrapping_sub(2);
        let value = (cpu.a as u16 << 8) + (cpu.f as u16);
        mem_access_w!(cpu.memory_unit, cpu.sp, value);
        cpu.last_m = 3; cpu.last_t = 12;
    }, //PUSHAF
    |cpu: &mut Z80| {
        cpu.a ^= mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //XORn
    |cpu: &mut Z80| {
        cpu.sp = cpu.sp.wrapping_sub(2);
        mem_access_w!(cpu.memory_unit, cpu.sp, cpu.pc);
        cpu.pc = 0x30;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //RST30
    |cpu: &mut Z80| {}, //LDHLSPn
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {
        let address = mem_access_w!(cpu.memory_unit, cpu.pc);
        cpu.a = mem_access_b!(cpu.memory_unit, address);
        cpu.pc = cpu.pc.wrapping_add(2);
        cpu.last_m = 4; cpu.last_t = 16;
    }, //LDAmm
    |cpu: &mut Z80| { cpu.ime = true; cpu.last_m = 1; cpu.last_t = 4; }, //EI
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {
        let val = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let (i, b) = cpu.a.overflowing_sub(val);
        cpu.f = SUB_FLAG;
        if i == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //CPn
    |cpu: &mut Z80| {
        cpu.sp = cpu.sp.wrapping_sub(2);
        mem_access_w!(cpu.memory_unit, cpu.sp, cpu.pc);
        cpu.pc = 0x38;
        cpu.last_m = 3; cpu.last_t = 12;
    }  //RST38
];