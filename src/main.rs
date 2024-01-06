use filetime::FileTime;
use std::fs;

fn main() {
    let metadata = fs::metadata("tests/20230224_182502.jpg").unwrap();
    println!("Raw file metadata: {:?}", metadata);

    let mtime = FileTime::from_last_modification_time(&metadata);
    println!("From last mod time: {:?}", mtime);
    println!("{}", mtime.seconds());
}
