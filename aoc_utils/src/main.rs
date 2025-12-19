use anyhow::Result;
use clap::{Parser, Subcommand};
use std::env;

mod challenge_day;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Fetch puzzle input and description
    Fetch(FetchArgs),
    Next(YearArgs),
    Refresh(RefreshArgs),
}

#[derive(Parser, Debug)]
pub(crate) struct FetchArgs {
    /// Day (1-25) of the advent calendar
    #[arg(short, long, value_parser=clap::value_parser!(u32).range(1..=25))]
    day: u32,
    /// Challenge year
    #[arg(short, long, default_value_t = 0)]
    year: u32,
}

#[derive(Parser, Debug)]
pub(crate) struct YearArgs {
    // Next day so need year
    #[arg(short, long, default_value_t = 0)]
    year: u32,
}

#[derive(Parser, Debug)]
pub(crate) struct RefreshArgs {
    /// Challenge year
    #[arg(short, long, default_value_t = 0)]
    year: u32,

    /// Force refresh of all inputs, even if they exist
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Create a new folder for the day, create a lib.rs with the template. Add a line for the mod day to the src/lib.rs
    match args.command {
        Commands::Fetch(fetch_args) => {
            let year = pick_year(fetch_args.year)?;

            challenge_day::create_day(fetch_args.day, year)?;
        }
        Commands::Next(next_args) => {
            let year = pick_year(next_args.year)?;
            challenge_day::get_next_day(year)?;
        }
        Commands::Refresh(refresh_args) => {
            let year = pick_year(refresh_args.year)?;

            challenge_day::refresh_inputs(year, refresh_args.force)?;
        }
    }

    Ok(())
}
/// Converts a possibly-zero year into a valid final year, respecting constraints
fn pick_year(input_year: u32) -> Result<u32> {
    let final_year = if input_year == 0 {
        let fallback = current_year();
        // For the official puzzle, disallow before December if current
        if fallback == get_current_year() && get_current_month() < 12 {
            anyhow::bail!("Can't fetch AoC puzzle for the current year before December.");
        }
        fallback
    } else {
        input_year
    };

    let current = get_current_year();
    if final_year < 2015 || final_year > current {
        anyhow::bail!("Year must be between 2015 and {current}");
    }
    Ok(final_year)
}

/// Returns the year from AOC_YEAR env or system year
fn current_year() -> u32 {
    env::var("AOC_YEAR")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or_else(get_current_year)
}

/// Returns the system year
fn get_current_year() -> u32 {
    jiff::Zoned::now().year() as u32
}

/// Returns the system month
fn get_current_month() -> u32 {
    jiff::Zoned::now().month() as u32
}
