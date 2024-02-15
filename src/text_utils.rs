use regex::Regex;
use url::Url;
pub fn return_valid_line(line: &str, email_regex: &Regex, login_regex: &Regex, number_regex: &Regex, charses: Vec<&str>, bad_word: Vec<&str>) -> Option<(String, i32)>  {
    let mut line_temp = line.trim().to_string();
    if line_temp.to_lowercase().starts_with("android://") {
        return Some((line.to_string(), 1));
    }
    if line.to_lowercase().contains(":unknown:") {
        return Some((line.to_string(), 2));
    }
    for char in charses.iter() {
        line_temp = line_temp.replace(char, "");
    }
    for bad in bad_word.iter() {
        if line_temp.to_lowercase().contains(bad) { return Some((line.to_string(), 3)); }
    }
    if line.matches(":").count() < 2 { return Some((line.to_string(), 3));}
    if !line_temp.starts_with("http") { line_temp = format!("https://{}", line_temp) }
    let parts: Vec<&str> = line_temp.split(":").collect();
    let cred = parts[parts.len() - 2..].to_vec();
    let mut url = line_temp.replace(&format!(":{}:{}", cred[0], cred[1]), "");
    url = match url.parse::<Url>() {
        Ok(url) => {
            match url.host() {
                Some(url) => url.to_string(),
                None => return Some((line.to_string(), 3))
            }
        },
        Err(_) => return Some((line.to_string(), 3)),
    };
    if check_print(cred[1]) {
        if email_regex.is_match(cred[0]) { return Some((format!("https://{}:{}:{}", url, cred[0], cred[1]), 0)); }
        if login_regex.is_match(cred[0]) { return Some((format!("https://{}:{}:{}", url, cred[0], cred[1]), 0)); }
        if number_regex.is_match(cred[0]) { return Some((format!("https://{}:{}:{}", url, cred[0], cred[1]), 0)); }
    } return Some((line.to_string(), 3));
}

fn check_print(line: &str) -> bool {
    line.chars().all(|c| c.is_ascii() && c.is_ascii_graphic())
}