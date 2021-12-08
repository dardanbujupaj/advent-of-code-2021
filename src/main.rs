use clap::Parser;

mod util;

mod binary_diagnostic;
mod dive;
mod hydrothermal_venture;
mod lanternfish;
mod seven_segment;
mod sonar_sweep;
mod whale;

use binary_diagnostic::binary_diagnostic;
use dive::dive;
use hydrothermal_venture::hydrothermal_venture;
use lanternfish::lanternfish;
use seven_segment::seven_segment;
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
        2 => dive(opts.input),
        3 => binary_diagnostic(opts.input),
        5 => hydrothermal_venture(opts.input),
        6 => lanternfish(opts.input),
        7 => whale(opts.input),
        8 => seven_segment(opts.input),
        _ => unimplemented!(),
    }
}
