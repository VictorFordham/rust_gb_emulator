/*
CPU implementation based on: http://imrannazar.com/content/files/jsgb.z80.js
*/
use crate::mmu::MMU;
use crate::{
    ADCr_x, ADDr_x, ANDr_x, CPr_x, DECr_x, INCr_x, LDHLmr_x, LDrHLm_x, LDrr_xx, ORr_x,
    RSTx, SBCr_x, SRAr_x, SRLr_x, SUBr_x, SWAPr_x, undefined, XORr_x,
};
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
    pc: u16,
    sp: u16,
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

static undefined: fn(u16) = |x: u16| { panic!("Hit undefined instruction at {:?}", stringify!(x - 1)); };

static isa_map: [fn(&mut Z80); 256] = [

    //00
    |cpu: &mut Z80| { cpu.last_m = 1; cpu.last_t = 4; }, //NOP
    |cpu: &mut Z80| { //LDBCnn
        cpu.c = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.b = mem_access_b!(cpu.memory_unit, cpu.pc.wrapping_add(1));
        cpu.pc = cpu.pc.wrapping_add(2); cpu.last_m = 3; cpu.last_t = 12;
    },
    |cpu: &mut Z80| { //LDBCmA
        let mut address: u16 = cpu.b as u16;
        address = (address << 8) + cpu.c as u16;
        mem_access_b!(cpu.memory_unit, address, cpu.a);
        cpu.last_m = 2; cpu.last_t = 8;
    },
    |cpu: &mut Z80| { //INCBC
        cpu.c = cpu.c.wrapping_add(1); if cpu.c == 0 { cpu.b = cpu.b.wrapping_add(1); }
        cpu.last_m = 1; cpu.last_t = 4;
    },
    INCr_x!(b), //INCr_b
    DECr_x!(b), //DECr_b
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
        let hl = ((cpu.h as u16) << 8) + cpu.l as u16;
        let bc = ((cpu.b as u16) << 8) + cpu.c as u16;
        let (i, b) = hl.overflowing_add(bc);
        if b {
            cpu.f |= CARRY_FLAG;
        } else {
            cpu.f &= 0xff - CARRY_FLAG;
        }
        cpu.h = (hl >> 8) as u8;
        cpu.l = (hl & 0xff) as u8;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //ADDHLBC
    |cpu: &mut Z80| {
        let address = ((cpu.b as u16) << 8) + cpu.c as u16;
        cpu.a = mem_access_b!(cpu.memory_unit, address);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDABCm
    |cpu: &mut Z80| {
        cpu.c = cpu.c.wrapping_sub(1); if cpu.c == 0xff { cpu.b = cpu.b.wrapping_sub(1); }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //DECBC
    INCr_x!(c), //INCr_c
    DECr_x!(c), //DECr_c
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
        let mut address: u16 = cpu.d as u16;
        address = (address << 8) + cpu.e as u16;
        mem_access_b!(cpu.memory_unit, address, cpu.a);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDDEmA
    |cpu: &mut Z80| {
        cpu.e = cpu.e.wrapping_add(1);
        if cpu.e == 0 { cpu.d = cpu.d.wrapping_add(1); }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //INCDE
    INCr_x!(d), //INCr_d
    DECr_x!(d), //DECr_d
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
        let hl: u16 = ((cpu.h as u16) << 8) + cpu.l as u16;
        let de: u16 = ((cpu.d as u16) << 8) + cpu.e as u16;
        let (i, b) = hl.overflowing_add(de);
        cpu.h = (i >> 8) as u8;
        cpu.l = (i & 0xff) as u8;
        if b { cpu.f |= CARRY_FLAG; } else { cpu.f &= 0xff - CARRY_FLAG; }
        cpu.last_m = 3; cpu.last_t = 12;
    }, //ADDHLDE
    |cpu: &mut Z80| {
        let mut address: u16 = cpu.d as u16;
        address = (address << 8) + cpu.e as u16;
        cpu.a = mem_access_b!(cpu.memory_unit, address);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDADEm
    |cpu: &mut Z80| {
        cpu.e = cpu.e.wrapping_sub(1);
        if cpu.e == 0xff { cpu.d = cpu.d.wrapping_sub(1); }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //DECDE
    INCr_x!(e), //INCr_e
    DECr_x!(e), //DECr_e
    |cpu: &mut Z80| {
        cpu.e = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDrn_e
    |cpu: &mut Z80| {
        let ci = (cpu.f & CARRY_FLAG != 0) as u8 * 0x80;
        let co = (cpu.a & 1 != 0) as u8 * 0x10;
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
        let mut address: u16 = ((cpu.h as u16) << 8) + cpu.l as u16;
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
    INCr_x!(h), //INCr_h
    DECr_x!(h), //DECr_h
    |cpu: &mut Z80| {
        cpu.h = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDrn_h
    undefined!(),
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
        let hl: u16 = ((cpu.h as u16) << 8) + cpu.l as u16;
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
        let mut address: u16 = ((cpu.h as u16) << 8) + cpu.l as u16;
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
    INCr_x!(l), //INCr_l
    DECr_x!(l), //DECr_l
    |cpu: &mut Z80| {
        cpu.l = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDrn_l
    |cpu: &mut Z80| {
        cpu.a = !cpu.a;
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 1; cpu.last_t = 4;
    }, //CPL

    //30
    |cpu: &mut Z80| {
        cpu.last_m = 2; cpu.last_t = 8;
        let offset = mem_access_b!(cpu.memory_unit, cpu.pc) as i8;
        cpu.pc = cpu.pc.wrapping_add(1);
        if cpu.f & CARRY_FLAG == 0 {
            cpu.pc = (cpu.pc as i16 + offset as i16) as u16;
            cpu.last_m += 1; cpu.last_t += 4;
        }
    }, //JRNCn
    |cpu: &mut Z80| {
        cpu.sp = mem_access_w!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(2);
        cpu.last_m = 3; cpu.last_t = 12;
    }, //LDSPnn
    |cpu: &mut Z80| {
        let hl = ((cpu.h as u16) << 8) + cpu.l as u16;
        mem_access_b!(cpu.memory_unit, hl, cpu.a);
        let val = hl.wrapping_sub(1);
        cpu.h = (val >> 8) as u8;
        cpu.l = (val & 0xff) as u8;
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDHLDA
    |cpu: &mut Z80| { cpu.sp = cpu.sp.wrapping_add(1); cpu.last_m = 1; cpu.last_t = 4; }, //INCSP
    |cpu: &mut Z80| {
        let mut address = (cpu.h as u16) << 8;
        address += cpu.l as u16;
        let val = mem_access_b!(cpu.memory_unit, address).wrapping_add(1);
        mem_access_b!(cpu.memory_unit, address, val);
        cpu.f = 0;
        if val == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 3; cpu.last_t = 12;
    }, //INCHLm
    |cpu: &mut Z80| {
        let mut address = (cpu.h as u16) << 8;
        address += cpu.l as u16;
        let val = mem_access_b!(cpu.memory_unit, address).wrapping_sub(1);
        mem_access_b!(cpu.memory_unit, address, val);
        cpu.f = 0;
        if val == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 3; cpu.last_t = 12;
    }, //DECHLm
    |cpu: &mut Z80| {
        let address = ((cpu.h as u16) << 8) + cpu.l as u16;
        let val = mem_access_b!(cpu.memory_unit, cpu.pc);
        mem_access_b!(cpu.memory_unit, address, val);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.last_m = 3; cpu.last_t = 12;
    }, //LDHLmn
    |cpu: &mut Z80| { cpu.f |= CARRY_FLAG; cpu.last_m = 1; cpu.last_t = 4; }, //SCF
    |cpu: &mut Z80| {
        cpu.last_m = 2; cpu.last_t = 8;
        let offset = mem_access_b!(cpu.memory_unit, cpu.pc) as i8;
        cpu.pc = cpu.pc.wrapping_add(1);
        if cpu.f & CARRY_FLAG == CARRY_FLAG {
            cpu.pc = (cpu.pc as i16 + offset as i16) as u16;
            cpu.last_m += 1; cpu.last_t += 4;
        }
    }, //JRCn
    |cpu: &mut Z80| {
        let hl = ((cpu.h as u16) << 8) + cpu.l as u16;
        let (i, b) = hl.overflowing_add(cpu.sp);
        if b { cpu.f |= CARRY_FLAG; } else { cpu.f &= 0xff - CARRY_FLAG; }
        cpu.h = (i >> 8) as u8;
        cpu.l = (i & 0xff) as u8;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //ADDHLSP
    |cpu: &mut Z80| {
        let mut address = ((cpu.h as u16) << 8) + cpu.l as u16;
        cpu.a = mem_access_b!(cpu.memory_unit, address);
        address = address.wrapping_sub(1);
        cpu.h = (address >> 8) as u8;
        cpu.l = (address & 0xff) as u8;
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDAHLD
    |cpu: &mut Z80| { cpu.sp = cpu.sp.wrapping_sub(1); cpu.last_m = 1; cpu.last_t = 4; }, //DECSP
    INCr_x!(a), //INCr_a
    DECr_x!(a), //DECr_a
    |cpu: &mut Z80| {
        cpu.a = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDrn_a
    |cpu: &mut Z80| {cpu.f ^= CARRY_FLAG; cpu.last_m = 1; cpu.last_t = 4; }, //CCF

    //40
    LDrr_xx!(b, b), //LDrr_bb
    LDrr_xx!(b, c), //LDrr_bc
    LDrr_xx!(b, d), //LDrr_bd
    LDrr_xx!(b, e), //LDrr_be
    LDrr_xx!(b, h), //LDrr_bh
    LDrr_xx!(b, l), //LDrr_bl
    LDrHLm_x!(b), //LDrHLm_b
    LDrr_xx!(b, a), //Ldrr_ba
    LDrr_xx!(c, b), //LDrr_cb
    LDrr_xx!(c, c), //LDrr_cc
    LDrr_xx!(c, d), //LDrr_cd
    LDrr_xx!(c, e), //LDrr_ce
    LDrr_xx!(c, h), //LDrr_ch
    LDrr_xx!(c, l), //LDrr_cl
    LDrHLm_x!(c), //LDrHLm_c
    LDrr_xx!(c, a), //LDrr_ca

    //50
    LDrr_xx!(d, b), //LDrr_db
    LDrr_xx!(d, c), //LDrr_dc
    LDrr_xx!(d, d), //LDrr_dd
    LDrr_xx!(d, e), //LDrr_de
    LDrr_xx!(d, h), //LDrr_dh
    LDrr_xx!(d, l), //LDrr_dl
    LDrHLm_x!(d), //LDrHLm_d
    LDrr_xx!(d, a), //LDrr_da
    LDrr_xx!(e, b), //LDrr_eb
    LDrr_xx!(e, c), //LDrr_ec
    LDrr_xx!(e, d), //LDrr_ed
    LDrr_xx!(e, e), //LDrr_ee
    LDrr_xx!(e, h), //LDrr_eh
    LDrr_xx!(e, l), //LDrr_el
    LDrHLm_x!(e), //LDrHLm_e
    LDrr_xx!(e, a), //LDrr_ea

    //60
    LDrr_xx!(h, b), //LDrr_hb
    LDrr_xx!(h, c), //LDrr_hc
    LDrr_xx!(h, d), //LDrr_hd
    LDrr_xx!(h, e), //LDrr_he
    LDrr_xx!(h, h), //LDrr_hh
    LDrr_xx!(h, l), //LDrr_hl
    LDrHLm_x!(h), //LDrHLm_h
    LDrr_xx!(h, a), //LDrr_ha
    LDrr_xx!(l, b), //LDrr_lb
    LDrr_xx!(l, c), //LDrr_lc
    LDrr_xx!(l, d), //LDrr_ld
    LDrr_xx!(l, e), //LDrr_le
    LDrr_xx!(l, h), //LDrr_lh
    LDrr_xx!(l, l), //LDrr_ll
    LDrHLm_x!(l), //LDrHLm_l
    LDrr_xx!(l, a), //LDrr_la

    //70
    LDHLmr_x!(b), //LDHLmr_b
    LDHLmr_x!(c), //LDHLmr_c
    LDHLmr_x!(d), //LDHLmr_d
    LDHLmr_x!(e), //LDHLmr_e
    LDHLmr_x!(h), //LDHLmr_h
    LDHLmr_x!(l), //LDHLmr_l
    |cpu: &mut Z80| { cpu.halt = true; cpu.last_m = 1; cpu.last_t = 4; }, //halt
    LDHLmr_x!(a), //LDHLmr_a
    LDrr_xx!(a, b), //LDrr_ab
    LDrr_xx!(a, c), //LDrr_ac
    LDrr_xx!(a, d), //LDrr_ad
    LDrr_xx!(a, e), //LDrr_ae
    LDrr_xx!(a, h), //LDrr_ah
    LDrr_xx!(a, l), //LDrr_al
    LDrHLm_x!(a), //LDrHLm_a
    LDrr_xx!(a, a), //LDrr_aa

    //80
    ADDr_x!(b), //ADDr_b
    ADDr_x!(c), //ADDr_c
    ADDr_x!(d), //ADDr_d
    ADDr_x!(e), //ADDr_e
    ADDr_x!(h), //ADDr_h
    ADDr_x!(l), //ADDr_l
    |cpu: &mut Z80| {
        let address = ((cpu.h as u16) << 8) + cpu.l as u16;
        let (val, b) = cpu.a.overflowing_add(mem_access_b!(cpu.memory_unit, address));
        cpu.a = val;
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //ADDHL
    ADDr_x!(a), //ADDr_a
    ADCr_x!(b), //ADCr_b
    ADCr_x!(c), //ADCr_c
    ADCr_x!(d), //ADCr_d
    ADCr_x!(e), //ADCr_e
    ADCr_x!(h), //ADCr_h
    ADCr_x!(l), //ADCr_l
    |cpu: &mut Z80| {
        let mut address = (cpu.h as u16) << 8;
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
    ADCr_x!(a), //ADCr_a

    //90
    SUBr_x!(b), //SUBr_b
    SUBr_x!(c), //SUBr_c
    SUBr_x!(d), //SUBr_d
    SUBr_x!(e), //SUBr_e
    SUBr_x!(h), //SUBr_h
    SUBr_x!(l), //SUBr_l
    |cpu: &mut Z80| {
        let mut address = (cpu.h as u16) << 8;
        address += cpu.l as u16;
        let val = mem_access_b!(cpu.memory_unit, address);
        let (i, b) = cpu.a.overflowing_sub(val);
        cpu.a = i;
        cpu.f = SUB_FLAG;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //SUBHL
    SUBr_x!(a), //SUBr_a
    SBCr_x!(b), //SBCr_b
    SBCr_x!(c), //SBCr_c
    SBCr_x!(d), //SBCr_d
    SBCr_x!(e), //SBCr_e
    SBCr_x!(h), //SBCr_h
    SBCr_x!(l), //SBCr_l
    |cpu: &mut Z80| {
        let mut address = (cpu.h as u16) << 8;
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
    SBCr_x!(a), //SBCr_a

    //a0
    ANDr_x!(b), //ANDr_b
    ANDr_x!(c), //ANDr_c
    ANDr_x!(d), //ANDr_d
    ANDr_x!(e), //ANDr_e
    ANDr_x!(h), //ANDr_h
    ANDr_x!(l), //ANDr_l
    |cpu: &mut Z80| {
        let mut address = (cpu.h as u16) << 8;
        address += cpu.l as u16;
        cpu.a &= mem_access_b!(cpu.memory_unit, address);
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //ANDHL
    ANDr_x!(a), //ANDr_a
    XORr_x!(b), //XORr_b
    XORr_x!(c), //XORr_c
    XORr_x!(d), //XORr_d
    XORr_x!(e), //XORr_e
    XORr_x!(h), //XORr_h
    XORr_x!(l), //XORr_l
    |cpu: &mut Z80| {
        let mut address = (cpu.h as u16) << 8;
        address += cpu.l as u16;
        cpu.a ^= mem_access_b!(cpu.memory_unit, address);
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //XORHL
    XORr_x!(a), //XORr_a

    //b0
    ORr_x!(b), //ORr_b
    ORr_x!(c), //ORr_c
    ORr_x!(d), //ORr_d
    ORr_x!(e), //ORr_e
    ORr_x!(h), //ORr_h
    ORr_x!(l), //ORr_l
    |cpu: &mut Z80| {
        let mut address = (cpu.h as u16) << 8;
        address += cpu.l as u16;
        cpu.a |= mem_access_b!(cpu.memory_unit, address);
        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //ORHL
    ORr_x!(a), //ORr_a
    CPr_x!(b), //CPr_b
    CPr_x!(c), //CPr_c
    CPr_x!(d), //CPr_d
    CPr_x!(e), //CPr_e
    CPr_x!(h), //CPr_h
    CPr_x!(l), //CPr_l
    |cpu: &mut Z80| {
        let mut address = (cpu.h as u16) << 8;
        address += cpu.l as u16;
        let val = mem_access_b!(cpu.memory_unit, address);
        let (i, b) = cpu.a.overflowing_sub(val);
        cpu.f = SUB_FLAG;
        if i == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //CPHL
    CPr_x!(a), //CPr_a

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
        cpu.b = (value >> 8) as u8;
        cpu.c = (value & 0xff) as u8;
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
        let value = ((cpu.b as u16) << 8) + (cpu.c as u16);
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
    RSTx!(0x00), //RST00
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
    |cpu: &mut Z80| {
        let val = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        if val < 0x40 {
            map_table[val as usize](cpu);
        } else if val < 0x80 {
            const handlers: [fn(&mut Z80, u8); 8] = [
                |c, i| { if c.b & (1 << i) == 0 { c.f |= ZERO_FLAG; } c.last_m = 2; c.last_t = 8; },
                |c, i| { if c.c & (1 << i) == 0 { c.f |= ZERO_FLAG; } c.last_m = 2; c.last_t = 8; },
                |c, i| { if c.d & (1 << i) == 0 { c.f |= ZERO_FLAG; } c.last_m = 2; c.last_t = 8; },
                |c, i| { if c.e & (1 << i) == 0 { c.f |= ZERO_FLAG; } c.last_m = 2; c.last_t = 8; },
                |c, i| { if c.h & (1 << i) == 0 { c.f |= ZERO_FLAG; } c.last_m = 2; c.last_t = 8; },
                |c, i| { if c.l & (1 << i) == 0 { c.f |= ZERO_FLAG; } c.last_m = 2; c.last_t = 8; },
                |c, i| {
                    let addr = ((c.h as u16) << 8) + c.l as u16;
                    let j = mem_access_b!(c.memory_unit, addr);
                    if j & (1 << i) == 0 { c.f |= ZERO_FLAG; }
                    c.last_m = 3; c.last_t = 12;
                },
                |c, i| { if c.a & (1 << i) == 0 { c.f |= ZERO_FLAG; } c.last_m = 2; c.last_t = 8; }
            ];

            cpu.f = 0;
            let i: u8 = (val & 0b111000) >> 3;
            let index: usize = val as usize & 0b111;
            handlers[index](cpu, i);
        } else {
            panic!("Error on MAP instruction");
        }
    }, //MAPcb
    |cpu: &mut Z80| {
        cpu.last_m = 3; cpu.last_t = 12;
        if cpu.f & ZERO_FLAG == ZERO_FLAG {
            cpu.sp = cpu.sp.wrapping_sub(2);
            mem_access_w!(cpu.memory_unit, cpu.sp, cpu.pc.wrapping_add(2));
            cpu.pc = mem_access_w!(cpu.memory_unit, cpu.pc);
            cpu.last_m += 2; cpu.last_t += 8;
        } else { cpu.pc += 2; }
    }, //CALLZnn
    |cpu: &mut Z80| {
        cpu.sp = cpu.sp.wrapping_sub(2);
        mem_access_w!(cpu.memory_unit, cpu.sp, cpu.pc.wrapping_add(2));
        cpu.pc = mem_access_w!(cpu.memory_unit, cpu.pc);
        cpu.last_m = 5; cpu.last_t = 20;
    }, //CALLnn
    |cpu: &mut Z80| {}, //ADCn
    RSTx!(0x08), //RST08

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
        cpu.d = (value >> 8) as u8;
        cpu.e = (value & 0xff) as u8;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //POPDE
    |cpu: &mut Z80| {
        cpu.last_m = 3; cpu.last_t = 12;
        if cpu.f & CARRY_FLAG == 0 {
            cpu.pc = mem_access_w!(cpu.memory_unit, cpu.pc);
            cpu.last_m += 1; cpu.last_t += 4;
        } else { cpu.pc = cpu.pc.wrapping_add(2); }
    }, //JPNCnn
    undefined!(),
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
        let value = ((cpu.d as u16) << 8) + (cpu.e as u16);
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
    RSTx!(0x10), //RST10
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
    undefined!(),
    |cpu: &mut Z80| {
        cpu.last_m = 3; cpu.last_t = 12;
        if cpu.f & CARRY_FLAG == CARRY_FLAG {
            cpu.sp = cpu.sp.wrapping_sub(2);
            mem_access_w!(cpu.memory_unit, cpu.sp, cpu.pc.wrapping_add(2));
            cpu.pc = mem_access_w!(cpu.memory_unit, cpu.pc);
            cpu.last_m += 2; cpu.last_t += 8;
        } else { cpu.pc = cpu.pc.wrapping_add(2); }
    }, //CALLCnn
    undefined!(),
    |cpu: &mut Z80| {
        
    }, //SBCn
    RSTx!(0x18), //RST18

    //e0
    |cpu: &mut Z80| {
        let address = 0xff00 + (mem_access_b!(cpu.memory_unit, cpu.pc) as u16);
        cpu.pc = cpu.pc.wrapping_add(1);
        mem_access_b!(cpu.memory_unit, address, cpu.a);
        cpu.last_m = 3; cpu.last_t = 12;
    }, //LDIOnA
    |cpu: &mut Z80| {
        let value = mem_access_w!(cpu.memory_unit, cpu.sp);
        cpu.sp = cpu.sp.wrapping_add(2);
        cpu.h = (value >> 8) as u8;
        cpu.l = (value & 0xff) as u8;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //POPHL
    |cpu: &mut Z80| {
        mem_access_b!(cpu.memory_unit, 0xff00 + (cpu.c as u16), cpu.a);
        cpu.last_m = 2; cpu.last_t = 8;
    }, //LDIOCA
    undefined!(),
    undefined!(),
    |cpu: &mut Z80| {
        cpu.sp = cpu.sp.wrapping_sub(2);
        let value = ((cpu.h as u16) << 8) + (cpu.l as u16);
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
    RSTx!(0x20), //RST20
    |cpu: &mut Z80| {
        let offset = mem_access_b!(cpu.memory_unit, cpu.pc) as i8;
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.sp = ((cpu.sp as i16).wrapping_add(offset as i16)) as u16;
        cpu.last_m = 4; cpu.last_t = 16;
    }, //ADDSPn
    |cpu: &mut Z80| {
        cpu.pc = ((cpu.h as u16) << 8) + (cpu.l as u16);
        cpu.last_m = 1; cpu.last_t = 4;
    }, //JPHL
    |cpu: &mut Z80| {
        let address = mem_access_w!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(2);
        mem_access_b!(cpu.memory_unit, address, cpu.a);
        cpu.last_m = 4; cpu.last_t = 16;
    }, //LDmmA
    undefined!(),
    undefined!(),
    undefined!(),
    |cpu: &mut Z80| {
        cpu.a |= mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);

        cpu.f = 0;
        if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //ORn
    RSTx!(0x28), //RST28

    //f0
    |cpu: &mut Z80| {
        let val = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.a = mem_access_b!(cpu.memory_unit, 0xff00 + (val as u16));
        cpu.last_m = 3; cpu.last_t = 12;
    }, //LDAIOn
    |cpu: &mut Z80| {
        let value = mem_access_w!(cpu.memory_unit, cpu.sp);
        cpu.sp = cpu.sp.wrapping_add(2);
        cpu.a = (value >> 8) as u8;
        cpu.f = (value & 0xff) as u8;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //POPAF
    |cpu: &mut Z80| {
        cpu.a = mem_access_b!(cpu.memory_unit, 0xff00 + (cpu.c as u16));
        cpu.last_m = 3; cpu.last_t = 12;
    }, //LDAIOC
    |cpu: &mut Z80| { cpu.ime = false; cpu.last_m = 1; cpu.last_t = 4; }, //DI
    undefined!(),
    |cpu: &mut Z80| {
        cpu.sp = cpu.sp.wrapping_sub(2);
        let value = ((cpu.a as u16) << 8) + (cpu.f as u16);
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
    RSTx!(0x30), //RST30
    |cpu: &mut Z80| {
        let i = mem_access_b!(cpu.memory_unit, cpu.pc) as i8;
        let val = (i as i16).wrapping_add(cpu.sp as i16);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.h = (val >> 8) as u8;
        cpu.l = (val & 0xff) as u8;
        cpu.last_m = 3; cpu.last_t = 12;
    }, //LDHLSPn
    undefined!(),
    |cpu: &mut Z80| {
        let address = mem_access_w!(cpu.memory_unit, cpu.pc);
        cpu.a = mem_access_b!(cpu.memory_unit, address);
        cpu.pc = cpu.pc.wrapping_add(2);
        cpu.last_m = 4; cpu.last_t = 16;
    }, //LDAmm
    |cpu: &mut Z80| { cpu.ime = true; cpu.last_m = 1; cpu.last_t = 4; }, //EI
    undefined!(),
    undefined!(),
    |cpu: &mut Z80| {
        let val = mem_access_b!(cpu.memory_unit, cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let (i, b) = cpu.a.overflowing_sub(val);
        cpu.f = SUB_FLAG;
        if i == 0 { cpu.f |= ZERO_FLAG; }
        if b { cpu.f |= CARRY_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
    }, //CPn
    RSTx!(0x38)  //RST38
];

static map_table: [fn(&mut Z80); 64] = [

    //00
    |cpu: &mut Z80| {}, //RLCr_b
    |cpu: &mut Z80| {}, //RLCr_c
    |cpu: &mut Z80| {}, //RLCr_d
    |cpu: &mut Z80| {}, //RLCr_e
    |cpu: &mut Z80| {}, //RLCr_h
    |cpu: &mut Z80| {}, //RLCr_l
    |cpu: &mut Z80| {}, //RLCHL
    |cpu: &mut Z80| {}, //RLCr_a
    |cpu: &mut Z80| {}, //RRCr_b
    |cpu: &mut Z80| {}, //RRCr_c
    |cpu: &mut Z80| {}, //RRCr_d
    |cpu: &mut Z80| {}, //RRCr_e
    |cpu: &mut Z80| {}, //RRCr_h
    |cpu: &mut Z80| {}, //RRCr_l
    |cpu: &mut Z80| {}, //RRCHL
    |cpu: &mut Z80| {}, //RRCr_a

    //10
    |cpu: &mut Z80| {}, //RLr_b
    |cpu: &mut Z80| {}, //RLr_c
    |cpu: &mut Z80| {}, //RLr_d
    |cpu: &mut Z80| {}, //RLr_e
    |cpu: &mut Z80| {}, //RLr_h
    |cpu: &mut Z80| {}, //RLr_l
    |cpu: &mut Z80| {}, //RLHL
    |cpu: &mut Z80| {}, //RLr_a
    |cpu: &mut Z80| {}, //RRr_b
    |cpu: &mut Z80| {}, //RRr_c
    |cpu: &mut Z80| {}, //RRr_d
    |cpu: &mut Z80| {}, //RRr_e
    |cpu: &mut Z80| {}, //RRr_h
    |cpu: &mut Z80| {}, //RRr_l
    |cpu: &mut Z80| {}, //RRHL
    |cpu: &mut Z80| {}, //RRr_a

    //20
    |cpu: &mut Z80| {}, //SLAr_b
    |cpu: &mut Z80| {}, //SLAr_c
    |cpu: &mut Z80| {}, //SLAr_d
    |cpu: &mut Z80| {}, //SLAr_e
    |cpu: &mut Z80| {}, //SLAr_h
    |cpu: &mut Z80| {}, //SLAr_l
    undefined!(),
    |cpu: &mut Z80| {}, //SLAr_a
    SRAr_x!(b), //SRAr_b
    SRAr_x!(c), //SRAr_c
    SRAr_x!(d), //SRAr_d
    SRAr_x!(e), //SRAr_e
    SRAr_x!(h), //SRAr_h
    SRAr_x!(l), //SRAr_l
    undefined!(),
    SRAr_x!(a), //SRAr_a

    //30
    SWAPr_x!(b), //SWAPr_b
    SWAPr_x!(c), //SWAPr_c
    SWAPr_x!(d), //SWAPr_d
    SWAPr_x!(e), //SWAPr_e
    SWAPr_x!(h), //SWAPr_h
    SWAPr_x!(l), //SWAPr_l
    undefined!(),
    SWAPr_x!(a), //SWAPr_a
    SRLr_x!(b), //SRLr_b
    SRLr_x!(c), //SRLr_c
    SRLr_x!(d), //SRLr_d
    SRLr_x!(e), //SRLr_e
    SRLr_x!(h), //SRLr_h
    SRLr_x!(l), //SRLr_l
    undefined!(),
    SRLr_x!(a) //SRLr_a
];