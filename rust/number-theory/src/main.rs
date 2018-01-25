#![feature(inclusive_range_syntax)]
#![feature(try_from)]
#![feature(inclusive_range)]
#![feature(step_trait)]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![recursion_limit = "1024"] // `error_chain!` can recurse deeply
#[macro_use]
extern crate error_chain;

mod afunc;

fn main2(k: usize, max: usize) {
    let mut div = afunc::AFunc::d(max);
    for _ in 0..k {
        div = div.iterate();
    }
    let out = div.to_string();
    println!("{}", out);

    let div2 = afunc::AFunc::from_string(&out).unwrap();
    let out2 = div2.plaintext().join("\n");
    println!("{}", out2);

}

fn main() {
    main2(6, 15);
}
