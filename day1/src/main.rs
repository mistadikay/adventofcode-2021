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

    println!("Answer for part 1 is {}", part1(&numbers));
    println!("Answer for part 2 is {}", part2(&numbers));
}

fn part1(numbers: &Vec<i64>) -> i64 {
    let mut count = 0;

    for (i, v) in numbers.iter().enumerate() {
        // print!("{} ", v);
        if i == 0 {
            // println!("(N/A - no previous measurement)")
        } else {
            if v > &numbers[i - 1] {
                // println!("(increased)");
                count += 1;
            } else {
                // println!("(decreased)")
            }
        }
    }

    return count;
}

fn part2(numbers: &Vec<i64>) -> i64 {
    let mut count = 0;

    for (i, v) in numbers.iter().enumerate() {
        if i == numbers.len() - 2 {
            break;
        }

        let value = v + &numbers[i + 1] + &numbers[i + 2];
        // print!("{} ", value);
        if i == 0 {
            // println!("(N/A - no previous sum)")
        } else {
            let prev = &numbers[i - 1] + v + &numbers[i + 1];
            if value > prev {
                // println!("(increased)");
                count += 1;
            } else if value < prev {
                // println!("(decreased)")
            } else {
                // println!("(no change)")
            }
        }
    }

    return count;
}
