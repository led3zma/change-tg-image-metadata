use filetime::FileTime;
use std::fs;

fn main() {
    let metadata = fs::metadata("test_folder/photo@31-12-2020_19-00-00.png").unwrap();
    println!("Raw file metadata: {:?}", metadata);

    filetime::set_file_mtime(
        "test_folder/photo@31-12-2020_19-00-00.png",
        FileTime::from_unix_time(100000, 0),
    )
    .unwrap();

    let mtime = FileTime::from_last_modification_time(&metadata);
    println!("From last mod time: {:?}", mtime);
    println!("{}", mtime.seconds());
}
