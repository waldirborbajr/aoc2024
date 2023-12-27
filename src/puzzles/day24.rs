#[test]
fn test() {
    solve(String::from(""));
}

pub fn solve(data: String) {
    println!("{}", data);
    for line in data.lines() {
        println!("{}", line);
    }
}
