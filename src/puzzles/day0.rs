pub fn solve(data: String) {
    println!("Text input: {}", data);
    let data = data.split("\n");
    let mut max_sum = 0;
    let mut sum = 0;
    for line in data {
        println!("Line: {}", line);
        if line.trim() == "" {
            println!("Sum: {}", sum);
            if sum > max_sum {
                max_sum = sum;
            }
            sum = 0;
        } else {
            let num = line.parse::<i32>();
            match num {
                Ok(num) => {
                    sum += num;
                }
                Err(error) => {
                    println!("Error parsing number: {}", error);
                }
            }
        }
    }
    println!("Max sum: {}", max_sum);
}
