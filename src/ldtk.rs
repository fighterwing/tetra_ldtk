use super::json_0_8_1::*;

use std::{fs::File, path::Path};

impl Project {
    pub fn new(f: String) -> Self {
        let json_file_path = Path::new(&f);
        let file = File::open(json_file_path)
            .expect("project file not found.");
        let o: Project = serde_json::from_reader(file)
            .expect("error while reading.");
        o
    }
}