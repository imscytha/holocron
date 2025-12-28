#[derive(Debug)]
struct Range(i64, i64);

fn part1(entries: &[Range]) -> i64 {
    // NOTE: put it in a closure since part2 can have different invalid check.
    let is_invalid_id = |num: i64| -> bool {
        let num_str = num.to_string();
        let len = num_str.len();

        if !len.is_multiple_of(2) {
            return false;
        }
        len.is_multiple_of(2) && {
            let (first, second) = num_str.split_at(len / 2);
            first == second
        }
    };

    let mut result = 0;
    for entry in entries {
        for num in entry.0..=entry.1 {
            if is_invalid_id(num) {
                result += num;
            }
        }
    }

    result
}

fn part2(entries: &[Range]) -> i64 {
    let is_invaild_id = |num: i64| -> bool {
        let num_str = num.to_string();
        let num_len = num_str.len();

        let seq = num_str.clone() + &num_str;
        seq[1..(2*num_len-1)].contains(&num_str)
    };

    let mut result = 0;
    for entry in entries {
        for num in entry.0..=entry.1 {
            if is_invaild_id(num) {
                result += num;
            }
        }
    }
    result
}

fn main() {
    let input = "./src/input.txt";
    let content = std::fs::read_to_string(input).expect("missing file");

    let entries: Vec<Range> = content
        .split(',')
        .map(|e| {
            let mut parts = e.split('-').filter_map(|s| s.trim().parse().ok());

            let first = parts.next().expect("first range");
            let last = parts.next().expect("last range");

            Range(first, last)
        })
        .collect();

    println!("Part 1: result = {result}", result = part1(&entries));
    println!("Part 2: result = {result}", result = part2(&entries));
}
