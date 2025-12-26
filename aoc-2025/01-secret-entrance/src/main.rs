#[derive(Debug, Clone)]
struct Instruction {
    direction: char,
    amount: i32,
}

fn part1(instructions: &Vec<Instruction>) -> i32 {
    let mut dial = 50;
    let clicks = 100;
    let mut zeroes = 0;

    for instruct in instructions {
        let delta = match instruct.direction {
            'L' => dial - instruct.amount,
            'R' => dial + instruct.amount, 
            d => unreachable!("Invalid direction: {:?}", d),
        };

        dial = delta.rem_euclid(clicks);

        if dial == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

fn part2(instructions: &Vec<Instruction>) -> i32 {
    let mut dial = 50;
    let clicks = 100;
    let mut zeroes = 0;

    for instruct in instructions {
        let (delta, dist_to_zero) = match instruct.direction {
            'R' => {
                let dist_to_zero = if dial == 0 { clicks } else { clicks - dial };

                (dial + instruct.amount, dist_to_zero)
            }
            'L' => {
                let dist_to_zero = if dial == 0 { clicks } else { dial };

                (dial - instruct.amount, dist_to_zero)
            }
            d => unreachable!("Invalid direction: {:?}", d),
        };

        if instruct.amount >= dist_to_zero {
            zeroes += 1 + (instruct.amount - dist_to_zero) / clicks;
        }

        dial = delta.rem_euclid(clicks);
    }

    zeroes
}

fn main() {
    let input = "./src/input.txt";
    let content = std::fs::read_to_string(input).expect("missing file");

    let instructions: Vec<Instruction> = content
        .lines()
        .map(|line| {
            let (dir, num) = line.split_at(1);
            let direction = dir.chars().next().unwrap();
            let amount = num.parse().expect("Invalid number");
            Instruction { direction, amount }
        })
        .collect();

    println!("Part 1: result = {result}", result = part1(&instructions));
    println!("Part 2: result = {result}", result = part2(&instructions));
}
