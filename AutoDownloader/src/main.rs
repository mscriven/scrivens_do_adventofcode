use std::collections::HashMap;
use std::{env, fs};
use std::path::Path;
use chrono::Datelike;
use reqwest::Client;

fn main() {
    println!("Hello, world!");
    let site = "https://adventofcode.com";
    let cookie =  match gcookie::get_cookies("firefox", site) {
        Ok(cookie) => cookie,
        Err(err) => panic!("An error occurred when get cookie '{}': {}", site, err),
    };

    let cookies_dict = get_cookies_dict(Some(&cookie));

    if let Some(value) = cookies_dict.get("session") {
        println!("{}", value);
    } else {
        println!("No session cookie found");
    }

    let today = chrono::Local::now(); // Get the current date
    let day_today = today.day();

    println!("Initialising day {}", day_today);
}

fn setup_day(day: u32, cookie: &str) -> Result<(), Box<dyn std::error::Error>> {
    let absolute_path = format!("{}/day{}", env::current_dir()?.display(), day);

    if !Path::new(&absolute_path).exists() {
        fs::create_dir(&absolute_path)?;
        env::set_current_dir(&absolute_path)?;

        let client = Client::new();
        let url = format!("https://adventofcode.com/2019/day/{}/input", day);
        let response = client.get(&url).header("Cookie", cookie).send()?.text()?;

        fs::write(format!("{}/input{}", absolute_path, day), response)?;
    }

    Ok(())
}

fn get_cookies_dict(cookies: Option<&str>) -> HashMap<String, String> {
    let mut cookie_map = HashMap::new();

    if let Some(cookies_str) = cookies {
        for cookie in cookies_str.split("; ") {
            if let Some((key, value)) = cookie.split_once('=') {
                cookie_map.insert(key.to_string(), value.to_string());
            }
        }
    }

    cookie_map
}