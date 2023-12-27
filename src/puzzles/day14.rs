use std::collections::HashMap;

#[test]
fn test() {
    solve(String::from(
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
    ));
}

enum Direction {
    North,
    East,
    South,
    West,
}

struct RockGrid {
    walls: Vec<(usize, usize)>,
    width: usize,
    height: usize,
    rocks: Vec<(usize, usize)>,
    cycle_states: HashMap<String, usize>,
    cycle_loads: HashMap<usize, usize>,
}

impl RockGrid {
    fn new(data: String) -> Self {
        let height = data.lines().count();
        let width = data.lines().next().unwrap().len();
        let mut walls = Vec::new();
        let mut rocks = Vec::new();
        for (row, line) in data.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                match char {
                    '#' => walls.push((row, col)),
                    'O' => rocks.push((row, col)),
                    _ => continue,
                }
            }
        }
        RockGrid {
            walls,
            width,
            height,
            rocks,
            cycle_states: HashMap::new(),
            cycle_loads: HashMap::new(),
        }
    }

    fn sort_rocks(&mut self, dir: Direction) {
        match dir {
            Direction::North => self
                .rocks
                .sort_by(|(row, _col), (row2, _col2)| row2.partial_cmp(row).unwrap()),
            Direction::East => self
                .rocks
                .sort_by(|(_row, col), (_row2, col2)| col.partial_cmp(col2).unwrap()),
            Direction::South => self
                .rocks
                .sort_by(|(row, _col), (row2, _col2)| row.partial_cmp(row2).unwrap()),
            Direction::West => self
                .rocks
                .sort_by(|(_row, col), (_row2, col2)| col2.partial_cmp(col).unwrap()),
        }
    }

    fn shift(&mut self, dir: Direction) {
        let mut new_rocks = Vec::new();
        let bound = match dir {
            Direction::North => -1,
            Direction::East => self.width as i32,
            Direction::South => self.height as i32,
            Direction::West => -1,
        };
        let step = match dir {
            Direction::North => -1,
            Direction::East => 1,
            Direction::South => 1,
            Direction::West => -1,
        };
        while let Some((row, col)) = self.rocks.pop() {
            let mut wall_coord = match dir {
                Direction::North => row,
                Direction::East => col,
                Direction::South => row,
                Direction::West => col,
            } as i32;
            let final_coord = loop {
                wall_coord += step;
                let new_coord = match dir {
                    Direction::North => (wall_coord as usize, col),
                    Direction::East => (row, wall_coord as usize),
                    Direction::South => (wall_coord as usize, col),
                    Direction::West => (row, wall_coord as usize),
                };
                if wall_coord == bound
                    || self.walls.contains(&new_coord)
                    || new_rocks.contains(&new_coord)
                {
                    break (wall_coord - step) as usize;
                }
            };
            let new_rock = match dir {
                Direction::North => (final_coord, col),
                Direction::East => (row, final_coord),
                Direction::South => (final_coord, col),
                Direction::West => (row, final_coord),
            };

            // println!("rock at {}, {} moved to row {}", row, col, new_rock.0);
            new_rocks.push(new_rock);
        }
        self.rocks = new_rocks;
    }

    fn calculate_load(&self) -> usize {
        let mut sum = 0;
        for (row, _) in &self.rocks {
            sum += self.height - row;
        }
        // dbg!(&self.rocks);
        sum
    }

    fn to_string(&self) -> String {
        let mut grid = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                grid.push(
                    match (
                        self.rocks.contains(&(row, col)),
                        self.walls.contains(&(row, col)),
                    ) {
                        (true, true) => 'X',
                        (true, false) => 'O',
                        (false, true) => '#',
                        (false, false) => '.',
                    },
                );
            }
            grid.push('\n');
        }
        grid
    }

    fn _print_grid(&self) {
        println!("grid:\n{}\n", self.to_string());
    }

    fn cycle(&mut self) {
        self.sort_rocks(Direction::North);
        self.shift(Direction::North);
        self.sort_rocks(Direction::West);
        self.shift(Direction::West);
        self.sort_rocks(Direction::South);
        self.shift(Direction::South);
        self.sort_rocks(Direction::East);
        self.shift(Direction::East);
    }

    fn check_for_cycle(&mut self, iter: usize) {
        let grid = self.to_string();
        if let Some(cycle_start) = self.cycle_states.get(&grid) {
            let period = iter - cycle_start;
            let remainder = (1000000000 - cycle_start) % period;
            panic!(
                "cycle found at {}..{} (final load: {})",
                cycle_start,
                iter,
                self.cycle_loads.get(&(remainder + cycle_start)).unwrap()
            );
        } else {
            self.cycle_states.insert(grid, iter);
            self.cycle_loads.insert(iter, self.calculate_load());
        }
    }
}

pub fn solve(data: String) {
    // println!("{}", data);
    let mut grid = RockGrid::new(data);
    // grid.print_grid();
    // grid.sort_rocks(Direction::North);
    // grid.shift(Direction::North);
    // grid.print_grid();
    for i in 0..1000000000 {
        println!("iteration {}", i);
        grid.check_for_cycle(i);
        grid.cycle();
        // if i % 10000 == 0 {
        // }
        // grid.shift(Direction::North);
    }

    dbg!(grid.calculate_load());
}
