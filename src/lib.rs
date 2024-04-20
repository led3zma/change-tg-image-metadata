use std::{error::Error as std_error, ffi::OsString, fs, path::Path};

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
            path.contains(".jpg") || path.contains(".png") || path.contains(".jpeg")
        })
        .collect())
}

/// Reads the path of a file and extracts the datetime from the file name
/// Since the file name has the following known format: photo_XX@XX-XX-XXXX_XX-XX-XX.jpg
/// we can take advantage of this by just reverse splitting the string until the @ and stripping the file extension
///
/// NOTE: If for some reason the telegram file export changes this format, it would be better to use regex to match
/// the date and time in the string
pub fn extract_datetime(file_path: OsString) -> Option<String> {
    Some(
        Path::new(&file_path)
            .file_stem()?
            .to_str()?
            .rsplit_once("@")?
            .1
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
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
                    !file.contains(".jpg") && !file.contains(".png") && !file.contains(".jpeg")
                })
                .collect::<Vec<_>>()
                .len()
                == 0
        );
    }

    #[test]
    fn extract_raw_datetime_from_filename() {
        let image_file = read_image_path("./test_folder/").unwrap().pop().unwrap();
        assert_eq!(
            String::from("31-12-2020_19-00-00"),
            extract_datetime(image_file).unwrap()
        );
    }
}
