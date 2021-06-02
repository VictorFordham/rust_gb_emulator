#[macro_export]
macro_rules! ADCr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            let (mut val, b) = cpu.a.overflowing_add(cpu.$reg);
            if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_add(1); }
            cpu.f = 0;
            if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
            if b || cpu.a > val { cpu.f |= CARRY_FLAG; }
            cpu.a = val;
            cpu.last_m = 1; cpu.last_t = 4;
        }
    }
}

#[macro_export]
macro_rules! ADDr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            let (val, b) = cpu.a.overflowing_add(cpu.$reg);
            cpu.a = val;
            cpu.f = 0;
            if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
            if b { cpu.f |= CARRY_FLAG; }
            cpu.last_m = 1; cpu.last_t = 4;
        }
    }
}

#[macro_export]
macro_rules! ANDr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            cpu.a &= cpu.$reg;
            cpu.f = 0;
            if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
            cpu.last_m = 1; cpu.last_t = 4;
        }
    }
}

#[macro_export]
macro_rules! CPr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            let (i, b) = cpu.a.overflowing_sub(cpu.$reg);
            cpu.f = SUB_FLAG;
            if i == 0 { cpu.f |= ZERO_FLAG; }
            if b { cpu.f |= CARRY_FLAG; }
            cpu.last_m = 1; cpu.last_t = 4;
        }
    }
}

#[macro_export]
macro_rules! DECr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            cpu.$reg = cpu.$reg.wrapping_sub(1);
            cpu.f = 0;
            if cpu.$reg == 0 { cpu.f |= ZERO_FLAG; }
            cpu.last_m = 1; cpu.last_t = 4;
        }
    }
}

#[macro_export]
macro_rules! INCr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            cpu.$reg = cpu.$reg.wrapping_add(1);
            cpu.f = 0;
            if cpu.$reg == 0 { cpu.f |= ZERO_FLAG; }
            cpu.last_m = 1; cpu.last_t = 4;
        }
    }
}

#[macro_export]
macro_rules! LDHLmr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            let mut address = (cpu.h as u16) << 8;
            address += cpu.l as u16;
            mem_access_b!(cpu.memory_unit, address, cpu.$reg);
            cpu.last_m = 2; cpu.last_t = 8;
        }
    }
}

#[macro_export]
macro_rules! LDrHLm_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            let mut address = (cpu.h as u16) << 8;
            address += cpu.l as u16;
            cpu.$reg = mem_access_b!(cpu.memory_unit, address);
            cpu.last_m = 2; cpu.last_t = 8;
        }
    }
}

#[macro_export]
macro_rules! LDrr_xx {
    ($dst_reg:ident, $src_reg:ident) => {
        |cpu: &mut Z80| { cpu.$dst_reg = cpu.$src_reg; cpu.last_m = 1; cpu.last_t = 4; }
    }
}

#[macro_export]
macro_rules! ORr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            cpu.a |= cpu.$reg;
            cpu.f = 0;
            if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
            cpu.last_m = 1; cpu.last_t = 4;
        }
    }
}

#[macro_export]
macro_rules! RLCr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            let carry = (cpu.$reg & 0x80 != 0) as u8;
            cpu.f = 0;
            if cpu.$reg & 0x80 != 0 { cpu.f |= CARRY_FLAG; }
            cpu.$reg <<= 1;
            cpu.$reg |= carry;
            if cpu.$reg == 0 { cpu.f |= ZERO_FLAG; }
            cpu.last_m = 2; cpu.last_t = 8;
        }
    }
}

#[macro_export]
macro_rules! RLr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            let carry = (cpu.f & CARRY_FLAG != 0) as u8;
            cpu.f = 0;
            if cpu.$reg & 1 != 0 { cpu.f |= CARRY_FLAG; }
            cpu.$reg <<= 1;
            cpu.$reg |= carry;
            if cpu.$reg == 0 { cpu.f |= ZERO_FLAG; }
            cpu.last_m = 2; cpu.last_t = 8;
        }
    }
}

