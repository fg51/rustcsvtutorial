// tutorial-setup-01.rs

extern crate csv;

use std::io;
use std::process;


fn main() {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for i in rdr.records() {
        match i {
            Ok(record) => println!("{:?}", record),
            Err(err) => {
                println!("error reading CSV from <stdin>: {}", err);
                process::exit(1);
            }
        }
    }
}
