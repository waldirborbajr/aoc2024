use core::{cmp::Ordering, fmt, panic};
#[test]
fn test() {
    solve(String::from(
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
    ));
}

pub fn solve(data: String) {
    let mut lines = data
        .split("\n")
        .filter(|s| s.trim() != "")
        .collect::<Vec<_>>();
    lines.sort_by(|a, b| {
        compare_cards(
            b.split_whitespace().next().unwrap(),
            a.split_whitespace().next().unwrap(),
        )
    });
    lines.sort_by(|a, b| {
        hand_from_wildcards(b.split_whitespace().next().unwrap())
            .partial_cmp(&hand_from_wildcards(a.split_whitespace().next().unwrap()))
            .unwrap()
    });
    let mut sum = 0;
    for (idx, line) in lines.iter().enumerate() {
        let mut parts = line.split_whitespace();
        //
        let cards = parts.next().unwrap();
        let bid = parts.next().unwrap().parse::<u32>().unwrap();
        println!(
            "cards: {}, counts: {}, hand: {}",
            cards,
            get_card_counts(cards.chars().collect::<Vec<_>>())
                .iter()
                .fold(String::new(), |a, v| a + &v.to_string()),
            get_hand(cards)
        );
        println!(
            "cards: {}, hand: {}, bid: {}, rank: {}, winnings: {}",
            cards,
            hand_from_wildcards(cards),
            bid,
            idx + 1,
            (idx + 1) as u32 * bid
        );
        sum += (idx + 1) as u32 * bid;
    }
    println!("total winnings: {}", sum);
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Hand {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Hand::FiveKind => "Five of a Kind",
                Hand::FourKind => "Four of a Kind",
                Hand::FullHouse => "Full House",
                Hand::ThreeKind => "Three of a Kind",
                Hand::TwoPair => "Two Pair",
                Hand::OnePair => "One Pair",
                Hand::HighCard => "High Card",
            }
        )
    }
}

fn compare_cards(hand1: &str, hand2: &str) -> Ordering {
    let ranks = "AKQT98765432J";
    let pairs = hand1.chars().zip(hand2.chars());
    for (a, b) in pairs {
        match ranks.find(a).partial_cmp(&ranks.find(b)).unwrap() {
            Ordering::Equal => continue,
            _ => return ranks.find(a).partial_cmp(&ranks.find(b)).unwrap(),
        }
    }
    Ordering::Equal
}

fn get_hand(cards: &str) -> Hand {
    let cards_chars = cards.chars().collect::<Vec<_>>();

    let counts = get_card_counts(cards_chars.clone());
    match counts.iter().max().unwrap() {
        5 => Hand::FiveKind,
        4 => Hand::FourKind,
        3 => {
            if is_full_house(cards) {
                Hand::FullHouse
            } else {
                Hand::ThreeKind
            }
        }
        2 => {
            if is_two_pair(counts) {
                Hand::TwoPair
            } else {
                Hand::TwoPair
            }
        }
        1 => Hand::HighCard,
        _ => panic!(
            "Zero counts on {}",
            cards_chars
                .iter()
                .fold(String::new(), |a, v| a + &v.to_string())
        ),
    }
    // for ele in cards {}
}

fn hand_from_wildcards(cards: &str) -> Hand {
    let mut uniques = std::collections::HashMap::new();
    for card in cards.chars().filter(|c| c != &'J') {
        if let Some(n) = uniques.get(&card) {
            uniques.insert(card, n + 1);
        } else {
            uniques.insert(card, 1);
        }
    }
    match uniques.len() {
        0 => Hand::FiveKind,
        1 => Hand::FiveKind,
        2 => {
            if uniques.values().all(|v| v != &1) {
                Hand::FullHouse
            } else {
                Hand::FourKind
            }
        }
        3 => {
            if is_three_kind(&uniques) {
                Hand::ThreeKind
            } else {
                Hand::TwoPair
            }
        }
        4 => Hand::OnePair,
        _ => Hand::HighCard,
    }
}

fn is_three_kind(uniques: &std::collections::HashMap<char, i32>) -> bool {
    let mut twos = 0;
    let mut threes = 0;
    for v in uniques.values() {
        match v {
            2 => twos += 1,
            3 => threes += 1,
            _ => continue,
        }
    }
    threes > 0 || twos < 2
}

fn get_card_counts(cards: Vec<char>) -> Vec<usize> {
    cards
        .iter()
        .map(|c| cards.iter().filter(|&ci| ci == c || ci == &'J').count())
        .collect::<Vec<_>>()
}

fn is_two_pair(counts: Vec<usize>) -> bool {
    let mut twos = 0;
    for count in counts {
        if count == 2 {
            twos += 1;
        }
    }
    twos == 4
}

fn is_full_house(cards: &str) -> bool {
    let _threes = 0;
    let _twos = 0;

    let mut uniques = std::collections::HashMap::new();
    for card in cards.chars().filter(|c| c != &'J') {
        if let Some(n) = uniques.get(&card) {
            uniques.insert(card, n + 1);
        } else {
            uniques.insert(card, 1);
        }
    }
    uniques.len() == 2 && uniques.values().all(|v| v == &2 || v == &3)
    // for count in counts {
    //     if count == 3 {
    //         threes += 1;
    //     } else if count == 2 {
    //         twos += 1;
    //     }
    // }
    // threes == 3 && twos == 2
}
