// tutorial-read-serde-02

extern crate csv;

use std::error::Error;
use std::io;
use std::process;

type Record = (String, String, Option<u64>, f64, f64);

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for i in rdr.deserialize() {
        let record: Record = i?;
        println!("{:?}", record);
    }
    Ok(())
}
