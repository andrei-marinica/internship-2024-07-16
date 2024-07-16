#![allow(unused)]

mod enum_example;
mod factorial;
mod mymodule;

use enum_example::*;
use factorial::*;
use mymodule::*;
use num_bigint::BigUint;


fn print_big_uint(x: &BigUint) {
    println!("Big!, {x}!");
}

fn inc_big_uint(x: &mut BigUint) {
    *x += 1u32;
}

fn main() {
    println!("Hello, {}!", mymodule::TEXT);

    // let mut x: BigUint = BigUint::from(5u32);
    // let mut fc = FactorialComputation::new(x);
    // let mut fc = FactorialComputation::from(4u32);
    let mut fc: FactorialComputation = 4u32.into();
    // let mut fc = FactorialComputation::default();
    fc.inc_input();
    fc.inc_input();
    fc.inc_input();
    fc.inc_input();
    fc.inc_input();

    fc.pretty();
    fc.compute_mut();
    fc.pretty();

    print_juc(&DeJucarie::Egld(3));
    print_juc(&DeJucarie::Achievement("wow".into()));
}

