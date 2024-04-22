use std::error::Error;

use clap::{command, Parser};
use telegram_image_metadata::{read_image_path, update_time_metadata};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path where the images are located
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let files = read_image_path(&args.path)?;

    files
        .into_iter()
        .for_each(|file| update_time_metadata(file).unwrap());
    Ok(())
}
