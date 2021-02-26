mod mmu;
mod cpu;

fn main() {
    println!("Hello, world!");

    let mut memory_unit = mmu::MMU::new();

    if memory_unit.set_b(5, 1).is_none() {
        println!("uh oh");
    }

    if let Some(n) = memory_unit.get_b(5) {
        println!("{}", n);
    }

    let mut t = || memory_unit.set_b(5, 3);
    t();
    //let t = mmu::MMU::set_b;
    //t(&mut memory_unit, 5, 3);

    let mut processor = cpu::Z80::new(memory_unit);

    println!("{}", processor.test());

    let i = processor.run();

    println!("{}", i);
}
