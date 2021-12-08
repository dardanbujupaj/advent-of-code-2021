use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn at(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    pub fn parse(input: &str) -> Point {
        let values: Vec<isize> = input.split(',').map(|p| p.parse().unwrap()).collect();

        Point {
            x: values[0],
            y: values[1],
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Point::new(), Point { x: 0, y: 0 });
    }

    #[test]
    fn test_at() {
        assert_eq!(Point::at(1, 2), Point { x: 1, y: 2 });
    }

    #[test]
    fn test_parse() {
        assert_eq!(Point::parse("1,2"), Point { x: 1, y: 2 });
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Point { x: 1, y: 2 } + Point { x: 2, y: 1 },
            Point { x: 3, y: 3 }
        );
        assert_ne!(
            Point { x: 1, y: 2 } + Point { x: 2, y: 1 },
            Point { x: 1, y: 2 }
        );
    }
}
