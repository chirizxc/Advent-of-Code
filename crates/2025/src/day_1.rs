mod helpers;

fn part_1(input: &str) -> anyhow::Result<i32> {
    let mut pos: i32 = 50;
    let mut count = 0;

    for line in input.trim().lines() {
        let (direction, distance) = line.split_at(1);
        let distance: i32 = distance.parse()?;

        pos = match direction {
            "L" => (pos - distance).rem_euclid(100),
            "R" => (pos + distance).rem_euclid(100),
            _ => continue,
        };

        if pos == 0 {
            count += 1;
        }
    }

    Ok(count)
}

fn part_2(input: &str) -> anyhow::Result<i32> {
    let mut pos: i32 = 50;
    let mut count = 0;

    for line in input.trim().lines() {
        let (direction, distance) = line.split_at(1);
        let distance: i32 = distance.parse()?;

        match direction {
            "L" | "R" => {
                let step = if direction == "L" { -1 } else { 1 };

                for _ in 0..distance {
                    pos = (pos + step).rem_euclid(100);
                    if pos == 0 {
                        count += 1;
                    }
                }
            }
            _ => continue,
        }
    }

    Ok(count)
}

fn main() -> anyhow::Result<()> {
    let input = helpers::read_txt("input/day_1.txt")?;
    println!("Part 1: {}", part_1(&input)?);
    println!("Part 2: {}", part_2(&input)?);
    Ok(())
}
