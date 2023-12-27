use core::panic;
use std::collections::HashMap;

#[test]
fn test() {
    solve(String::from(
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        // "## 2,1",
    ));
}
#[test]
fn test2() {
    // dbg!(count_valid_configurations("?###???????? 3,2,1"));
}

#[derive(Debug, Clone)]
enum SpringRun {
    Operational(u32),
    Damaged(u32),
    Unknown(u32),
}

#[derive(Debug, Clone)]
struct SpringRow {
    inside_group: bool,
    // spring_runs: Vec<SpringRun>,
    springs: String,
    groups: Vec<u32>,
}

impl SpringRow {
    fn new(data: &str) -> Self {
        let mut parts = data.split_whitespace();
        let springs = parts.next().unwrap();
        let groups = parts.next().unwrap();

        let mut spring_chars = Vec::new();

        let mut five_springs = String::from(springs);
        let mut five_groups = String::from(groups);
        for _ in 0..4 {
            five_springs.push('?');
            five_springs.push_str(springs);
            five_groups.push(',');
            five_groups.push_str(groups);
        }

        let springs = five_springs;
        let groups = five_groups;

        let mut chars = springs.chars();
        let mut curr = chars.next().unwrap();
        let mut num = 1;
        let mut runs = Vec::new();
        for char in chars {
            if char == curr {
                num += 1;
            } else {
                runs.push(match curr {
                    '.' => SpringRun::Operational(num),
                    '#' => SpringRun::Damaged(num),
                    '?' => SpringRun::Unknown(num),
                    _ => panic!("Encountered unknown spring character while parsing row"),
                });
                num = 1;
            }
            spring_chars.push(char);
            curr = char;
        }
        runs.push(match curr {
            '.' => SpringRun::Operational(num),
            '#' => SpringRun::Damaged(num),
            '?' => SpringRun::Unknown(num),
            _ => panic!("Encountered unknown spring character while parsing row"),
        });

        SpringRow {
            inside_group: false,
            // spring_runs: runs,
            springs: springs.to_string(),
            groups: groups
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>(),
        }
    }

    // fn is_valid(&self) -> Option<bool> {
    //     let damaged_runs = self
    //         .spring_runs
    //         .iter()
    //         .filter(|c| matches!(c, SpringRun::Damaged(_)))
    //         .map(|c| {
    //             if let SpringRun::Damaged(n) = c {
    //                 n
    //             } else {
    //                 panic!("error in is_valid finding damaged_runs")
    //             }
    //         });
    //     if damaged_runs.clone().count() == self.groups.len()
    //         && damaged_runs.zip(&self.groups).all(|(a, b)| a == b)
    //     {
    //         Some(true)
    //     } else {
    //         if self
    //             .spring_runs
    //             .iter()
    //             .any(|c| matches!(c, SpringRun::Unknown(_)))
    //         {
    //             None
    //         } else {
    //             Some(false)
    //         }
    //     }
    // }

    fn to_str(&self) -> String {
        let mut str = self.springs.clone();
        for i in &self.groups {
            str.push_str(format!("{},", i).as_str());
        }
        str
    }

