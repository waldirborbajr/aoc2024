#[test]
fn test() {
    solve(String::from(
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
    ));
    solve(String::from(
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
    ));
}

pub fn solve(data: String) {
    let mut lines = data.split("\n");
    let sequence = lines.next().unwrap();
    lines.next();
    let mut map = std::collections::HashMap::new();
    let mut heres = std::collections::HashSet::new();
    for line in lines {
        if line.trim() == "" {
            continue;
        }
        let mut parts = line.split("=");
        let key = parts.next().unwrap().trim();
        let paths = parts.next().unwrap().trim_matches(&[' ', '(', ')'] as &[_]);
        let mut paths = paths.split(", ");
        map.insert(key, (paths.next().unwrap(), paths.next().unwrap()));
        if key.ends_with("A") {
            heres.insert(key);
        }
    }
    // for (key, (left, right)) in map {
    //     println!("{} = ({}, {})", key, left, right);
    // }
    let mut steps = 0;
    let mut here = "AAA";
    let mut seq_chars = sequence.chars();

    while map.contains_key("AAA") && here != "ZZZ" {
        if let Some(dir) = seq_chars.next() {
            let paths = map.get(here).unwrap();
            here = match dir {
                'L' => paths.0,
                'R' => paths.1,
                _ => panic!("invalid dir"),
            };
            steps += 1;
        } else {
            seq_chars = sequence.chars();
        }
    }
    println!("steps to reach ZZZ: {}", steps);

    for here in heres {
        println!(
            "steps to reach end from {}: {}",
            here,
            steps_to_end(here, sequence, &map)
        );
    }

    // println!("steps to reach all end in Z: {}", steps);
}

fn steps_to_end(
    start: &str,
    seq: &str,
    map: &std::collections::HashMap<&str, (&str, &str)>,
) -> u32 {
    let mut steps = 0;
    let mut seq_chars = seq.chars();
    let mut here = start;

    while !here.ends_with("Z") {
        if let Some(dir) = seq_chars.next() {
            // print!("\nHeres:");
            // for here in &heres {
            //     print!(" {}", here);
            // }
            // if steps % 500000 == 0 {
            //     println!("steps: {}", steps);
            // }
            let paths = map.get(here).unwrap();
            here = match dir {
                'L' => paths.0,
                'R' => paths.1,
                _ => panic!("invalid dir"),
            };
            steps += 1;
        } else {
            seq_chars = seq.chars();
        }
    }
    steps
}
