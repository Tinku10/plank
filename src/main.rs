#![feature(bufreader_peek)]
#![feature(iter_array_chunks)]

mod file;
mod serde;
mod types;

use crate::file::reader::PlankReader;
use crate::file::writer::PlankWriter;

fn main() {
    // let mut f = PlankWriter::new("./data/addresses.plank").unwrap();
    // f.write_from_csv("./data/addresses.csv").unwrap();

    let mut f = PlankReader::open("./data/addresses.plank").unwrap();

    for rg in &mut f {
        if let Ok(rg) = rg {
            for row in rg {
                println!("{:?}", row);
            }
        }
    }
}
