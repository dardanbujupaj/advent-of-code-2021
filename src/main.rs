use clap::Parser;

mod lanternfish;
mod sonar_sweep;
mod whale;

use lanternfish::lanternfish;
use sonar_sweep::sonar_sweep;
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
        1 => sonar_sweep(opts.input),
        6 => lanternfish(opts.input),
        7 => whale(opts.input),
        _ => unimplemented!(),
    }
}
