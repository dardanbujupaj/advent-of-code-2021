use std::{env, fs};

use advent_of_code_2021::puzzles::{
    binary_diagnostic, dive, dumbo_octopus, hydrothermal_venture, lanternfish, seven_segment,
    smoke_basin, sonar_sweep, syntax_scoring, whale,
};

// this tests check if all solutions run through (without panic) with the example and actual input

fn read_input_file(filename: &str) -> String {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Cargo dir not available");
    let filepath = format!("{}/tests/inputs/{}", manifest_dir, filename);

    fs::read_to_string(filepath).expect("Something went wrong reading the file")
}

#[test]
fn test_dive_example_input() {
    let input = read_input_file("dive_example.txt");
    dive(input);
}

#[test]
fn test_dive_input() {
    let input = read_input_file("dive.txt");
    dive(input);
}

#[test]
fn test_sonar_sweep_example_input() {
    let input = read_input_file("sonar_sweep_example.txt");
    sonar_sweep(input);
}

#[test]
fn test_sonar_sweep_input() {
    let input = read_input_file("sonar_sweep.txt");
    sonar_sweep(input);
}

#[test]
fn test_binary_diagnostic_example_input() {
    let input = read_input_file("binary_diagnostic_example.txt");
    binary_diagnostic(input);
}

#[test]
fn test_binary_diagnostic_input() {
    let input = read_input_file("binary_diagnostic.txt");
    binary_diagnostic(input);
}

#[test]
fn test_lanternfish_input() {
    let input = read_input_file("lanternfish.txt");
    lanternfish(input);
}

#[test]
fn test_lanternfish_example_input() {
    let input = read_input_file("lanternfish_example.txt");
    lanternfish(input);
}

#[test]
fn test_whale_example_input() {
    let input = read_input_file("whale_example.txt");
    whale(input);
}

#[test]
fn test_whale_input() {
    let input = read_input_file("whale.txt");
    whale(input);
}

#[test]
fn test_hydrothermal_venture_example_input() {
    let input = read_input_file("hydrothermal_venture_example.txt");
    hydrothermal_venture(input);
}

#[test]
fn test_hydrothermal_venture_input() {
    let input = read_input_file("hydrothermal_venture.txt");
    hydrothermal_venture(input);
}

#[test]
fn test_seven_segment_example_input() {
    let input = read_input_file("seven_segment_example.txt");
    seven_segment(input);
}

#[test]
fn test_seven_segment_input() {
    let input = read_input_file("seven_segment.txt");
    seven_segment(input);
}

#[test]
fn test_smoke_basin_example_input() {
    let input = read_input_file("smoke_basin_example.txt");
    smoke_basin(input);
}

#[test]
fn test_smoke_basin_input() {
    let input = read_input_file("smoke_basin.txt");
    smoke_basin(input);
}

#[test]
fn test_syntax_scoring_example_input() {
    let input = read_input_file("syntax_scoring_example.txt");
    syntax_scoring(input);
}

#[test]
fn test_syntax_scoring_input() {
    let input = read_input_file("syntax_scoring.txt");
    syntax_scoring(input);
}

#[test]
fn test_dumbo_octopus_example_input() {
    let input = read_input_file("dumbo_octopus_example.txt");
    dumbo_octopus(input);
}

#[test]
fn test_dumbo_octopus_input() {
    let input = read_input_file("dumbo_octopus.txt");
    dumbo_octopus(input);
}
