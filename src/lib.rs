use crate::repository::{init, commit, restore};

pub mod repository;

pub fn run(command: &str, arguments: &[String]) {
    println!("Seu comando foi: {}", command);
    if command == "init" {
        init();
    } else if command == "commit" {
        commit();
    } else if command == "restore" {
        let version = match arguments[0].parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Número de versão inválido!");
                return;
            }
        };
        restore(version);
    }
}