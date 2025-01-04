use crate::read::Record;
use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::{collections::BTreeMap, path::Path};

const README_TEMPLATE: &str = include_str!("readme.tmpl");

pub fn write_to_csv(csv_file: &Path, data: &BTreeMap<(u32, u32), Record>) -> anyhow::Result<()> {
    std::fs::create_dir_all(csv_file.parent().unwrap())?;
    let mut wtr = csv::Writer::from_path(csv_file)?;
    for record in data.values() {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn write_to_readme(data: &BTreeMap<(u32, u32), Record>) -> anyhow::Result<()> {
    let pattern = "aoc_*";
    let years = glob::glob(pattern)?
        .filter_map(|entry| entry.ok())
        .filter_map(|path| scan_fmt!(&path.file_name()?.to_str()?, "aoc_{d}", u32).ok())
        .sorted()
        .rev();
    dbg!(&years);

    let mut output = String::new();

    for year in years {
        output.push_str(&markdown_for_year(data, year));
    }

    let readme = README_TEMPLATE.replace("{{table}}", &output);
    std::fs::write("readme.md", readme)?;

    Ok(())
}

fn markdown_for_year(data: &BTreeMap<(u32, u32), Record>, required_year: u32) -> String {
    let mut output = format!(
        "### {required_year}\n\n| Day  | Problem     | Solution    | Part 1 (ms) | Part 2 (ms) | Total (ms) |\n",
    );
    output.push_str(
        "|------|-------------|-------------|-------------|-------------|------------|\n",
    );
    let mut part_one_total = 0.0;
    let mut part_two_total = 0.0;
    let mut total_total = 0.0;
    for ((year, day), record) in data.iter().filter(|((year, _), _)| *year == required_year) {
        let url = format!("https://adventofcode.com/{}/day/{}", year, day);
        let problem_name =
            get_problem_name(*year, *day).unwrap_or_else(|| "Unknown problem name".to_string());
        let solution_url = format!("/aoc_{year}/src/day_{day}/mod.rs");
        output.push_str(&format!(
            "| {day} | [{problem_name}]({url}) | [Solution]({solution_url}) | {:.2} | {:.2} | {:.2} |\n",
            record.part_one_millis, record.part_two_millis, record.total
        ));
        part_one_total += record.part_one_millis;
        part_two_total += record.part_two_millis;
        total_total += record.total;
    }

    output.push_str(&format!(
        "|  |  | Total | {:.2}ms | {:.2}ms | {:.2}ms |\n\n",
        part_one_total, part_two_total, total_total
    ));

    output
}

fn get_problem_name(year: u32, day: u32) -> Option<String> {
    let path = format!("aoc_{year}/src/day_{day}/data/aoc.md");
    let content = std::fs::read_to_string(path).unwrap();
    let re = regex::Regex::new(r"-- Day [0-9]+: (.*) --").unwrap();
    let problem_name = re.captures(&content)?.get(1)?.as_str().to_string();
    Some(problem_name)
}
