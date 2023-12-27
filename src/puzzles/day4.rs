#[test]
fn test() {
    solve(String::from(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ));
}

pub fn solve(data: String) {
    let cards = data.split("\n");
    let mut point_sum = 0;
    let mut card_sum = 0;
    let mut copies = [1; 200];
    for card in cards {
        if card.trim() == "" {
            continue;
        }
        let halves: Vec<_> = card.split("|").collect();
        let my_nums: Vec<_> = halves[1].split(" ").collect();
        let first_quarters: Vec<_> = halves[0].split(":").collect::<Vec<_>>();
        let card_num = first_quarters[0].split_whitespace().collect::<Vec<_>>()[1]
            .parse::<usize>()
            .unwrap();
        let winning_nums = first_quarters[1].split(" ").collect();
        let winners = winning_count(winning_nums, my_nums);
        point_sum += card_points(winners);
        println!("card_num for {}: {}", card, card_num);
        card_sum += copies[card_num];
        for card_to_copy in card_num + 1..card_num + 1 + winners as usize {
            copies[card_to_copy] += copies[card_num];
        }
    }
    println!("point_sum: {}, card_sum: {}", point_sum, card_sum);
}

pub fn winning_count(nums: Vec<&str>, my_nums: Vec<&str>) -> u32 {
    let mut winning_count = 0;
    for my_num in my_nums {
        if my_num.trim() == "" {
            continue;
        }
        if nums.contains(&my_num) {
            winning_count += 1;
        }
    }
    winning_count
}

fn card_points(winning_count: u32) -> u32 {
    match winning_count {
        0 => 0,
        _ => 2_u32.pow(winning_count - 1),
    }
}
