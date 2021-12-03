use crate::helpers::DayData;

struct Movement {
    direction: Direction,
    distance: i32,
}

enum Direction {
    Forward,
    Up,
    Down,
}

impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(format!("'{}' is not a valid value for Direction", s)),
        }
    }
}

pub fn run(part: &i32) -> String {
    let input = parse_input();
    let answer = match part {
        1 => part1(&input).to_string(),
        _ => "".to_string(),
    };
    answer
}

fn parse_input() -> Vec<Movement> {
    DayData::from_file_path("./data/day_02/input.txt")
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            Movement {
                direction: split
                    .next()
                    .unwrap()
                    .parse::<Direction>()
                    .expect("Direction must be one of: forward, up or down"),
                distance: split
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .expect("Distance must be an integer"),
            }
        })
        .collect()
}

fn part1(movements: &Vec<Movement>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for movement in movements {
        match movement.direction {
            Direction::Forward => {
                horizontal += movement.distance;
            }
            Direction::Up => {
                depth -= movement.distance;
            }
            Direction::Down => {
                depth += movement.distance;
            }
        }
    }

    horizontal * depth
}
