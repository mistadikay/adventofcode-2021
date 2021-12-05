mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod helpers;

use std::env;

pub trait AdventDay {}

pub fn run(config: Config) -> Result<String, &'static str> {
    let day = match config.day {
        1 => Ok(day_01::run(&config.part)),
        2 => Ok(day_02::run(&config.part)),
        3 => Ok(day_03::run(&config.part)),
        4 => Ok(day_04::run(&config.part)),
        _ => Err("That day has not been done"),
    };
    day
}

pub struct Config {
    pub day: i32,
    pub part: i32,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let day = match args.next() {
            Some(arg) => match arg.parse::<i32>() {
                Ok(arg) => match arg {
                    arg if (1..26).contains(&arg) => arg,
                    _ => return Err("There are only 25 days"),
                },
                Err(_) => return Err("Day must be a number"),
            },
            None => return Err("Didn't get a day"),
        };

        let part = match args.next() {
            Some(arg) => match arg.parse::<i32>() {
                Ok(arg) => match arg {
                    arg if (1..3).contains(&arg) => arg,
                    _ => return Err("There are only 2 parts"),
                },
                Err(_) => return Err("Part must be a number"),
            },
            None => return Err("Didn't get a part"),
        };

        Ok(Config { day, part })
    }
}