    fn try_consume_group(
        &self,
        depth: usize,
        start: &str,
        memo: &mut HashMap<String, usize>,
    ) -> usize {
        if let Some(n) = memo.get(self.to_str().as_str()) {
            println!("saved function call: {} ({})", self.to_str(), n);
            return *n;
        }
        // let indent = "  ".repeat(depth);
        let mut start_chunk = String::from(start);
        // println!("{}try_consume_group {}/{}", indent, start, self.to_str());
        let mut self_clone = self.clone();

        for char in self_clone.springs.clone().chars() {
            match char {
                '.' => {
                    self_clone.springs.remove(0);
                    start_chunk.push('.');
                    if let Some(0) = self_clone.groups.first() {
                        self_clone.inside_group = false;
                        //println!("{}removing 0 group", indent);
                        self_clone.groups.remove(0);
                    } else {
                        if self_clone.inside_group {
                            // println!("inserting {}/{} (0)", start_chunk, self_clone.to_str());
                            // memo.insert(format!("{}{}", start_chunk, self_clone.to_str()), 0);
                            return 0;
                        }
                        continue;
                    }
                }
                '#' => {
                    self_clone.springs.remove(0);
                    start_chunk.push('#');
                    self_clone.inside_group = true;
                    if let Some(n) = self_clone.groups.first_mut() {
                        if *n == 0 {
                            // println!("{}FAILED", indent);
                            memo.insert(format!("{}{}", start_chunk, self_clone.springs), 0);
                            return 0;
                        }
                        // println!("{}# found - decreasing group", indent);
                        *n -= 1
                    } else {
                        // println!("{}FAILED", indent);
                        memo.insert(format!("{}{}", start_chunk, self_clone.springs), 0);
                        return 0;
                    }
                }
                '?' => {
                    let mut filled = self_clone.clone();
                    let mut empty = self_clone.clone();
                    filled.springs = filled.springs.replacen("?", "#", 1);
                    empty.springs = empty.springs.replacen("?", ".", 1);
                    let filled_configs =
                        filled.try_consume_group(depth + 1, start_chunk.as_str(), memo);
                    let empty_configs =
                        empty.try_consume_group(depth + 1, start_chunk.as_str(), memo);
                    memo.insert(format!("{}", filled.to_str()), filled_configs);
                    // memo.insert(format!("{}", empty.to_str()), empty_configs);
                    memo.insert(
                        format!("{}", self_clone.to_str()),
                        filled_configs + empty_configs,
                    );

                    return filled_configs + empty_configs;
                }
                _ => panic!("unrecognized character {}", char),
            }
        }

        if self_clone.groups.len() == 0
            || (self_clone.groups.len() == 1 && *self_clone.groups.first().unwrap() == 0)
        {
            // println!("valid configuration found: {}", start_chunk);
            memo.insert(format!("{}", start_chunk), 1);
            return 1;
        } else {
            // println!("{}FAILED", indent);
            // println!(
            //     "{}end of string reached with nonzero groups left - returning 0",
            //     indent
            // );
            memo.insert(format!("{}", start_chunk), 0);
            return 0;
        }
    }
}

// fn count_valid_configurations(data: &str) -> usize {
//     // println!("counting valid configurations for {}", data);
//     let row = SpringRow::new(data);
//     match row.is_valid() {
//         Some(valid) => {
//             if valid {
//                 1
//             } else {
//                 0
//             }
//         }
//         None => {
//             let parts = data.splitn(2, "?");
//             // let next_row = remove_first_question(data);
//             count_valid_configurations(&format!(
//                 "{}.{}",
//                 parts.clone().next().unwrap(),
//                 parts.clone().nth(1).unwrap()
//             )) + count_valid_configurations(&format!(
//                 "{}#{}",
//                 parts.clone().next().unwrap(),
//                 parts.clone().nth(1).unwrap()
//             ))
//         }
//     }
// }

// fn remove_first_question(data: &str) -> String {
//     data.chars()
//         .skip_while(|c| c != &'?')
//         .skip(1)
//         .collect::<String>()
// }

pub fn solve(data: String) {
    // println!("{}", data);
    let mut sum = 0;
    let mut memo = HashMap::new();
    for line in data.lines() {
        let row = SpringRow::new(line);
        // let mut parts = line.split_whitespace();
        // let springs = parts.next().unwrap().repeat(5);
        // let groups = parts.next().unwrap().repeat(5);
        // let configs = count_valid_configurations(&format!("{} {}", springs, groups));
        // let configs = count_valid_configurations(line);
        let configs = row.try_consume_group(0, "", &mut memo);
        // dbg!(row);
        // println!("valid configurations for {}: {}", line, configs);
        sum += configs;
    }
    println!("total configs: {}", sum);
}
