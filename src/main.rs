use clap::Parser;

use advent_of_code_2021::puzzles::{
    binary_diagnostic, dive, hydrothermal_venture, lanternfish, seven_segment, sonar_sweep, whale,
};

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
