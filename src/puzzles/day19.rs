use std::collections::HashMap;

use regex::Regex;

#[test]
fn test() {
    solve(String::from(
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
    ));
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

// struct PartCombos {
//     x: (u64, u64),
//     m: (u64, u64),
//     a: (u64, u64),
//     s: (u64, u64),
// }

// impl PartCombos {
//     fn new() -> Self {
//         PartCombos {
//             x: (1, 4000),
//             m: (1, 4000),
//             a: (1, 4000),
//             s: (1, 4000),
//         }
//     }
//
//     fn reduce_range(&mut self, cond: &str) {
//         let mut cond_chars = cond.chars();
//         let cmp: u64;
//
//         match (cond_chars.next().unwrap(), cond_chars.next().unwrap()) {
//             ('x', '<') => {
//                 cmp = cond.split("<").nth(1).unwrap().parse().unwrap();
//                 self.x.1 = min(self.x.1, cmp - 1);
//             }
//             ('x', '>') => {
//                 cmp = cond.split(">").nth(1).unwrap().parse().unwrap();
//                 self.x.0 = max(self.x.0, cmp + 1);
//             }
//             ('m', '<') => {
//                 cmp = cond.split("<").nth(1).unwrap().parse().unwrap();
//                 self.m.1 = min(self.m.1, cmp - 1);
//             }
//             ('m', '>') => {
//                 cmp = cond.split(">").nth(1).unwrap().parse().unwrap();
//                 self.m.0 = max(self.m.0, cmp + 1);
//             }
//             ('a', '<') => {
//                 cmp = cond.split("<").nth(1).unwrap().parse().unwrap();
//                 self.a.1 = min(self.a.1, cmp - 1);
//             }
//             ('a', '>') => {
//                 cmp = cond.split(">").nth(1).unwrap().parse().unwrap();
//                 self.a.0 = max(self.a.0, cmp + 1);
//             }
//             ('s', '<') => {
//                 cmp = cond.split("<").nth(1).unwrap().parse().unwrap();
//                 self.s.1 = min(self.s.1, cmp - 1);
//             }
//             ('s', '>') => {
//                 cmp = cond.split(">").nth(1).unwrap().parse().unwrap();
//                 self.s.0 = max(self.s.0, cmp + 1);
//             }
//             _ => panic!("error matching condition string: {}", cond),
//         };
//     }
// }

#[derive(Debug)]
struct Workflow {
    jobs: Vec<String>,
}

impl Part {
    fn new(data: &str) -> Self {
        println!("making part: {}", data);
        let re = Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap();
        let caps = re.captures(data).unwrap();
        Part {
            x: caps["x"].parse().unwrap(),
            m: caps["m"].parse().unwrap(),
            a: caps["a"].parse().unwrap(),
            s: caps["s"].parse().unwrap(),
        }
    }

    fn sum(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

impl Workflow {
    fn process_part(&self, part: &Part, workflows: &HashMap<&str, Workflow>) -> bool {
        println!("processing part through {}", self.jobs.join(","));
        for job in &self.jobs {
            let (cond, target) = if job.contains(":") {
                let mut parts = job.split(":");

                let cond_str = parts.next().unwrap();
                let mut cond_chars = cond_str.chars();
                (
                    match (cond_chars.next().unwrap(), cond_chars.next().unwrap()) {
                        ('x', '<') => {
                            let cmp = cond_str.split("<").nth(1).unwrap().parse().unwrap();
                            part.x < cmp
                        }
                        ('x', '>') => {
                            let cmp = cond_str.split(">").nth(1).unwrap().parse().unwrap();
                            part.x > cmp
                        }
                        ('m', '<') => {
                            let cmp = cond_str.split("<").nth(1).unwrap().parse().unwrap();
                            part.m < cmp
                        }
                        ('m', '>') => {
                            let cmp = cond_str.split(">").nth(1).unwrap().parse().unwrap();
                            part.m > cmp
                        }
                        ('a', '<') => {
                            let cmp = cond_str.split("<").nth(1).unwrap().parse().unwrap();
                            part.a < cmp
                        }
                        ('a', '>') => {
                            let cmp = cond_str.split(">").nth(1).unwrap().parse().unwrap();
                            part.a > cmp
                        }
                        ('s', '<') => {
                            let cmp = cond_str.split("<").nth(1).unwrap().parse().unwrap();
                            part.s < cmp
                        }
                        ('s', '>') => {
                            let cmp = cond_str.split(">").nth(1).unwrap().parse().unwrap();
                            part.s > cmp
                        }
                        _ => panic!("error matching condition string: {}", cond_str),
                    },
                    parts.next().unwrap(),
                )
            } else {
                (true, job.as_str())
            };
            if cond {
                match target {
                    "A" => return true,
                    "R" => return false,
                    _ => {
                        return workflows
                            .get(dbg!(target))
                            .unwrap()
                            .process_part(part, workflows)
                    }
                }
            }
        }
        dbg!(&self.jobs);
        panic!("reached end of job list");
    }
}

pub fn solve(data: String) {
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();
    let mut lines = data.lines();
    for line in &mut lines {
        if line == "" {
            break;
        }
        let mut parts = line.split("{");
        let name = parts.next().unwrap();
        let jobs = parts.next().unwrap().split("}").next().unwrap();
        let jobs = jobs.split(",");
        let jobs = jobs.map(|s| s.to_owned()).collect::<Vec<_>>();

        workflows.insert(name, Workflow { jobs });
    }
    for line in &mut lines {
        parts.push(Part::new(line));
    }
    let in_flow = workflows.get("in").unwrap();
    let accepted = parts.iter().filter(|p| in_flow.process_part(p, &workflows));
    let sum: i32 = accepted.map(|p| p.sum()).sum();
    println!("sum: {}", sum);
}
