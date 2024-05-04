use std::{error::Error as std_error, ffi::OsString, fs, path::Path};

use chrono::NaiveDateTime;
use filetime::FileTime;

/// Reads a path and returns a Vec containing every file in it
/// TODO: filter to return only images
pub fn read_image_path(path: &str) -> Result<Vec<OsString>, Box<dyn std_error>> {
    Ok(fs::read_dir(path)?
        .filter(|file| file.is_ok())
        .map(|entry| {
            entry
                .map(|file| file.path().into_os_string())
                .expect("Should only contain OK values")
        })
        .filter(|path| {
            let path = &path.to_str().unwrap();
            !path.contains("_thumb.")
                && (path.contains(".jpg") || path.contains(".png") || path.contains(".jpeg"))
        })
        .collect())
}

/// Gets a file path and updates its modified time metadata by extracting the datetime in its file name and parsing it into timestamp
pub fn update_time_metadata(file_path: OsString) -> Result<(), std::io::Error> {
    filetime::set_file_mtime(
        &file_path,
        FileTime::from_unix_time(
            // TODO #5 Fix update_time_metadata to correctly handle an error if an extraction of a file name's datetime fails due to incorrect pattern or format
            get_timestamp(extract_datetime(&file_path).unwrap()).unwrap(),
            0,
        ),
    )
}

/// Reads the path of a file and extracts the datetime from the file name
/// Since the file name has the following known format: photo_XX@XX-XX-XXXX_XX-XX-XX.jpg
/// we can take advantage of this by just reverse splitting the string until the @ and stripping the file extension
///
/// NOTE: If for some reason the telegram file export changes this format, it would be better to use regex to match
/// the date and time in the string
pub fn extract_datetime(file_path: &OsString) -> Option<String> {
    Some(
        Path::new(&file_path)
            .file_stem()?
            .to_str()?
            .rsplit_once("@")?
            .1
            .to_string(),
    )
}

/// Return the timestamp equivalent of the raw datetime extracted from a file name
fn get_timestamp(datetime: String) -> Result<i64, chrono::ParseError> {
    Ok(
        NaiveDateTime::parse_from_str(&datetime, "%d-%m-%Y_%H-%M-%S")
            .unwrap()
            .timestamp(),
    )
}

#[cfg(test)]
mod tests {
    use filetime::FileTime;

    use super::*;

    #[test]
    fn handle_invalid_image_path() {
        let path = "./test_folder/photo@31-12-2020_19-00-00.png";
        assert!(read_image_path(path).is_err());
    }

    #[test]
    fn collect_only_images() {
        let path = "./test_folder/";
        assert!(
            read_image_path(path)
                .unwrap()
                .iter()
                .filter(|&file| {
                    let file = &file.to_str().unwrap();
                    file.contains("_thumb.")
                        || (!file.contains(".jpg")
                            && !file.contains(".png")
                            && !file.contains(".jpeg"))
                })
                .collect::<Vec<_>>()
                .len()
                == 0
        );
    }

    #[test]
    fn extract_raw_datetime_from_filename() {
        let image_file = extract_datetime(&OsString::from(
            "test_folder/photo_1@31-12-2020_19-00-00.png",
        ));
        assert_eq!(String::from("31-12-2020_19-00-00"), image_file.unwrap());
        let image_file = extract_datetime(&OsString::from(
            "test_folder/photo_131-12-2020_19-00-00.png",
        ));
        assert!(image_file.is_none());
    }

    #[test]
    fn get_timestamp_from_filename_raw_datetime() {
        let image_datetime = extract_datetime(&OsString::from(
            "test_folder/photo_1@31-12-2020_19-00-00.png",
        ))
        .unwrap();
        assert_eq!(1609441200, get_timestamp(image_datetime).unwrap());
    }

    #[test]
    fn change_modified_time_from_file_metadata() {
        let image_file = OsString::from("test_folder/photo_1@31-12-2020_19-00-00.png");

        // Just to ensure it has an arbitrary modification time before setting the actual time
        filetime::set_file_mtime(&image_file, FileTime::from_unix_time(100000, 0)).unwrap();

        let extracted_datetime = extract_datetime(&image_file).unwrap();
        update_time_metadata(image_file, extracted_datetime).unwrap();

        let image_file = OsString::from("test_folder/photo_1@31-12-2020_19-00-00.png");
        let image_last_mod_time =
            FileTime::from_last_modification_time(&fs::metadata(image_file).unwrap());
        assert_eq!(1609441200, image_last_mod_time.seconds());
    }
}
