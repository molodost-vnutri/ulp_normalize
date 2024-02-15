use std::{
    fs::{File, OpenOptions},
    io::{
        BufRead,
        BufReader, Write
    },
    path::Path,
};
use colored::Colorize;

use regex::Regex;

use crate::utils::clear_screen;

use super::text_utils;


pub fn start(file: &str) {
    let mut good: Vec<String> = Vec::new();
    let mut android: Vec<String> = Vec::new();
    let mut unknown: Vec<String> = Vec::new();
    let mut bad: Vec<String> = Vec::new();

    let mut good_result: usize = 0;
    let mut bad_result: usize = 0;
    let mut unknown_result: usize = 0;
    let mut android_result: usize = 0;

    let email_regex = Regex::new(r"^\S+@\S+\.\S+$").unwrap();
    let login_regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$").unwrap();
    let number_regex = Regex::new(r"^\+?\d{1,4}?[-.\s]?\(?\d{1,3}?\)?[-.\s]?\d{1,4}[-.\s]?\d{1,4}[-.\s]?\d{1,9}$").unwrap();
    let bad_word = vec!["unknown", "null", "none"];
    let charses = vec![";", " |", "| ", "|", " ", ","];

    let file_ = File::open(file).unwrap();
    let reader = BufReader::new(file_);
    for line in std::io::BufReader::lines(reader) {
        if let Some((stat, good_, bad_, unknown_, android_)) = write(&good, &bad, &unknown, &android) {
            match stat {
                true => {
                    good_result += good_;
                    bad_result += bad_;
                    unknown_result += unknown_;
                    android_result += android_;
                    good.clear();
                    bad.clear();
                    unknown.clear();
                    android.clear();
                    clear_screen();
                    println!("С файла: {}\n    Хороших строк: {}\n    Плохих строк : {}\n    Unknown строк: {}\n    Android строк: {}", file, good_result.to_string().green(), bad_result.to_string().red(), unknown_result.to_string().yellow(), android_result.to_string().yellow());
                
                }
                _ => {}
            }
        }
        let line = line.unwrap();
        if let Some((url, stat)) = text_utils::return_valid_line(&line, &email_regex, &login_regex, &number_regex, charses.clone(), bad_word.clone()) {
            match stat {
                0 => good.push(url),
                1 => android.push(url),
                2 => unknown.push(url),
                3 => bad.push(url),
                _ => {}
            }
        }
    }
}

fn write(good: &Vec<String>, bad: &Vec<String>, unknown: &Vec<String>, android: &Vec<String>) -> Option<(bool, usize, usize, usize, usize)> {
    let good_len = good.len();
    let android_len = android.len();
    let unknonw_len = unknown.len();
    let bad_len = bad.len();
    if good_len + android_len + unknonw_len + bad_len < 100000 { return Some((false, 0, 0, 0, 0)); }
    let mut good_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("good.txt")
        .unwrap();
    let mut bad_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("bad.txt")
        .unwrap();
    let mut unknown_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("unknown.txt")
        .unwrap();
    let mut android_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("android.txt")
        .unwrap();
    for line in good.iter() {
        let _ = good_file.write(format!("{}\n", line).as_bytes());
    }
    for line in bad.iter() {
        let _ = bad_file.write(format!("{}\n", line).as_bytes());
    }
    for line in android.iter() {
        let _ = android_file.write(format!("{}\n", line).as_bytes());
    }
    for line in unknown.iter() {
        let _ = unknown_file.write(format!("{}\n", line).as_bytes());
    }
    return Some((true, good_len, bad_len, unknonw_len, android_len))
}


pub fn return_path() -> Option<(bool, String)> {
    loop {
        let mut path = String::new();
        clear_screen();
        print!("[Path]=> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut path).unwrap();

        path = path.trim().replace("& '", "").replace("'", "");
        path.retain(|c| c != '"');

        let link_path = Path::new(&path);
        
        if link_path.exists() {
            if link_path.is_dir() {
                return Some((false, path));
            } else if link_path.is_file() {
                return Some((true, path));
            } 
        } else { 
            println!("Путь {} не найден", path.trim());
            let _ = std::io::stdin().read_line(&mut String::new());
        }
    }
}
