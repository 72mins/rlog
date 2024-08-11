use std::{fs, io::Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    title: String,
    navtitle: String,
    description: String,
}

pub fn extract_meta(file_path: &str) -> Result<Meta, Error> {
    let meta_content = match fs::read_to_string(file_path) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);

            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "Error reading file",
            ));
        }
    };

    let meta: Meta = match toml::from_str(&meta_content) {
        Ok(m) => m,
        Err(e) => {
            println!("{:?}", e);

            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Error parsing meta file",
            ));
        }
    };

    Ok(meta)
}

