use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    assert_eq!(part1(&numbers), 1233);
    assert_eq!(part2(&numbers), 1275);
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
