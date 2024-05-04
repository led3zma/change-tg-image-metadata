use std::error::Error;

use clap::{command, Parser};
use telegram_image_metadata::{extract_datetime, read_image_path, update_time_metadata};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path where the images are located
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let files = read_image_path(&args.path)?;

    files.into_iter().for_each(|file_path| {
        if let Some(extracted_datetime) = extract_datetime(&file_path) {
            update_time_metadata(file_path, extracted_datetime).unwrap()
        }
    });
    Ok(())
}
