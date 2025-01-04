use std::path::PathBuf;

mod read;
mod write;

fn main() -> anyhow::Result<()> {
    let csv_path = PathBuf::from("benches/data/benches.csv");


    let records = read::read_all_data(&csv_path)?;
    dbg!(&records);
    write::write_to_readme(&records)?;
    println!("Wrote readme.md");
    write::write_to_csv(&csv_path, &records)?;
    println!("Wrote benches.csv");

    Ok(())
}