#[macro_export]
macro_rules! RRCr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            let carry = ((cpu.$reg & 1 != 0) as u8) * 0x80;
            cpu.f = 0;
            if cpu.$reg & 1 != 0 { cpu.f |= CARRY_FLAG; }
            cpu.$reg >>= 1;
            cpu.$reg |= carry;
            if cpu.$reg == 0 { cpu.f |= ZERO_FLAG; }
            cpu.last_m = 2; cpu.last_t = 8;
        }
    }
}

#[macro_export]
macro_rules! RRr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            let val = ((cpu.f & CARRY_FLAG != 0) as u8) * 0x80;
            cpu.f = 0;
            if cpu.$reg & 1 != 0 { cpu.f |= CARRY_FLAG; }
            cpu.$reg >>= 1;
            cpu.$reg |= val;
            if cpu.$reg == 0 { cpu.f |= ZERO_FLAG; }
            cpu.last_m = 2; cpu.last_t = 8;
        }
    }
}

#[macro_export]
macro_rules! RSTx {
    ($offset:expr) => {
        |cpu: &mut Z80| {
            cpu.sp = cpu.sp.wrapping_sub(2);
            mem_access_w!(cpu.memory_unit, cpu.sp, cpu.pc);
            cpu.pc = $offset;
            cpu.last_m = 3; cpu.last_t = 12;
        }
    }
}

#[macro_export]
macro_rules! SBCr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            let (mut val, b) = cpu.a.overflowing_sub(cpu.$reg);
            if cpu.f & CARRY_FLAG != 0 { val = val.wrapping_sub(1); }
            cpu.f = SUB_FLAG;
            if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
            if b || cpu.a < val { cpu.f |= CARRY_FLAG; }
            cpu.a = val;
            cpu.last_m = 1; cpu.last_t = 4;
        }
    }
}

#[macro_export]
macro_rules! SLAr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            cpu.f = 0;
            if cpu.$reg & 0x80 != 0 { cpu.f |= CARRY_FLAG; }
            cpu.$reg <<= 1;
            if cpu.$reg == 0 { cpu.f |= ZERO_FLAG; }
            cpu.last_m = 2; cpu.last_t = 8;
        }
    }
}

#[macro_export]
macro_rules! SRAr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| { 
        let val = cpu.$reg & 0x80;
        cpu.f = 0;

        if cpu.$reg != 0 {
            cpu.f |= CARRY_FLAG;
        }
        cpu.$reg = (cpu.$reg >> 1) + val;

        if cpu.$reg == 0 { cpu.f |= ZERO_FLAG; }
        cpu.last_m = 2; cpu.last_t = 8;
        }
    }
}

#[macro_export]
macro_rules! SRLr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| { 
            cpu.f = 0;
            if cpu.$reg >> 1 == 0 { cpu.f = ZERO_FLAG; }
            if cpu.$reg & 1 != 0 { cpu.f |= CARRY_FLAG;}
            cpu.$reg >>= 1;
            cpu.last_m = 2; cpu.last_t = 8;
        }
    }
}

#[macro_export]
macro_rules! SUBr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            let (val, b) = cpu.a.overflowing_sub(cpu.$reg);
            cpu.a = val;
            cpu.f = SUB_FLAG;
            if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
            if b { cpu.f |= CARRY_FLAG; }
            cpu.last_m = 1; cpu.last_t = 4;
        }
    }
}

#[macro_export]
macro_rules! SWAPr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| { 
        let address = ((cpu.h as u16) << 8) + (cpu.l as u16);
        let tmp = cpu.$reg;
        cpu.$reg = mem_access_b!(cpu.memory_unit, address);
        mem_access_b!(cpu.memory_unit, address, tmp);
        cpu.last_m = 4; cpu.last_t = 16;
        }
    }
}

#[macro_export]
macro_rules! undefined {
    () => {
        |cpu: &mut Z80| { panic!("Hit undefined instruction at {:?}", stringify!(cpu.pc - 1)); }
    }
}

#[macro_export]
macro_rules! XORr_x {
    ($reg:ident) => {
        |cpu: &mut Z80| {
            cpu.a ^= cpu.$reg;
            cpu.f = 0;
            if cpu.a == 0 { cpu.f |= ZERO_FLAG; }
            cpu.last_m = 1; cpu.last_t = 4;
        }
    }
}