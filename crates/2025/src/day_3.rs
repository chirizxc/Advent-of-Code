mod helpers;

fn part_1(input: &str) -> u64 {
    let mut joltage = 0u64;

    for line in input.trim().lines() {
        let mut maximum_joltage = 0u64;

        for i in 0..line.len() {
            for j in (i + 1)..line.len() {
                let char1 = line.chars().nth(i).unwrap();
                let char2 = line.chars().nth(j).unwrap();

                if let (Some(d1), Some(d2)) = (char1.to_digit(10), char2.to_digit(10)) {
                    let val = u64::from(d1) * 10 + u64::from(d2);
                    if val > maximum_joltage {
                        maximum_joltage = val;
                    }
                }
            }
        }

        joltage += maximum_joltage;
    }

    joltage
}

fn part_2(input: &str) -> u64 {
    let mut joltage = 0u64;

    for line in input.trim().lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut vec = Vec::new();
        let to_remove = chars.len().saturating_sub(12);

        for (i, &c) in chars.iter().enumerate() {
            while !vec.is_empty()
                && to_remove > 0
                && vec.len() + (chars.len() - i) > 12
                && c > *vec.last().unwrap()
            {
                vec.pop();
            }
            if vec.len() < 12 {
                vec.push(c);
            }
        }

        let val: u64 = vec
            .iter()
            .filter_map(|&c| c.to_digit(10))
            .fold(0u64, |acc, d| acc * 10 + u64::from(d));

        joltage += val;
    }

    joltage
}

fn main() -> anyhow::Result<()> {
    let input = helpers::read_txt("input/day_3.txt")?;
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
    Ok(())
}
