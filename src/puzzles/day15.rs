#[test]
fn test() {
    solve(String::from(
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
    ));
}

fn hash(data: &str) -> usize {
    let chars = data.bytes();
    let mut h = 0;
    for c in chars {
        h += c as usize;
        h *= 17;
        h %= 256;
    }
    h
}

struct Lens {
    label: String,
    focal_length: u32,
}

pub fn solve(data: String) {
    // println!("{}", data);
    let mut sum = 0;
    let mut boxes: [Vec<Lens>; 256] = std::array::from_fn(|_| Vec::new());
    for line in data.split(",") {
        println!("{}: {}", line, hash(line.trim()));
        sum += hash(line.trim());
        if line.trim().ends_with("-") {
            let mut parts = line.split("-");
            let label = parts.next().unwrap();
            let h = hash(label);
            boxes[h as usize].retain(|lens| lens.label != label);
            println!("removing {} from box {}", label, h);
        } else {
            let mut parts = line.split("=");
            let label = parts.next().unwrap();
            let h = hash(label);
            let focal_length = parts.next().unwrap().parse().unwrap();
            if let Some(lens) = boxes[h as usize]
                .iter_mut()
                .find(|lens| lens.label == label)
            {
                (*lens).focal_length = focal_length;
            } else {
                boxes[h as usize].push(Lens {
                    label: label.to_string(),
                    focal_length,
                });
            }
            println!("adding {}={} to box {}", label, focal_length, h);
        }
    }
    let mut lens_sum = 0;
    for i in 0..256 {
        for (j, lens) in boxes[i].iter().enumerate() {
            lens_sum += (i + 1) * (j + 1) * lens.focal_length as usize;
            println!("Box {}: [{} {}]", i, lens.label, lens.focal_length);
        }
    }
    println!("sum: {}, lens_sum: {}", sum, lens_sum);
}
