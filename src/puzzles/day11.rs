use std::collections::HashSet;

#[test]
fn test() {
    solve(String::from(
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
    ));
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

struct GalaxyMap {
    galaxies: Vec<Coords>,
    occupied_rows: HashSet<usize>,
    occupied_cols: HashSet<usize>,
}

impl GalaxyMap {
    fn new(data: String) -> GalaxyMap {
        let mut occupied_rows = HashSet::new();
        let mut occupied_cols = HashSet::new();
        let mut galaxies = Vec::new();

        for (y, line) in data.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                if tile == '#' {
                    occupied_rows.insert(y);
                    occupied_cols.insert(x);
                    galaxies.push(Coords { x, y });
                }
            }
        }

        GalaxyMap {
            galaxies,
            occupied_cols,
            occupied_rows,
        }
    }

    fn measure_distance(&self, g1: &Coords, g2: &Coords) -> usize {
        let initial = g1.x.abs_diff(g2.x) + g1.y.abs_diff(g2.y);

        let expand = if g1.x < g2.x { g1.x..g2.x } else { g2.x..g1.x }
            .filter(|v| !self.occupied_cols.contains(v.try_into().unwrap()))
            .count()
            + if g1.y < g2.y { g1.y..g2.y } else { g2.y..g1.y }
                .filter(|v| !self.occupied_rows.contains(v.try_into().unwrap()))
                .count();

        initial + expand * 999999
    }

    fn find_distances(&self) -> usize {
        let mut total = 0;
        for (idx, galaxy) in self.galaxies.iter().enumerate() {
            let rest = self.galaxies.iter().skip(idx);

            for (other_idx, other) in rest.enumerate() {
                let dist = self.measure_distance(galaxy, other);
                println!(
                    "dist between {} and {}: {}",
                    idx + 1,
                    idx + 1 + other_idx,
                    dist
                );
                total += dist;
            }
        }
        total
    }
}

pub fn solve(data: String) {
    println!("{}", data);
    let map = GalaxyMap::new(data);
    println!("distances: {}", map.find_distances());
}
