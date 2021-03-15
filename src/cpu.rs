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
    sp: usize,
    halt: bool
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
            halt: false
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

static set_flags = |cpu: &mut Z80| {
    //carry, when there is an overflow
    //half carry, similar but for the lower nibble of register
    //subtract flag
    //zero flag
};

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
    |cpu: &mut Z80| { //INCr_b FINISH ME !!
        cpu.b = cpu.b.wrapping_add(1);
    },
    |cpu: &mut Z80| {}, //DECr_b
    |cpu: &mut Z80| {}, //LDrn_b
    |cpu: &mut Z80| {}, //RLCA
    |cpu: &mut Z80| {}, //LDmmSP
    |cpu: &mut Z80| {}, //ADDHLBC
    |cpu: &mut Z80| {}, //LDABCm
    |cpu: &mut Z80| {}, //DECBC
    |cpu: &mut Z80| {}, //INCr_c
    |cpu: &mut Z80| {}, //DECr_c
    |cpu: &mut Z80| {}, //LDrn_c
    |cpu: &mut Z80| {}, //RRCA

    //10
    |cpu: &mut Z80| {}, //DJNZn
    |cpu: &mut Z80| {}, //LDDEnn
    |cpu: &mut Z80| { //LDDEmA
        let mut address: usize = cpu.d as usize;
        address = (address << 8) + cpu.e as usize;
        mem_access_b!(cpu.memory_unit, address, cpu.a);
        cpu.last_m = 2; cpu.last_t = 8;
    },
    |cpu: &mut Z80| {}, //INCDE
    |cpu: &mut Z80| {}, //INCr_d
    |cpu: &mut Z80| {}, //DECr_d
    |cpu: &mut Z80| {}, //LDrn_d
    |cpu: &mut Z80| {}, //RLA
    |cpu: &mut Z80| {}, //JRn
    |cpu: &mut Z80| {}, //ADDHLDE
    |cpu: &mut Z80| { //LDADEm
        let mut address: usize = cpu.d as usize;
        address = (address << 8) + cpu.e as usize;
        cpu.a = mem_access_b!(cpu.memory_unit, address);
        cpu.last_m = 2; cpu.last_t = 8;
    },
    |cpu: &mut Z80| {}, //DECDE
    |cpu: &mut Z80| {}, //INCr_e
    |cpu: &mut Z80| {}, //DECr_e
    |cpu: &mut Z80| {}, //LDrn_e
    |cpu: &mut Z80| {}, //RRA

    //20
    |cpu: &mut Z80| {}, //JRNZn
    |cpu: &mut Z80| {}, //LDHLnn
    |cpu: &mut Z80| {}, //LDHLIA
    |cpu: &mut Z80| {}, //INCHL
    |cpu: &mut Z80| {}, //INCr_h
    |cpu: &mut Z80| {}, //DECr_h
    |cpu: &mut Z80| {}, //LDrn_h
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {}, //JRZn
    |cpu: &mut Z80| {}, //ADDHLHL
    |cpu: &mut Z80| {}, //LDAHLI
    |cpu: &mut Z80| {}, //DECHL
    |cpu: &mut Z80| {}, //INCr_l
    |cpu: &mut Z80| {}, //DECr_l
    |cpu: &mut Z80| {}, //LDrn_l
    |cpu: &mut Z80| {}, //CPL

    //30
    |cpu: &mut Z80| {}, //JRNCn
    |cpu: &mut Z80| {}, //LDSPnn
    |cpu: &mut Z80| {}, //LDHLDA
    |cpu: &mut Z80| {}, //INCSP
    |cpu: &mut Z80| {}, //INCHLm
    |cpu: &mut Z80| {}, //DECHLm
    |cpu: &mut Z80| {}, //LDHLmn
    |cpu: &mut Z80| {}, //SCF
    |cpu: &mut Z80| {}, //JRCn
    |cpu: &mut Z80| {}, //ADDHLSP
    |cpu: &mut Z80| {}, //LDAHLD
    |cpu: &mut Z80| { //DECSP
        cpu.sp = cpu.sp.wrapping_sub(1);
        cpu.last_m = 1; cpu.last_t = 4;
    },
    |cpu: &mut Z80| {}, //INCr_a
    |cpu: &mut Z80| {}, //DECr_a
    |cpu: &mut Z80| {}, //LDrn_a
    |cpu: &mut Z80| {}, //CCF

    //40
    |cpu: &mut Z80| { cpu.b = cpu.b; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_bb
    |cpu: &mut Z80| { cpu.b = cpu.c; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_bc
    |cpu: &mut Z80| { cpu.b = cpu.d; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_bd
    |cpu: &mut Z80| { cpu.b = cpu.e; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_be
    |cpu: &mut Z80| { cpu.b = cpu.h; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_bh
    |cpu: &mut Z80| { cpu.b = cpu.l; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_bl
    |cpu: &mut Z80| {}, //LDrHLm_b
    |cpu: &mut Z80| { cpu.b = cpu.a; cpu.last_m = 1; cpu.last_t = 4; }, //Ldrr_ba
    |cpu: &mut Z80| { cpu.c = cpu.b; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_cb
    |cpu: &mut Z80| { cpu.c = cpu.c; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_cc
    |cpu: &mut Z80| { cpu.c = cpu.d; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_cd
    |cpu: &mut Z80| { cpu.c = cpu.e; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ce
    |cpu: &mut Z80| { cpu.c = cpu.h; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ch
    |cpu: &mut Z80| { cpu.c = cpu.l; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_cl
    |cpu: &mut Z80| {}, //LDrHLm_c
    |cpu: &mut Z80| { cpu.c = cpu.a; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ca

    //50
    |cpu: &mut Z80| { cpu.d = cpu.b; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_db
    |cpu: &mut Z80| { cpu.d = cpu.c; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_dc
    |cpu: &mut Z80| { cpu.d = cpu.d; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_dd
    |cpu: &mut Z80| { cpu.d = cpu.e; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_de
    |cpu: &mut Z80| { cpu.d = cpu.h; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_dh
    |cpu: &mut Z80| { cpu.d = cpu.l; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_dl
    |cpu: &mut Z80| {}, //LDrHLm_d
    |cpu: &mut Z80| { cpu.d = cpu.a; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_da
    |cpu: &mut Z80| { cpu.e = cpu.b; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_eb
    |cpu: &mut Z80| { cpu.e = cpu.c; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ec
    |cpu: &mut Z80| { cpu.e = cpu.d; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ed
    |cpu: &mut Z80| { cpu.e = cpu.e; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ee
    |cpu: &mut Z80| { cpu.e = cpu.h; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_eh
    |cpu: &mut Z80| { cpu.e = cpu.l; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_el
    |cpu: &mut Z80| {}, //LDrHLm_e
    |cpu: &mut Z80| { cpu.e = cpu.a; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ea

    //60
    |cpu: &mut Z80| { cpu.h = cpu.b; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_hb
    |cpu: &mut Z80| { cpu.h = cpu.c; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_hc
    |cpu: &mut Z80| { cpu.h = cpu.d; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_hd
    |cpu: &mut Z80| { cpu.h = cpu.e; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_he
    |cpu: &mut Z80| { cpu.h = cpu.h; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_hh
    |cpu: &mut Z80| { cpu.h = cpu.l; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_hl
    |cpu: &mut Z80| {}, //LDrHLm_h
    |cpu: &mut Z80| { cpu.h = cpu.a; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ha
    |cpu: &mut Z80| { cpu.l = cpu.b; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_lb
    |cpu: &mut Z80| { cpu.l = cpu.c; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_lc
    |cpu: &mut Z80| { cpu.l = cpu.d; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ld
    |cpu: &mut Z80| { cpu.l = cpu.e; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_le
    |cpu: &mut Z80| { cpu.l = cpu.h; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_lh
    |cpu: &mut Z80| { cpu.l = cpu.l; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_ll
    |cpu: &mut Z80| {}, //LDrHLm_l
    |cpu: &mut Z80| { cpu.l = cpu.a; cpu.last_m = 1; cpu.last_t = 4; }, //LDrr_la

    //70
    |cpu: &mut Z80| {}, //LDHLmr_b
    |cpu: &mut Z80| {}, //LDHLmr_c
    |cpu: &mut Z80| {}, //LDHLmr_d
    |cpu: &mut Z80| {}, //LDHLmr_e
    |cpu: &mut Z80| {}, //LDHLmr_h
    |cpu: &mut Z80| {}, //LDHLmr_l
    |cpu: &mut Z80| { cpu.halt = true; cpu.last_m = 1; cpu.last_t = 4; }, //halt
    |cpu: &mut Z80| {}, //LDHLmr_a
    |cpu: &mut Z80| {}, //LDrr_ab
    |cpu: &mut Z80| {}, //LDrr_ac
    |cpu: &mut Z80| {}, //LDrr_ad
    |cpu: &mut Z80| {}, //LDrr_ae
    |cpu: &mut Z80| {}, //LDrr_ah
    |cpu: &mut Z80| {}, //LDrr_al
    |cpu: &mut Z80| {}, //LDrHLm_a
    |cpu: &mut Z80| {}, //LDrr_aa

    //80
    |cpu: &mut Z80| {}, //ADDr_b
    |cpu: &mut Z80| {}, //ADDr_c
    |cpu: &mut Z80| {}, //ADDr_d
    |cpu: &mut Z80| {}, //ADDr_e
    |cpu: &mut Z80| {}, //ADDr_h
    |cpu: &mut Z80| {}, //ADDr_l
    |cpu: &mut Z80| {}, //ADDHL
    |cpu: &mut Z80| {}, //ADDr_a
    |cpu: &mut Z80| {}, //ADCr_b
    |cpu: &mut Z80| {}, //ADCr_c
    |cpu: &mut Z80| {}, //ADCr_d
    |cpu: &mut Z80| {}, //ADCr_e
    |cpu: &mut Z80| {}, //ADCr_h
    |cpu: &mut Z80| {}, //ADCr_l
    |cpu: &mut Z80| {}, //ADCHL
    |cpu: &mut Z80| {}, //ADCr_a

    //90
    |cpu: &mut Z80| {}, //SUBr_b
    |cpu: &mut Z80| {}, //SUBr_c
    |cpu: &mut Z80| {}, //SUBr_d
    |cpu: &mut Z80| {}, //SUBr_e
    |cpu: &mut Z80| {}, //SUBr_h
    |cpu: &mut Z80| {}, //SUBr_l
    |cpu: &mut Z80| {}, //SUBHL
    |cpu: &mut Z80| {}, //SUBr_a
    |cpu: &mut Z80| {}, //SBCr_b
    |cpu: &mut Z80| {}, //SBCr_c
    |cpu: &mut Z80| {}, //SBCr_d
    |cpu: &mut Z80| {}, //SBCr_e
    |cpu: &mut Z80| {}, //SBCr_h
    |cpu: &mut Z80| {}, //SBCr_l
    |cpu: &mut Z80| {}, //SBCHL
    |cpu: &mut Z80| {}, //SBCr_a

    //a0
    |cpu: &mut Z80| {}, //ANDr_b
    |cpu: &mut Z80| {}, //ANDr_c
    |cpu: &mut Z80| {}, //ANDr_d
    |cpu: &mut Z80| {}, //ANDr_e
    |cpu: &mut Z80| {}, //ANDr_h
    |cpu: &mut Z80| {}, //ANDr_l
    |cpu: &mut Z80| {}, //ANDHL
    |cpu: &mut Z80| {}, //ANDr_a
    |cpu: &mut Z80| {}, //XORr_b
    |cpu: &mut Z80| {}, //XORr_c
    |cpu: &mut Z80| {}, //XORr_d
    |cpu: &mut Z80| {}, //XORr_e
    |cpu: &mut Z80| {}, //XORr_h
    |cpu: &mut Z80| {}, //XORr_l
    |cpu: &mut Z80| {}, //XORHL
    |cpu: &mut Z80| {}, //XORr_a

    //b0
    |cpu: &mut Z80| {}, //ORr_b
    |cpu: &mut Z80| {}, //ORr_c
    |cpu: &mut Z80| {}, //ORr_d
    |cpu: &mut Z80| {}, //ORr_e
    |cpu: &mut Z80| {}, //ORr_h
    |cpu: &mut Z80| {}, //ORr_l
    |cpu: &mut Z80| {}, //ORHL
    |cpu: &mut Z80| {}, //ORr_a
    |cpu: &mut Z80| {}, //CPr_b
    |cpu: &mut Z80| {}, //CPr_c
    |cpu: &mut Z80| {}, //CPr_d
    |cpu: &mut Z80| {}, //CPr_e
    |cpu: &mut Z80| {}, //CPr_h
    |cpu: &mut Z80| {}, //CPr_l
    |cpu: &mut Z80| {}, //CPHL
    |cpu: &mut Z80| {}, //CPr_a

    //c0
    |cpu: &mut Z80| {}, //RETNZ
    |cpu: &mut Z80| {
        let value = mem_access_w!(Z80.memory_unit, Z80.sp);
        Z80.sp = Z80.sp.wrapping_add(2);
        Z80.b = value >> 8 as u8;
        Z80.c = value & 0xff as u8;
        Z80.last_m = 3; Z80.last_t = 12;
    }, //POPBC
    |cpu: &mut Z80| {
        Z80.last_m = 3; Z80.last_t = 12;
        if (Z80.f & 0x80) == 0x00 {
            Z80.pc = mem_access_w!(Z80.memory_unit, Z80.pc);
            Z80.last_m += 1; Z80.last_t += 4;
        } else { Z80.pc = Z80.pc.wrapping_add(2); }
    }, //JPNZnn
    |cpu: &mut Z80| {
        Z80.pc = mem_access_w!(cpu.memory_unit, cpu.pc);
        Z80.last_m = 3; Z80.last_t = 12;
    }, //JPnn
    |cpu: &mut Z80| {}, //CALLNZnn
    |cpu: &mut Z80| {
        Z80.sp = Z80.sp.wrapping_sub(2);
        let value = (Z80.b as u16 << 8) + (Z80.c as u16);
        mem_access_w!(Z80.memory_unit, Z80.sp, value);
        Z80.last_m = 3; Z80.last_t = 12;
    }, //PUSHBC
    |cpu: &mut Z80| {}, //ADDn
    |cpu: &mut Z80| {
        Z80.sp = Z80.sp.wrapping_sub(2);
        mem_access_w!(Z80.memory_unit, Z80.sp, Z80.pc);
        Z80.pc = 0x00;
        Z80.last_m = 3; Z80.last_t = 12;
    }, //RST00
    |cpu: &mut Z80| {}, //RETZ
    |cpu: &mut Z80| {
        Z80.pc = mem_access_w!(Z80.memory_unit, Z80.sp); Z80.sp += 2;
        Z80.last_m = 3; Z80.last_t = 12;
    }, //RET
    |cpu: &mut Z80| {
        Z80.last_m = 3; Z80.last_t = 12;
        if (Z80.f & 0x80) == 0x80 {
            Z80.pc = mem_access_w!(Z80.memory_unit, Z80.pc);
            Z80.last_m += 1; Z80.last_t += 4;
        } else { Z80.pc = Z80.pc.wrapping_add(2); }
    }, //JPZnn
    |cpu: &mut Z80| {}, //MAPcb
    |cpu: &mut Z80| {}, //CALLZnn
    |cpu: &mut Z80| {}, //CALLnn
    |cpu: &mut Z80| {}, //ADCn
    |cpu: &mut Z80| {
        Z80.sp = Z80.sp.wrapping_sub(2);
        mem_access_w!(Z80.memory_unit, Z80.sp, Z80.pc);
        Z80.pc = 0x08;
        Z80.last_m = 3; Z80.last_t = 12;
    }, //RST08

    //d0
    |cpu: &mut Z80| {}, //RETNC
    |cpu: &mut Z80| {
        let value = mem_access_w!(Z80.memory_unit, Z80.sp);
        Z80.sp = Z80.sp.wrapping_add(2);
        Z80.d = value >> 8 as u8;
        Z80.e = value & 0xff as u8;
        Z80.last_m = 3; Z80.last_t = 12;
    }, //POPDE
    |cpu: &mut Z80| {}, //JPNCnn
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {}, //CALLNCnn
    |cpu: &mut Z80| {
        Z80.sp = Z80.sp.wrapping_sub(2);
        let value = (Z80.d as u16 << 8) + (Z80.e as u16);
        mem_access_w!(Z80.memory_unit, Z80.sp, value);
        Z80.last_m = 3; Z80.last_t = 12;
    }, //PUSHDE
    |cpu: &mut Z80| {}, //SUBn
    |cpu: &mut Z80| {
        Z80.sp = Z80.sp.wrapping_sub(2);
        mem_access_w!(Z80.memory_unit, Z80.sp, Z80.pc);
        Z80.pc = 0x10;
        Z80.last_m = 3; Z80.last_t = 12;
    }, //RST10
    |cpu: &mut Z80| {}, //RETC
    |cpu: &mut Z80| {}, //RETI
    |cpu: &mut Z80| {}, //JPCnn
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {}, //CALLCnn
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {}, //SBCn
    |cpu: &mut Z80| {
        Z80.sp = Z80.sp.wrapping_sub(2);
        mem_access_w!(Z80.memory_unit, Z80.sp, Z80.pc);
        Z80.pc = 0x18;
        Z80.last_m = 3; Z80.last_t = 12;
    }, //RST18

    //e0
    |cpu: &mut Z80| {}, //LDIOnA
    |cpu: &mut Z80| {
        let value = mem_access_w!(Z80.memory_unit, Z80.sp);
        Z80.sp = Z80.sp.wrapping_add(2);
        Z80.h = value >> 8 as u8;
        Z80.l = value & 0xff as u8;
        Z80.last_m = 3; Z80.last_t = 12;
    }, //POPHL
    |cpu: &mut Z80| {}, //LDIOCA
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {
        Z80.sp = Z80.sp.wrapping_sub(2);
        let value = (Z80.h as u16 << 8) + (Z80.l as u16);
        mem_access_w!(Z80.memory_unit, Z80.sp, value);
        Z80.last_m = 3; Z80.last_t = 12;
    }, //PUSHHL
    |cpu: &mut Z80| {}, //ANDn
    |cpu: &mut Z80| {
        Z80.sp = Z80.sp.wrapping_sub(2);
        mem_access_w!(Z80.memory_unit, Z80.sp, Z80.pc);
        Z80.pc = 0x20;
        Z80.last_m = 3; Z80.last_t = 12;
    }, //RST20
    |cpu: &mut Z80| {}, //ADDSPn
    |cpu: &mut Z80| {}, //JPHL
    |cpu: &mut Z80| {}, //LDmmA
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {}, //ORn
    |cpu: &mut Z80| {
        Z80.sp = Z80.sp.wrapping_sub(2);
        mem_access_w!(Z80.memory_unit, Z80.sp, Z80.pc);
        Z80.pc = 0x28;
        Z80.last_m = 3; Z80.last_t = 12;
    }, //RST28

    //f0
    |cpu: &mut Z80| {}, //LDAIOn
    |cpu: &mut Z80| {
        let value = mem_access_w!(Z80.memory_unit, Z80.sp);
        Z80.sp = Z80.sp.wrapping_add(2);
        Z80.a = value >> 8 as u8;
        Z80.f = value & 0xff as u8;
        Z80.last_m = 3; Z80.last_t = 12;
    }, //POPAF
    |cpu: &mut Z80| {}, //LDAIOC
    |cpu: &mut Z80| {}, //DI
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {
        Z80.sp = Z80.sp.wrapping_sub(2);
        let value = (Z80.a as u16 << 8) + (Z80.f as u16);
        mem_access_w!(Z80.memory_unit, Z80.sp, value);
        Z80.last_m = 3; Z80.last_t = 12;
    }, //PUSHAF
    |cpu: &mut Z80| {}, //XORn
    |cpu: &mut Z80| {
        Z80.sp = Z80.sp.wrapping_sub(2);
        mem_access_w!(Z80.memory_unit, Z80.sp, Z80.pc);
        Z80.pc = 0x30;
        Z80.last_m = 3; Z80.last_t = 12;
    }, //RST30
    |cpu: &mut Z80| {}, //LDHLSPn
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {}, //LDAmm
    |cpu: &mut Z80| {}, //EI
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| { undefined(cpu.pc); },
    |cpu: &mut Z80| {}, //CPn
    |cpu: &mut Z80| {
        Z80.sp = Z80.sp.wrapping_sub(2);
        mem_access_w!(Z80.memory_unit, Z80.sp, Z80.pc);
        Z80.pc = 0x38;
        Z80.last_m = 3; Z80.last_t = 12;
    }  //RST38
];