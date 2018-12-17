// tutorial-read-serde-01

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

        let city = &record[0];
        let state = &record[1];

        let pop: Option<u64> = record[2].parse().ok();
        let latitude: f64 = record[3].parse()?;
        let longitude: f64 = record[4].parse()?;

        println!(
            "city: {:?}, state: {:?}, \
             pop: {:?}, latitude: {:?}, longitude: {:?}",
             city, state, pop, latitude, longitude);
    }
    Ok(())
}
