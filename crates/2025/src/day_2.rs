mod helpers;

fn part_1(input: &str) -> anyhow::Result<u64> {
    let mut total = 0u64;

    for range in input.trim().split(',') {
        let mut parts = range.split('-');
        let start: u64 = parts.next().unwrap_or("0").parse()?;
        let end: u64 = parts.next().unwrap_or("0").parse()?;

        for id in start..=end {
            let id_str = id.to_string();

            if id_str.len() % 2 == 0 {
                let half = id_str.len() / 2;
                if id_str[..half] == id_str[half..] {
                    total += id;
                }
            }
        }
    }

    Ok(total)
}

fn part_2(input: &str) -> anyhow::Result<u64> {
    let mut total = 0u64;

    for range in input.trim().split(',') {
        let mut parts = range.split('-');
        let start: u64 = parts.next().unwrap_or("0").parse()?;
        let end: u64 = parts.next().unwrap_or("0").parse()?;

        for id in start..=end {
            let id_str = id.to_string();
            let len = id_str.len();

            for p_len in 1..=len / 2 {
                if len % p_len != 0 {
                    continue;
                }

                let pattern = &id_str[..p_len];
                let mut is_valid = true;

                for chunk in id_str.as_bytes().chunks_exact(p_len) {
                    if chunk != pattern.as_bytes() {
                        is_valid = false;
                        break;
                    }
                }

                if is_valid && len / p_len >= 2 {
                    total += id;
                    break;
                }
            }
        }
    }

    Ok(total)
}

fn main() -> anyhow::Result<()> {
    let input = helpers::read_txt("input/day_2.txt")?;
    println!("Part 1: {}", part_1(&input)?);
    println!("Part 2: {}", part_2(&input)?);
    Ok(())
}
