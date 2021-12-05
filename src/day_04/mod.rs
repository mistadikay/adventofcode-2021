use crate::helpers::DayData;
use regex::Regex;

struct Input {
    numbers: Vec<i32>,
    boards: Vec<Board>,
}

#[derive(Clone)]
struct Board {
    horizontal: BoardNumbers,
    vertical: BoardNumbers,
}

type BoardNumbers = Vec<Vec<i32>>;

pub fn run(part: &i32) -> String {
    let input = parse_input();
    let answer = match part {
        1 => part1(&input).to_string(), // 54275
        2 => part2(&input).to_string(), //
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

    let boards: Vec<Board> = data.fold(vec![], |mut boards, datum| {
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

        boards.push(Board {
            vertical: rotate_board(&board),
            horizontal: board,
        });
        return boards;
    });

    Input { numbers, boards }
}

fn rotate_board(board: &BoardNumbers) -> BoardNumbers {
    let mut rotated_board: BoardNumbers = vec![vec![]; board[0].len()];
    for row in board {
        for (i, num) in row.iter().enumerate() {
            rotated_board[i].push(*num);
        }
    }
    return rotated_board;
}

fn check_nums(nums: &mut BoardNumbers, num: &i32) -> bool {
    for ri in 0..nums.len() {
        nums[ri].retain(|elem| elem != num);
        if nums[ri].len() == 0 {
            return true;
        }
    }

    return false;
}

fn part1(report: &Input) -> i32 {
    let mut winning_number: i32 = 0;
    let mut winning_board: BoardNumbers = vec![];
    let mut boards = report.boards.clone();
    'outer: for num in report.numbers.iter() {
        for bi in 0..boards.len() {
            if check_nums(&mut boards[bi].vertical, &num)
                || check_nums(&mut boards[bi].horizontal, &num)
            {
                winning_number = *num;
                winning_board = boards[bi].vertical.clone();
                break 'outer;
            }
        }
    }

    return winning_number * winning_board.into_iter().flatten().sum::<i32>();
}

fn part2(report: &Input) -> i32 {
    let mut last_winning_number: i32 = 0;
    let mut last_winning_board: BoardNumbers = vec![];
    let mut boards = report.boards.clone();
    let mut winning_board_idxs: Vec<usize> = vec![];
    'outer: for num in report.numbers.iter() {
        for bi in 0..boards.len() {
            if !winning_board_idxs.contains(&bi) {
                if check_nums(&mut boards[bi].vertical, &num)
                    || check_nums(&mut boards[bi].horizontal, &num)
                {
                    winning_board_idxs.push(bi);
                    if winning_board_idxs.len() == boards.len() {
                        last_winning_number = *num;
                        last_winning_board = boards[bi].vertical.clone();
                        break 'outer;
                    }
                }
            }
        }
    }

    return last_winning_number * last_winning_board.into_iter().flatten().sum::<i32>();
}
