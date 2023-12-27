use std::collections::HashMap;

#[test]
fn test() {
    solve(String::from(
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
    ));
}

#[derive(Eq, Copy, Clone, PartialEq, Debug, Hash)]
enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Coords {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct CrucibleMap {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct AStarScores {
    parent: (Coords, Direction),
    dir: Direction,
    g: usize,
    h: usize,
}

impl CrucibleMap {
    fn new(data: String) -> Self {
        let mut grid = Vec::new();
        for line in data.lines() {
            grid.push(Vec::new());
            for char in line.chars() {
                grid.last_mut().unwrap().push(char);
            }
        }
        CrucibleMap {
            width: grid.first().unwrap().len(),
            height: grid.len(),
            grid,
        }
    }

    fn draw_path(&mut self, path: &Vec<(Coords, Direction)>) {
        for (coords, dir) in path {
            self.grid[coords.row][coords.col] = match dir {
                Direction::Up(_) => '^',
                Direction::Down(_) => 'v',
                Direction::Left(_) => '<',
                Direction::Right(_) => '>',
            };
        }
        for i in 0..self.height {
            for j in 0..self.width {
                // if let Some(el) = path.get(&Coords { row: i, col: j }) {
                //     print!(
                //         "{}",
                //     );
                print!("{}", self.grid[i][j]);
            }
            print!("\n");
        }
    }

    fn get_cell_value(&self, coords: &Coords) -> usize {
        self.grid[coords.row][coords.col].to_digit(10).unwrap() as usize
    }

    fn find_lowest_f(
        &self,
        list: &HashMap<(Coords, Direction), AStarScores>,
    ) -> ((Coords, Direction), AStarScores) {
        let mut kvs = list.iter().collect::<Vec<_>>();
        kvs.sort_by(|(_k1, v1), (_k2, v2)| (v1.g + v1.h).partial_cmp(&(v2.g + v2.h)).unwrap());
        let curr = kvs.first().unwrap();
        (*(*curr).0, *(*curr).1)
    }

    fn find_path(&self) -> Vec<(Coords, Direction)> {
        let mut open = HashMap::new();
        let mut closed = HashMap::new();
        open.insert(
            (Coords { row: 0, col: 0 }, Direction::Right(0)),
            AStarScores {
                g: 0,
                h: self.heuristic(Coords { row: 0, col: 0 }),
                dir: Direction::Right(0),
                parent: (Coords { row: 0, col: 0 }, Direction::Right(0)),
            },
        );
        let (mut curr, path_len) = loop {
            println!("open: {}, closed: {}", open.len(), closed.len());
            if open.len() == 0 {
                panic!("unable to find path");
            }
            // let mut curr = open.clone().iter().next().unwrap().clone();
            // let mut min = curr.1.g + curr.1.h;
            // for sq in &open {
            //     if sq.1.g + sq.1.h < min {
            //         min = sq.1.g + sq.1.h;
            //         curr = sq.clone();
            //     }
            // }
            let curr = self.find_lowest_f(&open);
            // dbg!(curr);
            // let mut kvs = open.iter().collect::<Vec<_>>();
            // kvs.sort_by(|(k1, v1), (k2, v2)| (v1.g + v1.h).partial_cmp(&(v2.g + v2.h)).unwrap());
            // let curr = kvs.first().unwrap();
            open.remove(&curr.0);
            closed.insert(curr.0, curr.1);
            if curr.0 .0.row == self.height - 1 && curr.0 .0.col == self.width - 1 {
                break (curr.0, curr.1.g);
            }
            let valid_next = self.find_valid_next_2(curr.0 .0, curr.1.dir);
            // dbg!(&valid_next);
            for (next, dir) in valid_next {
                if closed.contains_key(&(next, dir)) {
                    continue;
                } else if let Some(scores) = open.get_mut(&(next, dir)) {
                    if curr.1.g + self.get_cell_value(&next) < scores.g {
                        scores.g = curr.1.g + self.get_cell_value(&next);
                        scores.parent = curr.0;
                        scores.dir = dir;
                    }
                } else {
                    open.insert(
                        (next, dir),
                        AStarScores {
                            parent: curr.0,
                            dir,
                            g: curr.1.g + self.get_cell_value(&next),
                            h: self.heuristic(next),
                        },
                    );
                }
            }
        };
        let mut path = vec![curr];
        while curr.0 != (Coords { row: 0, col: 0 }) {
            curr = closed.get(&curr).unwrap().parent;
            path.push(curr);
        }
        dbg!(path_len);
        path
    }

    fn find_valid_next_2(&self, curr: Coords, dir: Direction) -> Vec<(Coords, Direction)> {
        let mut coords = Vec::new();
        let consec_moves = match dir {
            Direction::Up(n) => n,
            Direction::Down(n) => n,
            Direction::Left(n) => n,
            Direction::Right(n) => n,
        };
        let can_turn = consec_moves == 0 || consec_moves >= 4;
        let must_turn = consec_moves >= 10;
        // dbg!(curr);
        // dbg!(dir);
        if curr.row > 0
            && !matches!(dir, Direction::Down(_))
            && ((!matches!(dir, Direction::Up(_)) && can_turn)
                || (matches!(dir, Direction::Up(_)) && !must_turn))
        {
            coords.push((
                Coords {
                    row: curr.row - 1,
                    col: curr.col,
                },
                if let Direction::Up(n) = dir {
                    Direction::Up(n + 1)
                } else {
                    Direction::Up(1)
                },
            ))
        }
        if curr.row < self.height - 1
            && !matches!(dir, Direction::Up(_))
            && ((!matches!(dir, Direction::Down(_)) && can_turn)
                || (matches!(dir, Direction::Down(_)) && !must_turn))
        {
            coords.push((
                Coords {
                    row: curr.row + 1,
                    col: curr.col,
                },
                if let Direction::Down(n) = dir {
                    Direction::Down(n + 1)
                } else {
                    Direction::Down(1)
                },
            ))
        }
        if curr.col > 0
            && !matches!(dir, Direction::Right(_))
            && ((!matches!(dir, Direction::Left(_)) && can_turn)
                || (matches!(dir, Direction::Left(_)) && !must_turn))
        {
            coords.push((
                Coords {
                    row: curr.row,
                    col: curr.col - 1,
                },
                if let Direction::Left(n) = dir {
                    Direction::Left(n + 1)
                } else {
                    Direction::Left(1)
                },
            ))
        }
        if curr.col < self.width - 1
            && !matches!(dir, Direction::Left(_))
            && ((!matches!(dir, Direction::Right(_)) && can_turn)
                || (matches!(dir, Direction::Right(_)) && !must_turn))
        {
            coords.push((
                Coords {
                    row: curr.row,
                    col: curr.col + 1,
                },
                if let Direction::Right(n) = dir {
                    Direction::Right(n + 1)
                } else {
                    Direction::Right(1)
                },
            ))
        }
        coords
        // dbg!(coords)
    }

    fn _find_valid_next(&self, curr: Coords, dir: Direction) -> Vec<(Coords, Direction)> {
        let mut coords = Vec::new();
        if curr.row > 0 && dir != Direction::Up(3) && !matches!(dir, Direction::Down(_)) {
            coords.push((
                Coords {
                    row: curr.row - 1,
                    col: curr.col,
                },
                if let Direction::Up(n) = dir {
                    Direction::Up(n + 1)
                } else {
                    Direction::Up(1)
                },
            ))
        }
        if curr.row < self.height - 1
            && dir != Direction::Down(3)
            && !matches!(dir, Direction::Up(_))
        {
            coords.push((
                Coords {
                    row: curr.row + 1,
                    col: curr.col,
                },
                if let Direction::Down(n) = dir {
                    Direction::Down(n + 1)
                } else {
                    Direction::Down(1)
                },
            ))
        }
        if curr.col > 0 && dir != Direction::Left(3) && !matches!(dir, Direction::Right(_)) {
            coords.push((
                Coords {
                    row: curr.row,
                    col: curr.col - 1,
                },
                if let Direction::Left(n) = dir {
                    Direction::Left(n + 1)
                } else {
                    Direction::Left(1)
                },
            ))
        }
        if curr.col < self.width - 1
            && dir != Direction::Right(3)
            && !matches!(dir, Direction::Left(_))
        {
            coords.push((
                Coords {
                    row: curr.row,
                    col: curr.col + 1,
                },
                if let Direction::Right(n) = dir {
                    Direction::Right(n + 1)
                } else {
                    Direction::Right(1)
                },
            ))
        }
        coords
    }

    fn heuristic(&self, coords: Coords) -> usize {
        (self.height - coords.row) + (self.width - coords.col) - 2
    }
}
pub fn solve(data: String) {
    let mut grid = CrucibleMap::new(data);
    let path = grid.find_path();
    dbg!(grid.draw_path(&path));
}
