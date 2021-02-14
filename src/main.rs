use std::env;
use std::fs::File;

use transactions::Transaction;

fn main() {
    let mut args = env::args();
    args.next().expect("first args is executable name");
    let filename = args.next().expect("no filename provided");

    let f = File::open(filename).expect("could not open file");

    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        let t: Transaction = result.expect("could not get transaction");
        println!("{:?}", t);
    }
}
