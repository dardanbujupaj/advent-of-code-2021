use clap::Parser;

mod lanternfish;
mod whale;

use lanternfish::lanternfish;
use whale::whale;

#[derive(Parser)]
#[clap(author = "Dardan Bujupaj")]
struct Opts {
    day: u8,
    input: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.day {
        6 => lanternfish(opts.input),
        7 => whale(opts.input),
        _ => unimplemented!(),
    }
}
