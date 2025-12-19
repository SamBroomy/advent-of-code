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
        self.fetch_and_save_input_files(true)?;
        self.update_lib_rs()?;
        println!(
            "Successfully created day {} for year {}",
            self.day, self.year
        );
        println!("Don't forget to double check the `data/sample-input.txt` file for the correct sample input and update the macro with the correct sample answer!");
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

    fn fetch_and_save_input_files(&self, force: bool) -> Result<()> {
        fs::create_dir_all(&self.data_path)
            .with_context(|| format!("failed to create data dir {:?}", &self.data_path))?;

        let input_path = self.data_path.join("input.txt");
        if input_path.exists() && !force {
            println!(
                "Input file already exists at {:?}. Not overwriting!",
                input_path
            );
        } else {
            let input_url = format!(
                "https://adventofcode.com/{}/day/{}/input",
                self.year, self.day
            );
            let input_data = self.fetch_data(&input_url)?;
            fs::write(&input_path, &input_data)
                .with_context(|| format!("failed to write input file {:?}", input_path))?;
        }

        let sample_path = self.data_path.join("sample-input.txt");

        let md_path = self.day_path.join("aoc.md");
        let page_url = format!("https://adventofcode.com/{}/day/{}", self.year, self.day);

        let mut fetch = !sample_path.exists() || force;

        if md_path.exists() {
            let existing = fs::read_to_string(&md_path)
                .with_context(|| format!("failed to read existing {:?}", md_path))?;
            if !existing.contains("--- Part Two ---") {
                println!(
                    "Existing markdown at {:?} seems incomplete. Part Two missing. Refetching.",
                    md_path
                );
                fetch = true;
            }
        }

        let sample_input = if fetch {
            let page_data = self.fetch_data(&page_url)?;
            let main_html = self
                .extract_html_body(&page_data)
                .ok_or_else(|| anyhow!("Failed to extract <main> from page HTML"))?;
            let md_page = self.parsed_page_to_markdown(main_html)?;
            let sample = self
                .extract_sample_input(main_html)
                .ok_or_else(|| anyhow!("Failed to extract sample input from page HTML"))?
                .to_string();
            fs::write(&md_path, &md_page)
                .with_context(|| format!("failed to write {:?}", md_path))?;
            sample
        } else {
            let existing = fs::read_to_string(&md_path)
                .with_context(|| format!("failed to read existing {:?}", md_path))?;
            self.extract_test_input_from_markdown(&existing)?
        };

        let sample_path = self.data_path.join("sample-input.txt");
        if sample_path.exists() && !force {
            println!("Test input file already exists. Not overwriting!");
        } else {
            fs::write(&sample_path, &sample_input)
                .with_context(|| format!("failed to write {:?}", sample_path))?;
        }
        Ok(())
    }

    fn copy_template(&self) -> Result<()> {
        let mod_file = self.day_path.join("mod.rs");
        if mod_file.exists() {
            println!("File already exists for day {}. Not overwriting!", self.day);
        } else {
            fs::copy(&self.template_path, &mod_file).context("Failed copying template")?;
        }
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
        } else {
            println!(
                "lib.rs already contains mod line for day {}. Not updating!",
                self.day
            );
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
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko)",
            )
            .header("Cookie", cookies.to_string())
            .send()
            .with_context(|| format!("HTTP GET {} failed", url))?
            .text()
            .with_context(|| format!("reading response body from {} failed", url))?;

        if resp.contains("Please log in to get your puzzle input.") {
            Err(anyhow!("Please log in to get your puzzle input."))
        } else {
            Ok(resp)
        }
    }

    fn extract_html_body<'a>(&self, page_data: &'a str) -> Option<&'a str> {
        Regex::new(r"(?s)<main>.*?</main>")
            .expect("valid regex")
            .find(page_data)
            .map(|m| m.as_str())
    }

    fn parsed_page_to_markdown(&self, main_html: &str) -> Result<String> {
        let article_re = Regex::new(r"(?s)<article\b[^>]*>.*?</article>").expect("valid regex");
        // remove <code> inside <pre> so html2md only emits a single ``` block
        let pre_code_re =
            Regex::new(r"(?is)(<pre\b[^>]*>)\s*<code\b[^>]*>(?P<body>.*?)</code>\s*(</pre>)")
                .expect("valid regex");
        let articles: Vec<String> = article_re
            .find_iter(main_html)
            .map(|m| {
                let html = m.as_str().to_string();

                let cleaned = pre_code_re
                    .replace_all(&html, |caps: &regex::Captures| {
                        let body = html_escape::decode_html_entities(&caps["body"]);
                        format!("{}{}{}", &caps[1], body, &caps[3])
                    })
                    .into_owned();
                html2md::rewrite_html(&cleaned, true)
            })
            .collect();
        if articles.is_empty() {
            return Err(anyhow!("No <article> blocks found in page HTML"));
        }
        let header = format!(
            "# [Day {} — Advent of Code {}](https://adventofcode.com/{}/day/{})",
            self.day, self.year, self.year, self.day
        );

        Ok(format!("{}\n\n{}", header, articles.join("\n\n\n")))
    }

    fn extract_sample_input<'a>(&self, main_html: &'a str) -> Option<&'a str> {
        let code_re = Regex::new(r"(?is)<pre\b[^>]*>\s*<code\b[^>]*>(?P<body>.*?)</code>\s*</pre>")
            .expect("valid regex");
        code_re
            .captures(main_html)
            .and_then(|caps| caps.name("body"))
            .map(|m| m.as_str())
    }

    fn extract_test_input_from_markdown(&self, parsed_markdown: &str) -> Result<String> {
        let re = Regex::new(r"```(?:\w*\n)?([\s\S]*?)```")?;
        let mut blocks: Vec<&str> = re
            .captures_iter(parsed_markdown)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str())
            .collect();

        blocks.sort_by_key(|a| a.len());
        blocks
            .last()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("No code blocks found in existing markdown"))
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

pub fn refresh_inputs(year: u32, force: bool) -> Result<()> {
    let current_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).canonicalize()?;
    let mut days = fs::read_dir(current_dir.join(format!("../aoc_{}", year)).join("src"))?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.file_name()
                    .into_string()
                    .ok()
                    .and_then(|s| s.strip_prefix("day_").and_then(|d| d.parse().ok()))
            })
        })
        .collect::<Vec<u32>>();

    days.sort();

    for day in days {
        println!("Refreshing day {}", day);
        let day_instance = Day::new(day, year)?;
        day_instance.fetch_and_save_input_files(force)?;
    }
    Ok(())
}
