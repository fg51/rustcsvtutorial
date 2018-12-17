// tutorial-setup-01.rs

extern crate csv;

use std::io;


fn main() {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for i in rdr.records() {
        let record = i.expect("a CSV record");
        println!("{:?}", record);
    }
}
