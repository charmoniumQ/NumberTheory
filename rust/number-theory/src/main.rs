#![feature(try_from)]
#![feature(inclusive_range)]
#![feature(step_trait)]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![recursion_limit = "1024"] // `error_chain!` can recurse deeply
#[macro_use]
#[warn(dead_code)]
extern crate error_chain;
extern crate image;
extern crate scarlet;
extern crate core;
extern crate time;
extern crate rayon;

use std::fmt::Display;
use std::path::Path;
mod afunc;
mod util;

fn main2(k: usize, size: usize) {
    // TODO: rm -rf and mkdir output/

    let mut start_time = time::now();
    let chartri = afunc::CharTri::kary(size);
    println!("Chartri {}", (time::now() - start_time).num_milliseconds());

    let mut mus = Vec::<Vec<i16>>::new();
    let mut afunc = 0; let mut image = 0; let mut must = 0;
    for i in 0..k {

        start_time = time::now();
        let div = chartri.afunc(i);
        afunc += (time::now() - start_time).num_milliseconds();

        start_time = time::now();
        div.draw_image(Path::new(&format!("output/tri/{}.png", i)));
        image += (time::now() - start_time).num_milliseconds();

        start_time = time::now();
        mus.push(div.mu());
        must += (time::now() - start_time).num_milliseconds();
    }
    println!("Afunc   {}", afunc);
    println!("Image   {}", image);
    println!("Must    {}", must);

    let mut rows = Vec::<Vec<Box<Display>>>::new();

    let mut first_row = Vec::<Box<Display>>::new();
    first_row.push(Box::new("k"));
    first_row.push(Box::new("mu(k)"));
    rows.push(first_row);

    start_time = time::now();
    for (i, mu) in mus.iter().enumerate() {
        let mut row = Vec::<Box<Display>>::new();
        row.push(Box::new(i));
        for e in mu {
            row.push(Box::new(e.clone()));
        }
        rows.push(row);
    }
    util::to_csv(rows, Path::new("output/mus.csv")).unwrap();
    println!("CSV     {}", (time::now() - start_time).num_milliseconds());

    start_time = time::now();
    chartri.draw_image(Path::new("output/chartri.png"));
    println!("Chartri {}", (time::now() - start_time).num_milliseconds());
}

fn main() {
    main2(300, 300);
}
