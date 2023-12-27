use std::collections::HashMap;

pub fn solve(data: String) {
    println!("day2");
    let lines = data.split("\n");
    let mut sum = 0;
    let mut power_sum = 0;
    for line in lines {
        let parts = line.split(":").collect::<Vec<&str>>();
        let mut valid = true;
        println!("{}", line);
        if parts.len() != 2 {
            continue;
        }
        let sets = parts[1].split(";");
        power_sum += find_power(sets.clone().into_iter().collect());
        for set in sets {
            let cubes = set.split(",");
            for cube in cubes {
                let words = cube.trim().split(" ").collect::<Vec<&str>>();
                if let Ok(n) = words[0].parse::<i32>() {
                    if n > match words[1] {
                        "red" => 12,
                        "green" => 13,
                        "blue" => 14,
                        _ => unimplemented!("invalid color: {}", cube),
                    } {
                        valid = false;
                    }
                }
            }
        }
        println!("valid? {}", valid);
        if valid {
            sum += parts[0].trim().split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
        }
    }
    println!("{}, {}", sum, power_sum)
}

fn find_power(sets: Vec<&str>) -> i32 {
    let mut cubes: HashMap<&str, i32> = HashMap::new();
    for set in sets {
        for cube in set.split(", ") {
            let (num, color) = get_cube_data(cube);
            if !cubes.contains_key(color) || cubes.get(color).unwrap() < &num {
                cubes.insert(color, num);
            }
        }
    }
    for (color, num) in &cubes {
        println!("{}: {}", color, num)
    }
    cubes.into_values().reduce(|a, b| a * b).unwrap()
}

fn get_cube_data(s: &str) -> (i32, &str) {
    let words = s.trim().split(" ").collect::<Vec<&str>>();
    if let Ok(n) = words[0].parse::<i32>() {
        return (n, words[1]);
    }
    panic!("Invalid cube string {}", s);
}
