use std::collections::{HashMap, HashSet};
#[test]
fn test() {
    solve(String::from(
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    ));
}

pub fn solve(data: String) {
    // println!("{}, {}", data, parse_num(&data).num);
    let lines = data.split("\n").collect::<Vec<_>>();
    let mut gears = HashMap::new();
    let mut sum = 0;
    let mut gear_ratio = 0;
    for idx in 0..lines.len() {
        println!("{} {} {} ", lines[idx], lines.len(), idx);
        let before = if idx > 0 {
            find_symbols(lines[idx - 1], idx - 1)
        } else {
            Vec::new()
        };
        let after = if idx < lines.len() - 1 {
            find_symbols(lines[idx + 1], idx + 1)
        } else {
            Vec::new()
        };
        for n in find_valid_numbers(before, lines[idx], after, idx, &mut gears) {
            println!("valid number: {}", n);
            sum += n;
        }
    }
    println!("sum: {}", sum);

    for (gear, nums) in gears {
        println!("gear: {} {}", gear.0, gear.1);
        if nums.len() != 2 {
            continue;
        }
        let mut product = 1;
        for num in nums {
            product *= num;
            println!("num: {}", num);
        }
        gear_ratio += product;
    }
    println!("gear_ratio: {}", gear_ratio);
}

struct SymbolInfo {
    row: usize,
    col: usize,
    gear: bool,
}
fn find_symbols(str: &str, row: usize) -> Vec<SymbolInfo> {
    let mut idxs = Vec::new();
    for (idx, char) in str.char_indices() {
        if char != '.' && !char.is_digit(10) {
            idxs.push(SymbolInfo {
                row,
                col: idx,
                gear: char == '*',
            });
        }
    }
    idxs
}

struct NumInfo {
    num: u32,
    len: usize,
}
fn parse_num(str: &str) -> NumInfo {
    let mut num = 0;
    let mut len = 0;
    for char in str.chars() {
        if let Some(d) = char.to_digit(10) {
            num *= 10;
            num += d;
            len += 1;
        } else {
            return NumInfo { num, len };
        }
    }
    NumInfo { num, len }
}

fn find_valid_numbers(
    before: Vec<SymbolInfo>,
    line: &str,
    after: Vec<SymbolInfo>,
    row: usize,
    gears: &mut HashMap<(usize, usize), HashSet<u32>>,
) -> Vec<u32> {
    let cur_symbols = find_symbols(line, row);
    let all_symbols = before.iter().chain(cur_symbols.iter()).chain(after.iter());

    let mut valid_numbers = Vec::new();
    let mut inside_num = false;
    for (idx, char) in line.char_indices() {
        if char.is_digit(10) {
            if !inside_num {
                inside_num = true;
                let NumInfo { num, len } = parse_num(&line[idx..]);
                let valid_indices = (if idx > 0 { idx - 1 } else { 0 })..idx + len + 1;
                for symbol in all_symbols.clone() {
                    println!("symbol: {} {} {}", symbol.row, symbol.col, symbol.gear);
                    if valid_indices.contains(&symbol.col) {
                        valid_numbers.push(num);
                        if !symbol.gear {
                            continue;
                        }
                        if !gears.contains_key(&(symbol.row, symbol.col)) {
                            gears.insert((symbol.row, symbol.col), HashSet::new());
                        }
                        gears
                            .get_mut(&(symbol.row, symbol.col))
                            .unwrap()
                            .insert(num);
                    }
                }
                // for valid_idx in valid_indices {
                //     if before.contains(&valid_idx)
                //         || cur_symbols.contains(&valid_idx)
                //         || after.contains(&valid_idx)
                //     {
                //         valid_numbers.push(num);
                //     }
                // }
            }
        } else {
            inside_num = false;
        }
    }
    valid_numbers
}
