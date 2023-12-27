use std::collections::HashSet;

#[test]
fn test() {
    solve(String::from(
        r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
    ));
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coords {
    row: usize,
    col: usize,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_string(&self) -> &str {
        match self {
            Direction::Up => "U",
            Direction::Down => "D",
            Direction::Left => "L",
            Direction::Right => "R",
        }
    }
    fn bounce(&self, c: char) -> Vec<Self> {
        use Direction::*;
        match (self, c) {
            (_, '.') => vec![self.clone()],
            (Up, '|') => vec![Up],
            (Down, '|') => vec![Down],
            (Left, '|') => vec![Up, Down],
            (Right, '|') => vec![Up, Down],
            (Up, '-') => vec![Left, Right],
            (Down, '-') => vec![Left, Right],
            (Left, '-') => vec![Left],
            (Right, '-') => vec![Right],
            (Up, '/') => vec![Right],
            (Down, '/') => vec![Left],
            (Left, '/') => vec![Down],
            (Right, '/') => vec![Up],
            (Up, '\\') => vec![Left],
            (Down, '\\') => vec![Right],
            (Left, '\\') => vec![Up],
            (Right, '\\') => vec![Down],
            _ => panic!("unknown character in grid"),
        }
    }

    fn next(&self, loc: Coords) -> Coords {
        match self {
            Direction::Up => Coords {
                row: if loc.row > 0 { loc.row - 1 } else { loc.row },
                col: loc.col,
            },
            Direction::Down => Coords {
                row: loc.row + 1,
                col: loc.col,
            },
            Direction::Left => Coords {
                row: loc.row,
                col: if loc.col > 0 { loc.col - 1 } else { loc.col },
            },
            Direction::Right => Coords {
                row: loc.row,
                col: loc.col + 1,
            },
        }
    }
}

#[derive(Debug)]
struct MirrorGrid {
    grid: Vec<Vec<char>>,
    glowing: HashSet<Coords>,
    memo: HashSet<String>,
    width: usize,
    height: usize,
}

impl MirrorGrid {
    fn new(data: &str) -> Self {
        let mut lines = Vec::new();
        for line in data.lines() {
            lines.push(Vec::new());
            for char in line.chars() {
                lines.last_mut().unwrap().push(char);
            }
        }
        MirrorGrid {
            grid: lines.clone(),
            glowing: HashSet::new(),
            memo: HashSet::new(),
            width: lines.first().unwrap().len(),
            height: lines.len(),
        }
    }

    fn get_char(&self, loc: Coords) -> char {
        self.grid[loc.row][loc.col]
    }

    fn in_bounds(&self, loc: Coords) -> bool {
        loc.row < self.height && loc.col < self.width
    }

    fn propagate_ray(&mut self, dir: Direction, loc: Coords) {
        let h = format!("{}{},{}", dir.to_string(), loc.row, loc.col);

        if !self.memo.contains(&h) && self.in_bounds(loc) {
            self.glowing.insert(loc);
            println!(
                "propagating {} {},{} ({})",
                dir.to_string(),
                loc.row,
                loc.col,
                self.get_char(loc)
            );
            self.memo.insert(h);
            let new_dirs = dir.bounce(self.get_char(loc));
            for new_dir in new_dirs {
                println!("bouncing {}", new_dir.to_string());
                self.propagate_ray(new_dir, new_dir.next(loc));
            }
        }
    }

    fn count_glowing(&mut self, dir: Direction, loc: Coords) -> usize {
        self.glowing.clear();
        self.memo.clear();
        self.propagate_ray(dir, loc);
        self.glowing.len()
    }
}

pub fn solve(data: String) {
    println!("{}", data);
    let mut grid = MirrorGrid::new(&data);
    let glowing = grid.count_glowing(Direction::Right, Coords { col: 0, row: 0 });
    let mut max = 0;
    for i in 0..grid.width {
        max = std::cmp::max(
            max,
            grid.count_glowing(Direction::Down, Coords { col: i, row: 0 }),
        );
        max = std::cmp::max(
            max,
            grid.count_glowing(
                Direction::Up,
                Coords {
                    col: i,
                    row: grid.height - 1,
                },
            ),
        );
    }
    for i in 0..grid.height {
        max = std::cmp::max(
            max,
            grid.count_glowing(Direction::Right, Coords { col: 0, row: i }),
        );
        max = std::cmp::max(
            max,
            grid.count_glowing(
                Direction::Left,
                Coords {
                    col: grid.width - 1,
                    row: i,
                },
            ),
        );
    }
    println!("glowing cells: {}, max: {}", glowing, max);
}
