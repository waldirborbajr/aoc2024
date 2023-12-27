#[test]
fn test_overlap() {
    assert_eq!(get_overlap(0, 10, 2, 8), Overlap::BothOverlap(2, 2));
    assert_eq!(
        get_overlap(100, 500, 300, 400),
        Overlap::BothOverlap(200, 100)
    );
    assert_eq!(get_overlap(0, 10, 5, 15), Overlap::LeftOverlap(5));
    assert_eq!(get_overlap(0, 10, -5, 15), Overlap::Contained);
    assert_eq!(get_overlap(0, 10, 11, 20), Overlap::Disjoint);
    assert_eq!(get_overlap(0, 10, -5, 8), Overlap::RightOverlap(8));
}
#[test]
fn test_maps() {
    let range = MapRange {
        src: 0,
        dest: 10,
        len: 9,
    };
    let range2 = MapRange {
        src: 50,
        dest: 150,
        len: 9,
    };
    let seed_map = SeedMap {
        ranges: vec![range, range2],
    };

    dbg!(seed_map.get_out_ranges(&InputRange {
        start: -5,
        len: 200
    }));
}
#[test]
fn test() {
    solve(String::from(
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
    ));
}

#[derive(Debug, PartialEq)]
enum Overlap {
    Disjoint,
    LeftOverlap(i64),
    Contained,
    RightOverlap(i64),
    BothOverlap(i64, i64),
}

fn get_overlap(start: i64, end: i64, other_start: i64, other_end: i64) -> Overlap {
    println!(
        "getting overlap between ({}..{}) and ({}..{})",
        start, end, other_start, other_end
    );
    if end < other_start || start > other_end {
        dbg!(Overlap::Disjoint)
    } else if start < other_start && end <= other_end {
        dbg!(Overlap::LeftOverlap(end - other_start))
    } else if start >= other_start && end <= other_end {
        dbg!(Overlap::Contained)
    } else if start >= other_start && end > other_end {
        dbg!(Overlap::RightOverlap(other_end - start))
    } else if start < other_start && end > other_end {
        dbg!(Overlap::BothOverlap(other_start - start, end - other_end))
    } else {
        panic!(
            "Error calculating overlap: ({}..{}) ({}..{})",
            start, end, other_start, other_end
        );
    }
}

#[derive(Debug, Copy, Clone)]
struct InputRange {
    start: i64,
    len: i64,
}

#[derive(Debug)]
struct MapRange {
    dest: i64,
    src: i64,
    len: i64,
}

struct SeedMap {
    ranges: Vec<MapRange>,
}

impl SeedMap {
    fn get_out_ranges(&self, in_range: &InputRange) -> Vec<InputRange> {
        let mut out_ranges = Vec::new();
        let mut leftovers: Vec<InputRange> = vec![in_range.to_owned()];
        for range in &self.ranges {
            // print!("Range: ");
            // dbg!(range);
            let mut new_leftovers: Vec<InputRange> = Vec::new();
            for leftover in leftovers {
                // print!("Leftover: ");
                // dbg!(leftover);
                match get_overlap(
                    leftover.start,
                    leftover.start + leftover.len,
                    range.src,
                    range.src + range.len,
                ) {
                    Overlap::Disjoint => {
                        new_leftovers.push(leftover);
                    }
                    Overlap::LeftOverlap(n) => {
                        out_ranges.push(InputRange {
                            start: range.dest,
                            len: n,
                        });
                        new_leftovers.push(InputRange {
                            start: leftover.start,
                            len: range.src - leftover.start,
                        });
                    }
                    Overlap::Contained => {
                        out_ranges.push(InputRange {
                            start: range.dest + (leftover.start - range.src),
                            len: leftover.len,
                        });
                    }
                    Overlap::RightOverlap(n) => {
                        out_ranges.push(InputRange {
                            start: leftover.start,
                            len: n,
                        });
                        new_leftovers.push(InputRange {
                            start: range.src + range.len,
                            len: leftover.start + leftover.len - (range.src + range.len),
                        });
                    }
                    Overlap::BothOverlap(l, r) => {
                        out_ranges.push(InputRange {
                            start: range.dest,
                            len: range.len,
                        });
                        new_leftovers.push(InputRange {
                            start: leftover.start,
                            len: l,
                        });
                        new_leftovers.push(InputRange {
                            start: range.src + range.len,
                            len: r,
                        });
                    }
                }
            }
            leftovers = new_leftovers;
        }
        [out_ranges, leftovers].concat()
    }
}

struct Maps {
    maps: Vec<SeedMap>,
}

impl Maps {
    fn walk_maps(&self, start: Vec<InputRange>) -> Vec<InputRange> {
        let mut intermediate_ranges = start;
        for map in &self.maps {
            intermediate_ranges = intermediate_ranges
                .iter()
                .flat_map(|r| map.get_out_ranges(r))
                .collect();
        }
        intermediate_ranges
    }
}

pub fn solve(data: String) {
    let mut lines = data.split("\n");
    let _seeds = get_seeds(lines.next().unwrap());
    let mut maps = Maps { maps: Vec::new() };
    for line in lines {
        if line.contains("map:") {
            maps.maps.push(SeedMap { ranges: Vec::new() });
        } else if line.trim() == "" {
            continue;
        } else {
            maps.maps.last_mut().unwrap().ranges.push(get_ranges(line));
        }
    }
    // let ends = seeds.into_iter().map(|n| walk_maps(n, &maps));
    // println!("min location: {}", ends.min().unwrap());
    let inputs = data.lines().next().unwrap();
    let input_pairs = inputs.split_whitespace().skip(1).collect::<Vec<_>>();
    let mut input_ranges = Vec::new();
    for pair in input_pairs.chunks(2) {
        input_ranges.push(InputRange {
            start: pair[0].parse().unwrap(),
            len: pair[1].parse().unwrap(),
        });
    }
    let out_ranges = maps.walk_maps(input_ranges);
    let min_start = out_ranges.iter().map(|r| r.start).min().unwrap();
    println!("min: {}", min_start);
}

fn get_seeds(seed_line: &str) -> Vec<i64> {
    let parts = seed_line.split_whitespace();
    let mut nums = Vec::new();
    for part in parts {
        if let Ok(n) = part.parse::<i64>() {
            nums.push(n);
        }
    }
    nums
}

fn _walk_maps(seed: i64, maps: &Vec<Vec<MapRange>>) -> i64 {
    let mut curr = seed;
    'outer: for map in maps {
        for range in map {
            if range.src <= curr && curr < range.src + range.len {
                curr = range.dest + (curr - range.src);
                continue 'outer;
            }
        }
    }
    curr
}

fn get_ranges(number_line: &str) -> MapRange {
    let nums = number_line.split_whitespace();
    let nums_parsed = nums
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    MapRange {
        dest: nums_parsed[0],
        src: nums_parsed[1],
        len: nums_parsed[2],
    }
}
