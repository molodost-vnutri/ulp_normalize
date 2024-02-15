use std::fs::OpenOptions;
use std::io::Write;
use std::fs;

mod text_utils;
mod file_utils;
mod utils;

fn main() {
    let mut log: Vec<String> = Vec::new();
    utils::clear_screen();
    if let Some((stat, path)) = file_utils::return_path() {
        match stat {
            true => {
                file_utils::start(&path);
                log.push(path.to_string())
            },
            false => {
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            if path.is_file() {
                                file_utils::start(path.to_str().unwrap());
                                log.push(path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .unwrap();
    for line in log.iter() {
        let _ = log_file.write(format!("Обработал файл: {}\n", line).as_bytes());
    }
    println!("Завершил обработку");
    let _ = std::io::stdin().read_line(&mut String::new()).unwrap();
}
