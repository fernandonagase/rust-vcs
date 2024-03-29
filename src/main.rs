use std::env;

use vcs::run;

const EXECUTABLE_NAME: &str = "vcs";

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    run(&config.command, &config.arguments);
}

struct Config {
    command: String,
    arguments: Vec<String>,
}

impl Config {
    fn new(args: &[String]) -> Self {
        if args.len() < 2 {
            panic!("Uso do comando: {EXECUTABLE_NAME} comando <argumentos>?");
        }
        let command = args[1].clone();
        let arguments: Vec<String> = args[2..].iter().cloned().collect();

        if command == "commit" && arguments.len() != 1 {
            panic!("O comando commit exige um argumento: descrição");
        } else if command == "restore" && arguments.len() != 1 {
            panic!("O comando restore exige um argumento: versão");
        }

        Self { command, arguments }
    }
}
