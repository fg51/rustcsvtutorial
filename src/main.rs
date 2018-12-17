// tutorial-setup-01.rs

extern crate csv;

use std::error::Error;
use std::io;
use std::process;


fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for i in rdr.records() {
        let record = i?;
        println!("{:?}", record);
    }
    Ok(())
}
