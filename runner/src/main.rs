use dotenv::dotenv;
use std::{
    env,
    io::{self, BufRead},
};

use crate::process::CargoProcess;

mod process;

fn main() {
    dotenv().ok();
    let db = env::var("DB").expect("DB must be set");
    let back = env::var("BACK_PORT").expect("BACK_PORT must be set");
    let front = env::var("FRONT_PORT").expect("FRONT_PORT must be set");

    let mut back_process = CargoProcess::new(
        "web",
        true,
        &[("db_url", db.as_str()), ("port", back.as_str())],
    );
    let mut front_process = CargoProcess::new(
        "web",
        true,
        &[("port", front.as_str()), ("back_port", back.as_str())],
    );

    back_process.start().expect("Failed to start back process");
    front_process
        .start()
        .expect("Failed to start front process");

    println!("DB: {}", db);
    println!("BACK_PORT: {}", back);
    println!("FRONT_PORT: http://localhost:{}", front);
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line.unwrap().as_str() {
            "q" => {
                back_process.stop().unwrap();
                front_process.stop().unwrap();
                break;
            }
            "r" => {
                back_process.stop().unwrap();
                front_process.stop().unwrap();
                back_process.start().unwrap();
                front_process.start().unwrap();
            }
            _ => {
                println!("Unknown command");
                println!("q - quit");
                println!("r - restart");
            }
        }
    }
}
