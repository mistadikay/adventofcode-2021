use crate::helpers::DayData;

pub fn run(part: &i32) -> String {
    let input = parse_input();
    let answer = match part {
        1 => part1(&input).to_string(), // 3847100
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
