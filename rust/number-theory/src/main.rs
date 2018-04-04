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

use std::fmt::Display;
use std::path::Path;
mod afunc;
mod util;

fn main2(k: usize, size: usize) {
    // TODO: mkdir
    let chartri = afunc::CharTri::kary(size);
    println!("Chartri");

    let mut mus = Vec::<Vec<i16>>::new();
    for i in 0..k {
        println!("Image {}", i);
        let div = chartri.afunc(i);
        // div.draw_image(Path::new(&format!("output/tri/{}.png", i)));
        mus.push(div.mu());
    }
    println!("Mus");

    let mut rows = Vec::<Vec<Box<Display>>>::new();

    let mut first_row = Vec::<Box<Display>>::new();
    first_row.push(Box::new("k"));
    first_row.push(Box::new("mu(k)"));
    rows.push(first_row);

    for (i, mu) in mus.iter().enumerate() {
        let mut row = Vec::<Box<Display>>::new();
        row.push(Box::new(i));
        for e in mu {
            row.push(Box::new(e.clone()));
        }
        rows.push(row);
    }
    println!("Mus2");
    util::to_csv(rows, Path::new("output/mus.csv")).unwrap();

    // chartri.draw_image(Path::new("output/chartri.png"));
}

fn main() {
    main2(500, 500);
}
