use std::{error::Error, io, path::Path, process};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[arg(short, long, value_parser = verify_input_text)]
    file_name: String,
}
#[derive(Debug, serde::Deserialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u64>
}

fn main() {
    let args: Args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }

    if let Err(err) = read_csv(&args.file_name) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

fn read_csv(input: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(input)?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn verify_input_text (file_name: &str) -> Result<String, &'static str> {
    if Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("THE FILE NAME IS NOT EXISTS")
    }
}