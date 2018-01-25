#![feature(inclusive_range_syntax)]
#![feature(try_from)]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![recursion_limit = "1024"] // `error_chain!` can recurse deeply
#[macro_use]
extern crate error_chain;

use std::convert::TryInto;
mod afunc;

fn main2(k: usize, max: usize) {
    let mut div = afunc::AFunc::d(max);
    for _ in 0..k {
        div = div.iterate();
    }
    let out: String = div.into();
    println!("{}", out);
    let inf: afunc::AFunc = out.try_into().unwrap();
}

fn main() {
    main2(10, 100);
}
