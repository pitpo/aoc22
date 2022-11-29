extern crate reqwest;

use reqwest::header::COOKIE;
use std::env;
use std::fs;
use std::path::Path;

fn fetch_input_for_day(year: &u32, day: &u32, session: &str) -> String {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = reqwest::Client::new();

    let tokio_runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    
    let input = tokio_runtime.block_on(async move { client
        .get(&url)
        .header(COOKIE, format!("session={}", session))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap() 
    });
    if input.contains("Not Found") || input.contains("Please don't repeatedly request this endpoint before it unlocks!") {
        panic!("Puzzle for day {} is not live yet", day);
    }
    if input.contains("log in") {
        panic!("Session cookie is invalid");
    }
    input
}

fn save_input(input_path: &str, input: &str) {
    let input_dir = input_path.clone().split("/").nth(0).unwrap_or(".");
    if !Path::new(input_dir).is_dir() {
        fs::create_dir(input_dir).expect("Can't create directory for puzzle input files");
    }
    fs::write(input_path, &input).expect("Can't create puzzle input file");
}

pub fn get_input_for_day(year: u32, day: u32) -> String {
    let session = env::vars()
        .find(|var| var.0 == "AOC_SESSION")
        .expect("Please put your session cookie in AOC_SESSION environmental variable")
        .1;
    let input_path = format!("input/day{}.txt", day);
    match fs::read_to_string(&input_path) {
        Ok(input) => input,
        Err(_) => {
            println!(
                "Puzzle input file doesn't exist, fetching it and saving in \"{}\"",
                input_path
            );
            let input = fetch_input_for_day(&year, &day, &session);
            save_input(&input_path, &input);
            input
        }
    }
}
