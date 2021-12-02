use adventofcode2021::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match adventofcode2021::run(config) {
        Ok(result) => {
            println!("{}", result);
        }
        Err(e) => {
            eprintln!("Application error: {}", e);

            process::exit(1);
        }
    }
}
