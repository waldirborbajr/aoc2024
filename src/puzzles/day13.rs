#[test]
fn test() {
    solve(String::from(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
    ));
}

struct Grid {
    rows: Vec<String>,
    cols: Vec<String>,
    // width: usize,
    // height: usize,
}

impl Grid {
    fn new(data: &str) -> Self {
        let mut rows = Vec::new();
        let mut cols = vec![String::new(); data.lines().next().unwrap().len()];

        for line in data.lines() {
            rows.push(String::from(line));
            for (idx, char) in line.chars().enumerate() {
                cols[idx].push(char);
            }
        }

        Grid {
            // height: rows.len(),
            // width: cols.len(),
            rows,
            cols,
        }
    }

    fn print(&self) {
        println!("{}", self.rows.join("\n"));
    }

    fn get_mirror_lines(&self) -> MirrorLines {
        let horizontal_axis = find_mirror_point(
            self.rows.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            MirrorLines::Horizontal(0),
        );
        let vertical_axis = find_mirror_point(
            self.cols.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            MirrorLines::Vertical(0),
        );

        if horizontal_axis > 0 {
            dbg!(MirrorLines::Horizontal(horizontal_axis))
        } else if vertical_axis > 0 {
            dbg!(MirrorLines::Vertical(vertical_axis))
        } else {
            self.print();
            panic!("axis not found^");
        }
    }

    fn find_smudge_line(&self, original: MirrorLines) -> MirrorLines {
        let grid = self.rows.join("\n");
        // let new_rows = self.rows.clone();

        for (idx, char) in grid.char_indices() {
            if char == '#' || char == '.' {
                let mut new_grid = grid.clone();
                new_grid.replace_range(idx..idx + 1, if char == '#' { "." } else { "#" });

                println!("looking in new grid:\n{}", new_grid);
                let new_axis = find_mirror_point_old(
                    new_grid.split("\n").collect::<Vec<_>>(),
                    original,
                    MirrorLines::Horizontal(0),
                );
                if new_axis > 0 && MirrorLines::Horizontal(new_axis) != original {
                    return MirrorLines::Horizontal(new_axis);
                }
            }
        }
        let grid = self.cols.join("\n");
        // let new_cols = self.cols.clone();

        for (idx, char) in grid.char_indices() {
            if char == '#' || char == '.' {
                let mut new_grid = grid.clone();
                new_grid.replace_range(idx..idx + 1, if char == '#' { "." } else { "#" });
                // let mut to_print = new_grid.clone();
                // to_print.replace_range(idx..idx + 1, "[X]");
                //
                // println!("looking in new grid:\n{}", to_print);
                let new_axis = find_mirror_point_old(
                    new_grid.split("\n").collect::<Vec<_>>(),
                    original,
                    MirrorLines::Vertical(0),
                );
                if new_axis > 0 && MirrorLines::Vertical(new_axis) != original {
                    return MirrorLines::Vertical(new_axis);
                }
            }
        }

        panic!("smudge line not found");
    }
}
fn find_mirror_point(lines: Vec<&str>, this_axis: MirrorLines) -> usize {
    find_mirror_point_old(lines, MirrorLines::Horizontal(0), this_axis)
}

fn find_mirror_point_old(lines: Vec<&str>, old_axis: MirrorLines, this_axis: MirrorLines) -> usize {
    let mut axis = 1;
    while axis < lines.len() {
        let mut mirrored_lines = 0;
        let (mut left, mut right): (i32, usize) =
            ((axis - 1 - mirrored_lines) as i32, axis + mirrored_lines);
        while left >= 0 && right < lines.len() {
            // println!(
            //     "checking equality:\n[{}] {}\n[{}] {}",
            //     left, lines[left as usize], right, lines[right]
            // );
            if lines[left as usize] == lines[right] {
                // println!("true");
                mirrored_lines += 2;
                left -= 1;
                right += 1;
            } else {
                // println!("false");
                break;
            }
        }
        if !(left >= 0 && right < lines.len()) {
            println!("mirror axis found: {}", axis);
            let current_axis = match this_axis {
                MirrorLines::Horizontal(_) => MirrorLines::Horizontal(axis),
                MirrorLines::Vertical(_) => MirrorLines::Vertical(axis),
            };
            if current_axis != old_axis {
                return axis;
            }
        }
        axis += 1;
    }
    0
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MirrorLines {
    Horizontal(usize),
    Vertical(usize),
}

pub fn solve(data: String) {
    println!("{}", data);
    let patterns = data.split("\n\n");
    let mut sum = 0;
    let mut smudge_sum = 0;
    for pattern in patterns {
        let grid = Grid::new(pattern);
        let mirror_line = grid.get_mirror_lines();
        match mirror_line {
            MirrorLines::Vertical(n) => sum += n,
            MirrorLines::Horizontal(n) => sum += 100 * n,
        }
        match grid.find_smudge_line(mirror_line) {
            MirrorLines::Vertical(n) => smudge_sum += n,
            MirrorLines::Horizontal(n) => smudge_sum += 100 * n,
        }
        grid.print();
    }
    println!("sum: {}", sum);
    println!("smudge_sum: {}", smudge_sum);
}
