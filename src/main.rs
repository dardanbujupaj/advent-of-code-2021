use clap::Parser;

use aoc::puzzles::{
    binary_diagnostic, chiton, dive, dumbo_octopus, extended_polymerization, hydrothermal_venture,
    lanternfish, packet_decoder, seven_segment, smoke_basin, sonar_sweep, syntax_scoring, whale,
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
        9 => smoke_basin(opts.input),
        10 => syntax_scoring(opts.input),
        11 => dumbo_octopus(opts.input),
        14 => extended_polymerization(opts.input),
        15 => chiton(opts.input),
        16 => packet_decoder(opts.input),
        _ => unimplemented!(),
    }
}
