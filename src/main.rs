#![feature(bufreader_peek)]

mod file;
mod serde;
// mod util;

use crate::file::rowgroup::column::Column;
use crate::file::rowgroup::RowGroup;
use crate::serde::Serialize;
use crate::file::{SF2Reader};

fn main() {
    // let rg = vec![RowGroup::new(
    //     vec![
    //         Column::new(vec![
    //             "1".to_string(),
    //             "2".to_string(),
    //             "3".to_string(),
    //             "4".to_string(),
    //         ]),
    //         Column::new(vec![
    //             "A".to_string(),
    //             "B".to_string(),
    //             "C".to_string(),
    //             "D".to_string(),
    //         ]),
    //     ],
    // )];
    // let f = footer::Footer::new(
    //     vec![
    //         ("Firstname".to_string(), "str".to_string()),
    //         ("Lastname".to_string(), "str".to_string()),
    //     ],
    //     vec![10, 20, 30, 40, 50, 60],
    //     10,
    //     2,
    // );

    // util::csv_to_sf2("./data/addresses.csv").unwrap().write("./data/test.sf2").unwrap();
    //
    //
    let mut f = SF2Reader::open("./data/test.sf2").unwrap();
    // println!("{:?}", f.head(Some(6)).unwrap())

    for row in f.iter() {
        println!("{:?}", row);
    }


    // SF2::new(rg, f).write("./data/test.sf2");

    // println!("{}", f.to_string());
}
