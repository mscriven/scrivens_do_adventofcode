mod problem;
mod days;
use chrono::Datelike;
use reqwest::Client;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let site = "https://adventofcode.com";
    let cookie = match gcookie::get_cookies("firefox", site) {
        Ok(cookie) => cookie,
        Err(err) => panic!("An error occurred when get cookie '{}': {}", site, err),
    };

    let cookies_dict = get_cookies_dict(Some(&cookie));

    if cookies_dict.get("session").is_some() {
        let today = chrono::Local::now();
        let day_today = today.day();

        println!("Initialising day {}", day_today);

        match setup_day(day_today, cookie).await {
            Ok(_) => println!("Day setup completed successfully."),
            Err(e) => eprintln!("An error occurred while setting up the day: {}", e),
        }
    } else {
        println!("No session cookie found");
    }
    Ok(())
}

async fn setup_day(day: u32, cookie: String) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let absolute_path = format!("{}/day{}", current_dir.display(), day);

    if !Path::new(&absolute_path).exists() {
        fs::create_dir(&absolute_path).await?;
        env::set_current_dir(&absolute_path)?;

        let client = Client::new();
        let url = format!("https://adventofcode.com/2024/day/{}/input", day);
        let response = client.get(&url).header("Cookie", cookie)
            .send().await?
            .text().await?;

        fs::write(format!("{}/input{}", absolute_path, day), response).await?;
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