use crate::helpers::DayData;
use std::error;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_straight_line(&self) -> bool {
        return self.start.x == self.end.x || self.start.y == self.end.y;
    }

    fn to_points(&self) -> Vec<Point> {
        if self.start.x == self.end.x {
            let x = self.start.x;
            let (start, end) = match self.start.y > self.end.y {
                true => (self.end.y, self.start.y),
                false => (self.start.y, self.end.y),
            };
            return (start..end + 1)
                .into_iter()
                .map(|y| Point { x, y })
                .collect();
        }

        let y = self.start.y;
        let (start, end) = match self.start.x > self.end.x {
            true => (self.end.x, self.start.x),
            false => (self.start.x, self.end.x),
        };
        return (start..end + 1)
            .into_iter()
            .map(|x| Point { x, y })
            .collect();
    }
}

impl FromStr for Line {
    type Err = Box<dyn error::Error>;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut points = raw
            .split(" -> ")
            .map(|point| {
                let coords: Vec<i32> = point
                    .split(",")
                    .map(|s| s.parse::<i32>().expect("error parsing input"))
                    .into_iter()
                    .collect();

                Point {
                    x: coords[0],
                    y: coords[1],
                }
            })
            .into_iter();

        return Ok(Line {
            start: points.next().unwrap(),
            end: points.next().unwrap(),
        });
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn equals(&self, point: &Point) -> bool {
        self.x == point.x && self.y == point.y
    }
}

pub fn run(part: &i32) -> String {
    let input = parse_input(
        DayData::from_file_path("./data/day_05/input.txt")
            .lines()
            .collect(),
    );
    let answer = match part {
        1 => part1(input).to_string(), // 5608
        2 => part2(input).to_string(), //
        _ => "".to_string(),
    };
    answer
}

fn parse_input(iter: Vec<String>) -> Vec<Line> {
    iter.into_iter()
        .map(|s| s.parse::<Line>().unwrap())
        .into_iter()
        .collect()
}

fn lines_to_points(lines: &Vec<Line>) -> Vec<Point> {
    let mut points: Vec<Point> = lines
        .into_iter()
        .filter(|line| line.is_straight_line())
        .map(|line| line.to_points())
        .flatten()
        .collect();

    points.sort();

    return points;
}

fn part1(report: Vec<Line>) -> i32 {
    let mut points = lines_to_points(&report).into_iter();
    let mut prev_point: Point = points.next().unwrap();
    let mut is_prev_duplicate = false;

    return points.fold(0, |count, point| {
        if point.equals(&prev_point) {
            if !is_prev_duplicate {
                is_prev_duplicate = true;
                return count + 1;
            }
        } else {
            is_prev_duplicate = false
        }

        prev_point = point;

        return count;
    });
}

fn part2(_report: Vec<Line>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_line() {
        assert_eq!(
            "973,543 -> 601,915".parse::<Line>().unwrap(),
            Line {
                start: Point { x: 973, y: 543 },
                end: Point { x: 601, y: 915 }
            }
        )
    }

    #[test]
    fn can_recognize_straight_line() {
        let diagonal_line = "973,543 -> 601,915".parse::<Line>().unwrap();
        let straight_line = "0,9 -> 5,9".parse::<Line>().unwrap();
        assert_eq!(diagonal_line.is_straight_line(), false);
        assert_eq!(straight_line.is_straight_line(), true);
    }

    #[test]
    fn can_lines_to_points() {
        let input = parse_input(
            vec!["3,4 -> 1,4", "0,0 -> 8,8", "2,2 -> 2,1", "6,4 -> 2,0"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        );
        let mut actual: Vec<Point> = lines_to_points(&input);
        let expected: Vec<Point> = vec![
            Point { x: 1, y: 4 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 },
            Point { x: 2, y: 4 },
            Point { x: 3, y: 4 },
        ];

        actual.sort();
        assert_eq!(actual, expected)
    }

    #[test]
    fn part1_simple() {
        let input = parse_input(
            vec![
                "0,9 -> 5,9",
                "8,0 -> 0,8",
                "9,4 -> 3,4",
                "2,2 -> 2,1",
                "7,0 -> 7,4",
                "6,4 -> 2,0",
                "0,9 -> 2,9",
                "3,4 -> 1,4",
                "0,0 -> 8,8",
                "5,5 -> 8,2",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
        );

        assert_eq!(part1(input), 5)
    }
}
