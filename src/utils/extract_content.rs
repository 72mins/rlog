use std::{fs, io::Error};

pub fn extract_content(file_path: &str) -> Result<String, Error> {
    let content = match fs::read_to_string(file_path) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);

            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "Error reading file",
            ))
        }
    };

    Ok(content)
}
