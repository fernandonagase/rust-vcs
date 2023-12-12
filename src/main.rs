use std::env;

use vcs::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    run(&config.command);
}

struct Config {
    command: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let command = args[1].clone();

        Config { command }
    }
}