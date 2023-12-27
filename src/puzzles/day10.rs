use std::collections::HashSet;

#[test]
fn test() {
    solve(String::from(
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
    ));
}

struct PipeMap {
    width: usize,
    height: usize,
    tiles: Vec<Vec<char>>,
    start: Coords,
    path_tiles: HashSet<Coords>,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl PipeMap {
    fn print(&self) {
        for line in &self.tiles {
            for tile in line {
                print!("{}", tile);
            }
            print!("\n");
        }
    }

    fn walk_path(&mut self) -> usize {
        let mut length = 1;
        self.path_tiles.insert(self.start);
        let (mut next, mut dir) = if self.start.x > 0
            && get_valid_tiles(&Direction::Right)
                .contains(self.tiles[self.start.y][self.start.x - 1])
        {
            (
                Coords {
                    x: self.start.x - 1,
                    y: self.start.y,
                },
                Direction::Right,
            )
        } else if self.start.x < self.width - 1
            && get_valid_tiles(&Direction::Left)
                .contains(self.tiles[self.start.y][self.start.x + 1])
        {
            (
                Coords {
                    x: self.start.x + 1,
                    y: self.start.y,
                },
                Direction::Left,
            )
        } else if self.start.y > 0
            && get_valid_tiles(&Direction::Down)
                .contains(self.tiles[self.start.y - 1][self.start.x])
        {
            (
                Coords {
                    x: self.start.x,
                    y: self.start.y - 1,
                },
                Direction::Down,
            )
        } else if self.start.y < self.height - 1
            && get_valid_tiles(&Direction::Up).contains(self.tiles[self.start.y + 1][self.start.x])
        {
            (
                Coords {
                    x: self.start.x,
                    y: self.start.y + 1,
                },
                Direction::Up,
            )
        } else {
            panic!("error finding first tile");
        };
        while next != self.start {
            self.path_tiles.insert(next);
            (next, dir) = self.find_next(&dir, &next);
            length += 1;
        }
        length
    }

    fn count_enclosing(&self) -> usize {
        let mut inside = false;
        let mut horizontal = None;
        let mut count = 0;
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if self.path_tiles.contains(&Coords { x, y }) {
                    match (tile, &horizontal) {
                        ('|', _) => inside = !inside,
                        ('F', _) => horizontal = Some(Direction::Down),
                        ('L', _) => horizontal = Some(Direction::Up),
                        ('7', Some(Direction::Up)) => {
                            horizontal = None;
                            inside = !inside;
                        }
                        ('7', Some(Direction::Down)) => horizontal = None,
                        ('J', Some(Direction::Down)) => {
                            horizontal = None;
                            inside = !inside;
                        }
                        ('J', Some(Direction::Up)) => horizontal = None,
                        (_, _) => continue,
                    }
                } else if inside {
                    count += 1;
                }
            }
        }
        count
    }

    fn find_next(&self, from_dir: &Direction, cur_tile: &Coords) -> (Coords, Direction) {
        let tile_value = self.tiles[cur_tile.y][cur_tile.x];
        match (from_dir, tile_value) {
            (Direction::Left, '7') => (
                Coords {
                    x: cur_tile.x,
                    y: cur_tile.y + 1,
                },
                Direction::Up,
            ),
            (Direction::Left, 'J') => (
                Coords {
                    x: cur_tile.x,
                    y: cur_tile.y - 1,
                },
                Direction::Down,
            ),
            (Direction::Left, '-') => (
                Coords {
                    x: cur_tile.x + 1,
                    y: cur_tile.y,
                },
                Direction::Left,
            ),
            (Direction::Right, 'L') => (
                Coords {
                    x: cur_tile.x,
                    y: cur_tile.y - 1,
                },
                Direction::Down,
            ),
            (Direction::Right, 'F') => (
                Coords {
                    x: cur_tile.x,
                    y: cur_tile.y + 1,
                },
                Direction::Up,
            ),
            (Direction::Right, '-') => (
                Coords {
                    x: cur_tile.x - 1,
                    y: cur_tile.y,
                },
                Direction::Right,
            ),
            (Direction::Up, 'L') => (
                Coords {
                    x: cur_tile.x + 1,
                    y: cur_tile.y,
                },
                Direction::Left,
            ),
            (Direction::Up, 'J') => (
                Coords {
                    x: cur_tile.x - 1,
                    y: cur_tile.y,
                },
                Direction::Right,
            ),
            (Direction::Up, '|') => (
                Coords {
                    x: cur_tile.x,
                    y: cur_tile.y + 1,
                },
                Direction::Up,
            ),
            (Direction::Down, '7') => (
                Coords {
                    x: cur_tile.x - 1,
                    y: cur_tile.y,
                },
                Direction::Right,
            ),
            (Direction::Down, 'F') => (
                Coords {
                    x: cur_tile.x + 1,
                    y: cur_tile.y,
                },
                Direction::Left,
            ),
            (Direction::Down, '|') => (
                Coords {
                    x: cur_tile.x,
                    y: cur_tile.y - 1,
                },
                Direction::Down,
            ),
            _ => panic!("invalid tile"),
        }
    }
}

pub fn solve(data: String) {
    let lines = data.split("\n").collect::<Vec<_>>();
    let mut rows = Vec::new();
    let mut start = Coords { x: 0, y: 0 };

    for (idx, line) in lines.iter().enumerate() {
        let chars = line.chars().collect::<Vec<_>>();
        if let Some(n) = chars.iter().position(|c| c == &'S') {
            start = Coords { x: n, y: idx };
        }
        rows.push(chars);
    }

    let mut map = PipeMap {
        start,
        width: lines[0].len(),
        height: lines.len(),
        tiles: rows,
        path_tiles: HashSet::new(),
    };
    map.print();
    println!("path length: {}", map.walk_path());
    println!("enclosed tiles: {}", map.count_enclosing());
}
fn get_valid_tiles(from_dir: &Direction) -> &str {
    match from_dir {
        Direction::Left => "7J-",
        Direction::Right => "FL-",
        Direction::Up => "JL|",
        Direction::Down => "F7|",
    }
}
