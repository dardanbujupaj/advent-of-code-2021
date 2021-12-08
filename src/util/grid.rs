use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Grid<S> {
    data: Vec<S>,
    width: isize,
    height: isize,
}

impl<S> Grid<S>
where
    S: Copy,
    S: Display,
{
    pub fn new(width: isize, height: isize, default: S) -> Self {
        Grid {
            data: vec![default; (width * height).try_into().unwrap()],
            width,
            height,
        }
    }

    pub fn data(&self) -> &Vec<S> {
        &self.data
    }

    pub fn get(&self, x: isize, y: isize) -> S {
        self.data[(y * self.width + x) as usize]
    }

    pub fn set(&mut self, x: isize, y: isize, value: S) {
        self.data[(y * self.width + x) as usize] = value;
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get(x, y));
            }
            println!();
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Grid::new(1, 5, 0u8),
            Grid {
                data: vec![0u8; 5],
                width: 1,
                height: 5
            }
        )
    }

    #[test]
    fn test_get() {
        let mut grid = Grid {
            data: vec![1, 2, 3, 4],
            width: 2,
            height: 2,
        };

        assert_eq!(grid.get(0, 0), 1);
        assert_eq!(grid.get(1, 0), 2);
        assert_eq!(grid.get(0, 1), 3);
        assert_eq!(grid.get(1, 1), 4);
    }

    #[test]
    fn test_set() {
        let mut grid = Grid {
            data: vec![0u8; 4],
            width: 2,
            height: 2,
        };

        grid.set(0, 0, 1);
        assert_eq!(
            grid,
            Grid {
                data: vec![1, 0, 0, 0],
                width: 2,
                height: 2
            }
        );
        grid.set(1, 0, 2);
        assert_eq!(
            grid,
            Grid {
                data: vec![1, 2, 0, 0],
                width: 2,
                height: 2
            }
        );
        grid.set(0, 1, 3);
        assert_eq!(
            grid,
            Grid {
                data: vec![1, 2, 3, 0],
                width: 2,
                height: 2
            }
        );
        grid.set(1, 1, 4);
        assert_eq!(
            grid,
            Grid {
                data: vec![1, 2, 3, 4],
                width: 2,
                height: 2
            }
        );
    }
}
