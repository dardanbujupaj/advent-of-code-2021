use clap::Parser;

mod lanternfish;

use lanternfish::lanternfish;

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
        _ => unimplemented!(),
    }
}
