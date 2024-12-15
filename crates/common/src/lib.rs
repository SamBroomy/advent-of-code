use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use rookie::{common::enums::CookieToString, firefox};
use std::fs;
use std::path::Path;

fn get_data(url: &str) -> Result<String> {
    let client = Client::new();

    let base_url = url
        .strip_prefix("https://")
        .and_then(|url| url.split('/').next())
        .ok_or_else(|| anyhow!("Failed to get base url"))?;

    let cookies = firefox(Some(vec![base_url.into()])).map_err(|e| anyhow!(e))?;

    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36")
        .header("Cookie", cookies.to_string())
        .send()?
        .text()?;

    let invalid_response = "Please log in to get your puzzle input.";

    if response.contains(invalid_response) {
        return Err(anyhow!("Please log in to get your puzzle input."));
    }

    assert!(!response.contains("Please log in to get your puzzle input."));
    Ok(response)
}

pub fn get_input(day: u32) -> Result<String> {
    // Get the manifest directory at compile time
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    // navigate to 'advent-of-code/crates/day-N'
    let data_dir = Path::new(manifest_dir)
        .parent() // 'advent-of-code/crates'
        .expect("Failed to get parent directory")
        .join(format!("day-{}/data", day));
    // Create the 'data' directory if it doesn't exist

    let input_file = data_dir.join("input.txt");
    if input_file.try_exists()? {
        return Ok(fs::read_to_string(input_file)?);
    }
    println!("Downloading input for day '{}'!", day);

    let url = format!("https://adventofcode.com/2024/day/{}/input", day);

    let input_data = get_data(&url)?;

    fs::create_dir_all(&data_dir)?;
    fs::write(&input_file, &input_data)?;

    Ok(input_data)
}
