struct Bank {
    digits: Vec<u32>,
}

fn part1(banks: &[Bank]) -> u32 {
    let max_joltage = |digits: &[u32]| -> u32 {
        let mut max_left = 0;
        let mut best = 0;

        for &d in digits {
            best = best.max(max_left * 10 + d);
            max_left = max_left.max(d);
        }

        best
    };

    banks.iter().map(|bank| max_joltage(&bank.digits)).sum()
}

fn part2(banks: &[Bank]) -> u64 {
    let max_joltage = |digits: &[u32]| -> u64 {
        let mut best = [0u64; 13];

        for &d in digits {
            let d = d as u64;

            for len in (1..=12).rev() {
                best[len] = best[len].max(best[len - 1] * 10 + d);
            }
        }

        best[12]
    };

    banks.iter().map(|bank| max_joltage(&bank.digits)).sum()
}

fn main() {
    let input = "./src/input.txt";
    let content = std::fs::read_to_string(input).expect("missing file");

    let banks: Vec<Bank> = content
        .lines()
        .map(|l| {
            let digits = l.chars().map(|c| c.to_digit(10).unwrap()).collect();
            Bank { digits }
        })
        .collect();

    println!("Part 1: result = {result}", result = part1(&banks));
    println!("Part 2: result = {result}", result = part2(&banks));
}
