use std::fmt::Display;

const CYCLE_DAYS: usize = 7;
const NEW_DAYS: usize = CYCLE_DAYS + 2;

struct School {
    day: usize,
    ages: [usize; NEW_DAYS],
}

impl School {
    fn new() -> Self {
        School {
            day: 0,
            ages: [0; NEW_DAYS],
        }
    }

    fn parse(input: String) -> Self {
        let fishes = input.split(",");

        let mut ages = [0; NEW_DAYS];

        for age_str in fishes {
            let age = usize::from_str_radix(age_str, 10).unwrap();

            ages[age] += 1;
        }

        School { day: 0, ages }
    }

    pub fn iterate(&mut self) {
        self.day += 1;

        let mut new_ages = [0; NEW_DAYS];
        for i in 1..NEW_DAYS {
            new_ages[i - 1] = self.ages[i]
        }

        new_ages[CYCLE_DAYS - 1] += self.ages[0];
        new_ages[NEW_DAYS - 1] += self.ages[0];

        self.ages = new_ages;
    }

    pub fn count(&self) -> usize {
        let mut count = 0;

        for i in 0..NEW_DAYS {
            count += self.ages[i]
        }

        count
    }
}

impl Display for School {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "After {} days: {}", self.day, self.count())
    }
}

/// https://adventofcode.com/2021/day/6
pub fn lanternfish(input: String) {
    println!("Simulating lanternfish");
    println!("Input: {}", input);
    let mut school = School::parse(input);
    println!("{}", school);

    // PART 1
    for _ in 0..80 {
        school.iterate();
    }
    println!("{}", school);

    // PART 2
    for _ in 0..(256 - 80) {
        school.iterate();
    }
    println!("{}", school);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let s = School {
            day: 0,
            ages: [5; NEW_DAYS],
        };

        assert_eq!(s.count(), NEW_DAYS * 5)
    }

    #[test]
    fn test_iterate_days() {
        let mut s = School::new();
        assert_eq!(s.day, 0);

        s.iterate();

        assert_eq!(s.day, 1);
    }

    #[test]
    fn test_iterate_age() {
        let mut s = School::parse("1".to_string());
        assert_eq!(s.ages[0], 0);
        assert_eq!(s.ages[1], 1);

        s.iterate();
        assert_eq!(s.ages[0], 1);
        assert_eq!(s.ages[1], 0);

        s.iterate();
        assert_eq!(s.ages[0], 0);
        assert_eq!(s.ages[CYCLE_DAYS - 1], 1);
        assert_eq!(s.ages[NEW_DAYS - 1], 1);

        assert_eq!(s.count(), 2);
    }


    #[test]
    fn test_parse_input() {
        let input = "0,2,3,4,5".to_string();
        let s = School::parse(input);

        assert_eq!(s.count(), 5);
        assert_eq!(s.ages, [1, 0, 1, 1, 1, 1, 0, 0, 0]);
    }

    #[test]
    fn test_example() {
        let input = "3,4,3,1,2".to_string();

        let mut s = School::parse(input);

        for _ in 0..18 {
            s.iterate();
        }

        assert_eq!(s.count(), 26);

        for _ in 0..(80 - 18) {
            s.iterate();
        }

        assert_eq!(s.count(), );

    }
}
