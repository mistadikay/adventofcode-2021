use crate::helpers::DayData;

pub fn run(part: &i32) -> String {
    let input = parse_input();
    let answer = match part {
        2 => part2(&input).to_string(), // 1275
        1 => part1(&input).to_string(), // 1233
        _ => "".to_string(),
    };
    answer
}

fn parse_input() -> Vec<i64> {
    DayData::from_file_path("./data/day_01/input.txt")
        .lines()
        .map(|line| {
            line.parse::<i64>()
                .expect("All of the depths should have been numbers")
        })
        .collect()
}

fn part1(numbers: &Vec<i64>) -> i64 {
    return numbers.iter().enumerate().fold(0, |count, (i, v)| {
        if i > 0 && v > &numbers[i - 1] {
            count + 1
        } else {
            count
        }
    });
}

fn part2(numbers: &Vec<i64>) -> i64 {
    return numbers.iter().enumerate().fold(0, |count, (i, v)| {
        if i == 0 || i >= numbers.len() - 2 {
            count
        } else {
            let value = v + &numbers[i + 1] + &numbers[i + 2];
            let prev = &numbers[i - 1] + v + &numbers[i + 1];
            if value > prev {
                count + 1
            } else {
                count
            }
        }
    });
}
