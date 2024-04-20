use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path where the images are located
    path: String,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
