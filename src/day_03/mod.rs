use crate::helpers::DayData;

pub fn run(part: &i32) -> String {
    let input = parse_input();
    let answer = match part {
        1 => part1(&input).to_string(), // 3847100
        2 => part2(&input).to_string(), // 4105235
        _ => "".to_string(),
    };
    answer
}

fn parse_input() -> Vec<String> {
    DayData::from_file_path("./data/day_03/input.txt")
        .lines()
        .collect()
}

fn part1(report: &Vec<String>) -> isize {
    let report_size = report.len();
    let mut results: Vec<usize> = vec![0; report[0].len()];
    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();

    for line in report {
        for (i, b) in line.chars().enumerate() {
            if b == '1' {
                results[i] += 1;
            }
        }
    }

    for bit in results {
        if bit >= report_size / 2 {
            gamma += "1";
            epsilon += "0";
        } else {
            gamma += "0";
            epsilon += "1";
        }
    }

    let gamma_rate = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&epsilon, 2).unwrap();

    return gamma_rate * epsilon_rate;
}

fn part2(report: &Vec<String>) -> isize {
    let mut oxygen_ratings = report.clone();
    let mut co2_ratings = report.clone();

    for pos in 0..report[0].len() {
        oxygen_ratings = filter_by_bit(oxygen_ratings, pos, |ones_len, zero_len| {
            ones_len >= zero_len
        });

        co2_ratings = filter_by_bit(co2_ratings, pos, |ones_len, zero_len| {
            ones_len < zero_len && ones_len != 0
        });
    }

    let oxygen_rating = isize::from_str_radix(&oxygen_ratings[0], 2).unwrap();
    let co2_rating = isize::from_str_radix(&co2_ratings[0], 2).unwrap();

    return oxygen_rating * co2_rating;
}

fn filter_by_bit(records: Vec<String>, pos: usize, f: fn(usize, usize) -> bool) -> Vec<String> {
    if records.len() == 1 {
        return records;
    }

    let mut ones = vec![];
    let mut zeros = vec![];

    for line in records {
        if line.chars().nth(pos).unwrap() == '1' {
            ones.push(line);
        } else {
            zeros.push(line);
        }
    }

    if f(ones.len(), zeros.len()) {
        return ones;
    }

    return zeros;
}
