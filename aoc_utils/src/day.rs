use anyhow::{anyhow, Context, Result};
use regex::Regex;
use reqwest::blocking::Client;
use rookie::{common::enums::CookieToString, firefox};
use std::{fs, path::PathBuf};

/// Represents a particular Advent of Code day
#[derive(Debug)]
struct Day {
    day: u32,
    year: u32,
    template_path: PathBuf,
    year_path: PathBuf,
    day_path: PathBuf,
    data_path: PathBuf,
}

impl Day {
    pub fn new(day: u32, year: u32) -> Result<Self> {
        let base_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).canonicalize()?;
        let year_path = base_path.join(format!("../aoc_{}", year)).canonicalize()?;
        let day_path = year_path.join("src").join(format!("day_{}", day));
        let data_path = day_path.join("data");
        let template_path = base_path.join("templates/rust.tmpl");

        Ok(Self {
            day,
            year,
            year_path,
            day_path,
            data_path,
            template_path,
        })
    }

    pub fn setup(&self) -> Result<()> {
        self.ensure_year_folder_exists()?;
        self.copy_template()?;
        self.fetch_and_save_input_files()?;
        self.update_lib_rs()?;
        println!(
            "Successfully created day {} for year {}",
            self.day, self.year
        );
        println!("Don't forget to check the `data/test-input.txt` file for the correct test input and update the macro with the correct test input");
        Ok(())
    }

    fn ensure_year_folder_exists(&self) -> Result<()> {
        if !self.year_path.exists() {
            Err(anyhow!(
                "Year {} folder does not exist at {:?}",
                self.year,
                self.year_path
            ))
        } else if !self.template_path.exists() {
            Err(anyhow!(
                "Template file not found at {:?}",
                self.template_path
            ))
        } else {
            fs::create_dir_all(self.day_path.clone())?;
            Ok(())
        }
    }

    fn fetch_and_save_input_files(&self) -> Result<()> {
        fs::create_dir_all(&self.data_path)?;
        let input_url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            self.year, self.day
        );
        let input_data = self.fetch_data(&input_url)?;
        fs::write(self.data_path.join("input.txt"), &input_data)?;

        let page_url = format!("https://adventofcode.com/{}/day/{}", self.year, self.day);
        let page_data = self.fetch_data(&page_url)?;
        let test_input = self.extract_test_input(&page_data)?;
        fs::write(self.data_path.join("test-input.txt"), test_input)?;

        Ok(())
    }

    fn copy_template(&self) -> Result<()> {
        let mod_file = self.day_path.join("mod.rs");
        fs::copy(&self.template_path, &mod_file).context("Failed copying template")?;
        Ok(())
    }

    fn update_lib_rs(&self) -> Result<()> {
        let lib_path = self.year_path.join("src/lib.rs");
        let content = fs::read_to_string(&lib_path).context("Failed to read lib.rs")?;
        let mod_line = format!("pub mod day_{};\n", self.day);

        if !content.contains(&mod_line) {
            let mut updated = content;
            updated.push_str(&mod_line);
            fs::write(&lib_path, updated).context("Failed to update lib.rs")?;
        }
        Ok(())
    }

    fn fetch_data(&self, url: &str) -> Result<String> {
        let client = Client::new();
        let base = url
            .strip_prefix("https://")
            .and_then(|u| u.split('/').next())
            .ok_or_else(|| anyhow!("Failed to determine base URL from {url}"))?;

        let cookies = firefox(Some(vec![base.into()])).map_err(|e| anyhow!(e))?;
        let resp = client
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36").header("Cookie", cookies.to_string())
            .send()?
            .text()?;

        if resp.contains("Please log in to get your puzzle input.") {
            Err(anyhow!("Please log in to get your puzzle input."))
        } else {
            Ok(resp)
        }
    }

    fn extract_test_input(&self, page_data: &str) -> Result<String> {
        let re = regex::Regex::new(r"<main>(?s).*</main>")?;
        let main = re.find(page_data).unwrap().as_str();
        let parsed_markdown = html2md::parse_html(main);
        let re = Regex::new(r"```\n([\s\S]*?)\n```")?;
        let mut blocks: Vec<&str> = re
            .captures_iter(&parsed_markdown)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str())
            .collect();

        blocks.sort_by_key(|a| a.len());
        blocks
            .last()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("No code blocks found for test input"))
    }
}

pub fn create_day(day: u32, year: u32) -> Result<()> {
    let day_instance = Day::new(day, year)?;
    day_instance.setup()
}

pub fn get_next_day(year: u32) -> Result<()> {
    let current_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).canonicalize()?;
    let current_day = fs::read_dir(current_dir.join(format!("../aoc_{}", year)).join("src"))?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.file_name()
                    .into_string()
                    .ok()
                    .and_then(|s| s.strip_prefix("day_").and_then(|d| d.parse().ok()))
            })
        })
        .max()
        .unwrap_or(0);

    println!("Highest existing day: {}", current_day);
    println!("Creating day {}", current_day + 1);
    create_day(current_day + 1, year)
}
