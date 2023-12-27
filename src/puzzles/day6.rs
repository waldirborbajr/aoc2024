#[test]
fn test() {
    solve(String::from(
        "Time:      7  15   30
Distance:  9  40  200",
    ));
}

pub fn solve(data: String) {
    let mut lines = data.split("\n");
    let times = lines.next().unwrap().split_whitespace();
    let dists = lines.next().unwrap().split_whitespace();
    let pairs = times.zip(dists);
    let mut product = 1;
    let mut time_concat = String::new();
    let mut dist_concat = String::new();
    for pair in pairs {
        if pair.0 == "Time:" {
            continue;
        }
        time_concat += pair.0;
        dist_concat += pair.1;
        let winning_times = get_winning_times(pair.0.parse().unwrap(), pair.1.parse().unwrap());
        println!("{}, {}", pair.0, pair.1);
        println!("winning times: {}", winning_times);
        product *= winning_times;
    }
    let winning_times = get_winning_times(
        time_concat.parse::<u64>().unwrap(),
        dist_concat.parse::<u64>().unwrap(),
    );
    println!("product: {}", product);
    println!(
        "concat time: {}, concat dist: {}, winning times: {}",
        time_concat, dist_concat, winning_times
    );
}

fn get_winning_times(time: u64, distance_to_beat: u64) -> u64 {
    let mut first = 0;
    for hold_time in 1..time {
        if hold_time * (time - hold_time) > distance_to_beat {
            first = hold_time;
            break;
        }
    }
    for hold_time in (1..time).rev() {
        if hold_time * (time - hold_time) > distance_to_beat {
            return hold_time - first + 1;
        }
    }
    time - first + 1
}
