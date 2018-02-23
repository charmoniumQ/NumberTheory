#![feature(inclusive_range_syntax)]
#![feature(try_from)]
#![feature(inclusive_range)]
#![feature(step_trait)]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![recursion_limit = "1024"] // `error_chain!` can recurse deeply
#[macro_use]
#[warn(dead_code)]
extern crate error_chain;
extern crate image;
extern crate core;
//extern crate rayon;

//use std::path::Path;
mod afunc;

fn main2(k: usize, size: usize) {
    let div = afunc::AFunc::kary(k, size);
    println!("generated div");
    let muf = div.mu();
    println!("{:?}", muf)
    //div.draw_image(Path::new("test.png"));
}

fn main() {
    main2(20, 100);
}
