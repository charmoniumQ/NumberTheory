use std::fs::File;
use std::path::Path;
use std::fmt::Display;
use std::vec::Vec;
use std::io::Write;

error_chain! { }

pub fn to_csv(data: Vec<Vec<Box<Display>>>, dest: &Path) -> Result<()> {
    let mut file = File::create(dest).chain_err(|| "file no open")?;
    for row in data {
        for cell in row {
            write!(file, "{},", cell).chain_err(|| "writing to file")?;

        }
        write!(file, "\n").chain_err(|| "writing to file")?;
    }
    
    Ok(())
}

// pub fn concat<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
//     let mut c = Vec::<T>::new();
//     for elem in a {
//         c.push(elem);
//     }
//     for elem in b {
//         c.push(elem);
//     }
//     c
// }
