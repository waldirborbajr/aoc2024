use std::{collections::HashSet, ops::Rem};

use num::traits::Euclid;

#[test]
fn test() {
    solve(String::from(
        "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
    ));
}

struct Garden {
    grid: Vec<Vec<char>>,
    reachable: HashSet<(i64, i64)>,
    height: usize,
    width: usize,
}

impl Garden {
    fn new(data: &str) -> Self {
        let mut rows = Vec::new();
        let mut reachable = HashSet::new();
        for (ridx, line) in data.lines().enumerate() {
            let mut row = Vec::new();
            for (cidx, char) in line.chars().enumerate() {
                row.push(char);
                if char == 'S' {
                    reachable.insert((ridx as i64, cidx as i64));
                }
            }
            rows.push(row);
        }
        Garden {
            width: rows.first().unwrap().len(),
            height: rows.len(),
            reachable,
            grid: rows,
        }
    }

    fn get_pu_char(&self, row: i64, col: i64) -> char {
        let ridx = row.rem_euclid(self.height as i64);
        let cidx = col.rem_euclid(self.width as i64);
        self.grid[ridx as usize][cidx as usize]
    }

    fn walk(&mut self, steps: usize) -> u64 {
        if steps == 0 {
            return self.reachable.len() as u64;
        }

        let mut new_tiles = HashSet::new();

        for (row, col) in &self.reachable {
            if self.get_pu_char(row - 1, *col) == '.' {
                new_tiles.insert((row - 1, *col));
            }
            if self.get_pu_char(row + 1, *col) == '.' {
                new_tiles.insert((row + 1, *col));
            }
            if self.get_pu_char(*row, col - 1) == '.' {
                new_tiles.insert((*row, col - 1));
            }
            if self.get_pu_char(*row, col + 1) == '.' {
                new_tiles.insert((*row, col + 1));
            }
        }

        self.reachable = new_tiles;

        self.walk(steps - 1)
    }
}

pub fn solve(data: String) {
    let mut garden = Garden::new(data.as_str());
    println!("{}", garden.walk(5000));
}
