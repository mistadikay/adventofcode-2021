use crate::helpers::DayData;
use regex::Regex;

struct Input {
    numbers: Vec<i32>,
    boards: Vec<Board>,
}

type Board = Vec<Vec<i32>>;

pub fn run(part: &i32) -> String {
    let input = parse_input();
    let answer = match part {
        1 => part1(&input).to_string(), // 3847100
        2 => part2(&input).to_string(), // 4105235
        _ => "".to_string(),
    };
    answer
}

fn parse_input() -> Input {
    let input = DayData::from_file_path("./data/day_04/input.txt").to_string();
    let re_newline = Regex::new(r"(\n\n)").expect("Invalid regex");
    let mut data = re_newline.split(&input).map(|s| s.to_string()).into_iter();

    let numbers = data
        .next()
        .expect("unexpected error parsing numbers")
        .clone()
        .split(",")
        .map(|n| n.parse::<i32>().expect("numbers parsing error"))
        .collect();

    let mut boards: Vec<Board> = data.fold(vec![], |mut boards, datum| {
        let board = datum
            .trim()
            .split('\n')
            .map(|s| {
                s.split_whitespace()
                    .map(|s| s.trim().parse::<i32>().unwrap())
                    .into_iter()
                    .collect()
            })
            .collect();

        boards.push(rotate_board(&board));
        boards.push(board);
        return boards;
    });

    Input { numbers, boards }
}

fn rotate_board(board: &Board) -> Board {
    let mut rotated_board: Board = vec![vec![]; board[0].len()];
    for row in board {
        for (i, num) in row.iter().enumerate() {
            rotated_board[i].push(*num);
        }
    }
    return rotated_board;
}

fn part1(report: &Input) -> i32 {
    let mut winning_number: i32 = 0;
    let mut winning_board: Board = vec![];
    let mut boards = report.boards.clone();
    'outer: for num in report.numbers.iter() {
        for bi in 0..boards.len() {
            for ri in 0..boards[bi].len() {
                boards[bi][ri].retain(|elem| elem != num);
                if boards[bi][ri].len() == 0 {
                    winning_number = *num;
                    winning_board = boards[bi].clone();
                    break 'outer;
                }
            }
        }
    }

    return winning_number * winning_board.into_iter().flatten().sum::<i32>();
}

fn part2(report: &Input) -> isize {
    0
}
