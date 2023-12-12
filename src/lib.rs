use crate::repository::{init, commit};

pub mod repository;

pub fn run(command: &str) {
    println!("Seu comando foi: {}", command);
    if command == "init" {
        init();
    } else if command == "commit" {
        commit();
    }
}