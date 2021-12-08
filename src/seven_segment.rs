pub fn seven_segment(input: String) {
    println!("Counting segments");
    println!("Count 1,4,7,8 (Part 1): {}", part_1(input));
}

/// count 1, 4, 7, 8
fn part_1(input: String) -> usize {
    let count: usize = input
        .lines()
        .map(Entry::parse)
        .map(|e| {
            e.output
                .iter()
                .filter(|x| vec![1, 4, 7, 8].contains(x))
                .count()
        })
        .sum();

    count
}

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    input: Vec<u8>,
    output: Vec<u8>,
}

impl Entry {
    pub fn parse(input: &str) -> Self {
        let input_output: Vec<&str> = input.split(" | ").collect();
        let input = parse_sequence(input_output[0]);
        let output = parse_sequence(input_output[1]);

        Entry { input, output }
    }
}

fn parse_sequence(input: &str) -> Vec<u8> {
    input
        .split_whitespace()
        .map(|d| match d.chars().count() {
            2 => 1,
            4 => 4,
            3 => 7,
            7 => 8,
            _ => u8::MAX,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sequence() {
        let sequence = "gecf egdcabf bgf bfgea";

        let expected = vec![4, 8, 7, u8::MAX];

        assert_eq!(parse_sequence(sequence), expected);
    }

    #[test]
    fn test_parse_entry() {
        let entry_sequence =
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea";
        let expected_entry = Entry {
            input: vec![
                8,
                7,
                1,
                u8::MAX,
                u8::MAX,
                4,
                u8::MAX,
                u8::MAX,
                u8::MAX,
                u8::MAX,
            ],
            output: vec![4, 8, 7, u8::MAX],
        };

        assert_eq!(Entry::parse(entry_sequence), expected_entry);
    }

    #[test]
    fn test_part_1() {
        let entry_sequence =
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"
                .to_string();

        assert_eq!(part_1(entry_sequence), 3);
    }
}
