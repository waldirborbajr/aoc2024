use std::collections::HashMap;

#[test]
fn test() {
    solve(String::from(
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
    ));
}

// enum Move {
//     U(usize),
//     D(usize),
//     L(usize),
//     R(usize),
// }

struct DigPlot {
    grid: Vec<Vec<char>>,
    dug: HashMap<(usize, usize), bool>,
    width: usize,
    height: usize,
}

impl DigPlot {
    fn new(data: String) -> Self {
        let moves_sum: usize = data
            .lines()
            .map(|s| {
                s.split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            })
            .sum();

        let (mut width, mut height) = (moves_sum + 2, moves_sum + 2);
        let (mut row, mut col) = (height / 2, width / 2);

        let mut grid = Vec::new();
        for _ in 0..height {
            let mut rv = Vec::new();
            for _ in 0..width {
                rv.push('.');
            }
            grid.push(rv);
        }

        for line in data.lines() {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            match parts[..] {
                ["U", n, _c] => {
                    let n = n.parse().unwrap();
                    // dig_plan.push(Dig {
                    //     color: c.to_owned(),
                    //     dig_move: Move::U(n),
                    // });
                    for _ in 0..n {
                        grid[row][col] = '#';
                        row -= 1;
                    }
                    // row -= n as i32;
                }
                ["D", n, _c] => {
                    let n = n.parse().unwrap();
                    // dig_plan.push(Dig {
                    //     color: c.to_owned(),
                    //     dig_move: Move::D(n),
                    // });
                    for _ in 0..n {
                        grid[row][col] = '#';
                        row += 1;
                    }
                    // row += n as i32;
                }
                ["L", n, _c] => {
                    let n = n.parse().unwrap();
                    // dig_plan.push(Dig {
                    //     color: c.to_owned(),
                    //     dig_move: Move::L(n),
                    // });
                    for _ in 0..n {
                        grid[row][col] = '#';
                        col -= 1;
                    }
                    // col -= n as i32;
                }
                ["R", n, _c] => {
                    let n = n.parse().unwrap();
                    // dig_plan.push(Dig {
                    //     color: c.to_owned(),
                    //     dig_move: Move::R(n),
                    // });
                    for _ in 0..n {
                        grid[row][col] = '#';
                        col += 1;
                    }
                    // col += n as i32;
                }
                _ => panic!("error parsing dig line"),
            }
            if row > height {
                height = row;
            }
            if col > width {
                width = col;
            }
        }
        // if row < 0 {
        //     height += -row;
        // }
        // if col < 0 {
        //     width += -col;
        // }
        DigPlot {
            width,
            height,
            grid,
            dug: HashMap::new(),
        }
    }

    fn count_dug(&self) -> usize {
        let mut sum = 0;
        for line in &self.grid {
            for c in line {
                if *c == '#' {
                    sum += 1;
                }
            }
        }
        sum
    }

    // fn count_dug(&self) -> (usize, String) {
    //     let mut sum = 0;
    //     let mut grid = String::new();
    //     for line in &self.grid {
    //         let mut inside = false;
    //         let mut on_wall = false;
    //         for c in line {
    //             if on_wall && *c == '.' {
    //                 inside = !inside;
    //             }
    //             on_wall = *c == '#';
    //             if on_wall || inside {
    //                 sum += 1;
    //                 grid.push('#');
    //             } else {
    //                 grid.push('.');
    //             }
    //         }
    //         grid.push('\n');
    //     }
    //     (sum, grid)
    // }

    fn fill_space(&mut self) {
        self.dug = HashMap::new();
        self.fill_cell((self.height / 2 + 1, self.width / 2 + 1));
    }

    fn fill_cell(&mut self, (row, col): (usize, usize)) {
        if self.grid[row][col] == '#' || self.dug.contains_key(&(row, col)) {
            return;
        }

        // println!("filling cell {}, {}", row, col);
        self.grid[row][col] = '#';
        self.dug.insert((row, col), true);

        if row > 0 {
            self.fill_cell((row - 1, col));
        }
        if row < self.height - 1 {
            self.fill_cell((row + 1, col));
        }
        if col > 0 {
            self.fill_cell((row, col - 1));
        }
        if col < self.width - 1 {
            self.fill_cell((row, col + 1));
        }
    }

    fn to_string(&self) -> String {
        let mut grid = String::new();
        for line in &self.grid {
            let s: String = line.iter().collect();
            grid.push_str(&s);
            grid.push('\n');
        }
        grid
    }
}

pub fn solve(data: String) {
    let mut plot = DigPlot::new(data);
    plot.fill_space();
    println!("{}\ndug: {}", plot.to_string(), plot.count_dug());
    // let (dug, dug_grid) = plot.count_dug();
    // println!("{}\ndug: {}", dug_grid, dug);
}
