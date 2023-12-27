#[test]
fn test() {
    solve(String::from(
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
    ));
}

pub fn solve(data: String) {
    let lines = data.split("\n");
    let mut sum = 0;
    for line in lines {
        let nums = line
            .split_whitespace()
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
        // let derivative = find_derivative(&nums);
        // print!("\n{}: ", line);
        // for val in derivative {
        //     print!(" {}", val);
        // }
        let next = predict_next(&nums);
        println!("line: {}, next: {}", line, next);
        sum += next;
    }
    println!("sum: {}", sum);
}

fn predict_next(values: &Vec<i32>) -> i32 {
    if values.iter().all(|v| v == &0) {
        0
    } else {
        let derivative = find_derivative(&values);
        let next = values.first().unwrap() - predict_next(&derivative);
        // println!("next: {}", next);
        next
    }
}

fn find_derivative(values: &Vec<i32>) -> Vec<i32> {
    let mut derivative = Vec::new();
    for i in 0..values.len() - 1 {
        derivative.push(values[i + 1] - values[i]);
    }
    // println!("last: {}", derivative.last().unwrap());
    derivative
}
