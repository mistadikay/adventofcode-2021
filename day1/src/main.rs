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
