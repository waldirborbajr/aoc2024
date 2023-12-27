pub fn solve(data: String) {
    let lines = data.split("\n");
    let mut sum = 0;

    for line in lines {
        println!("{}", line);
        let l = find_number(line);
        let r = rfind_number(line);
        match (l, r) {
            (Some(l), Some(r)) => {
                println!("{}, {}", l, r);
                sum += l * 10;
                sum += r;
            }
            _ => continue,
        }
        println!("{}", sum)
    }
    println!("{}", sum)
}

fn to_number(s: &str) -> Option<u32> {
    match s {
        s if s.len() == 1 => {
            if let Some(n) = s.chars().nth(0).unwrap().to_digit(10) {
                Some(n)
            } else {
                None
            }
        }
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn find_number(s: &str) -> Option<u32> {
    for i in 0..s.len() {
        for j in 1..6 {
            if i + j > s.len() {
                continue;
            }
            match to_number(&s[i..i + j]) {
                Some(n) => return Some(n),
                None => continue,
            }
        }
    }
    None
}
fn rfind_number(s: &str) -> Option<u32> {
    for i in (0..s.len()).rev() {
        for j in 1..6 {
            if i + j > s.len() {
                continue;
            }
            match to_number(&s[i..i + j]) {
                Some(n) => return Some(n),
                None => continue,
            }
        }
    }
    None
}
