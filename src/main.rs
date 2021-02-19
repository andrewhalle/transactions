use std::env;
use std::fs::File;

use transactions::process::{self, State};
use transactions::Transaction;

fn main() {
    // open the input file
    let mut args = env::args();
    args.next().expect("first arg is executable name");
    let filename = args.next().expect("no filename provided");
    let f = File::open(filename).expect("could not open file");
    // TODO: do I need to buffer?

    // set up state
    let mut state = State::new();

    // process all transactions
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        if result.is_err() {
            /* handle the error as desired */
            continue;
        }

        let t: Transaction = result.expect("could not get transaction");
        if process::process_one(&mut state, t).is_err() { /* handle the error as desired */ }
    }

    // print output to stdout
    // TODO: do I need to lock this?
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    for (_, account) in state.accounts.iter() {
        wtr.serialize(account).expect("could not write record");
    }
    wtr.flush().expect("could not flush");
}
