use crate::helpers::DayData;

const ADULT_TIMER: i32 = 7;
const CHILD_TIMER: i32 = 9;

#[derive(Debug, PartialEq, Clone)]
struct Fish {
    timer: i32,
}

impl Fish {
    fn count_generations(&self, days: i32) -> i32 {
        let mut children: Vec<Fish> = vec![];
        let mut timer = self.timer;

        while timer < days {
            children.push(Fish {
                timer: CHILD_TIMER + timer,
            });
            timer = timer + ADULT_TIMER;
        }

        children.len() as i32
            + children
                .into_iter()
                .fold(0, |count, fish| count + fish.count_generations(days))
    }
}

pub fn run(part: &i32) -> String {
    let input = parse_input(
        DayData::from_file_path("./data/day_06/input.txt")
            .lines()
            .next()
            .unwrap(),
    );
    let answer = match part {
        1 => part1(&input, 80).to_string(), // 349549
        2 => part2(&input).to_string(),     //
        _ => "".to_string(),
    };
    answer
}

fn parse_input(input: String) -> Vec<Fish> {
    input
        .split(",")
        .map(|s| Fish {
            timer: s.parse::<i32>().unwrap(),
        })
        .into_iter()
        .collect()
}

fn part1(fishes: &Vec<Fish>, days: i32) -> i32 {
    fishes.len() as i32
        + fishes
            .into_iter()
            .fold(0, |count, fish| count + fish.count_generations(days))
}

fn part2(fishes: &Vec<Fish>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_input() {
        let input = parse_input("3,4,3,1,2".to_string());

        assert_eq!(
            input,
            vec![
                Fish { timer: 3 },
                Fish { timer: 4 },
                Fish { timer: 3 },
                Fish { timer: 1 },
                Fish { timer: 2 },
            ]
        )
    }

    #[test]
    fn part1_simple() {
        let input = parse_input("3,4,3,1,2".to_string());

        assert_eq!(part1(&input, 18), 26);
        assert_eq!(part1(&input, 80), 5934);
    }
}
