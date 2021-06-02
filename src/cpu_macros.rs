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