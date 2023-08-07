#![allow(dead_code)]

use c_art_2_volume_changes::{read_csv, records_to_data, dataset_to_stats};
use clap::Parser;
use std::error::Error;
use std::fs::File;
use simple_logger::SimpleLogger;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// CSV input file [semicolon delimited]
    #[arg(short, long)]
    file: String,
    /// JSON file where the results are written to.
    #[arg(short, long, default_value="volume_changes_stats.json")]
    results: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();
    let args = Args::parse();

    let records = read_csv(&args.file)?;
    let dataset = records_to_data(&records);
    let stats = dataset_to_stats(&dataset)?;

    let file = File::create(args.results)?;
    serde_json::to_writer_pretty(file, &stats)?;

    Ok(())
}
