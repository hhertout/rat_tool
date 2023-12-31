use std::fs;

use crate::YamlParser;
use schema::Schema;

use logger::Logger;

pub struct FileCopier {
    pub config_file_path: String,
}

impl YamlParser for FileCopier {
    fn parse_yml(&self) -> Schema {
        let content = fs::read_to_string(&self.config_file_path);
        if content.is_err() {
            Logger::error_file_not_found();
        }
        let content_unwrap = content.unwrap();
        match serde_yaml::from_str(&content_unwrap) {
            Ok(parsed_content) => parsed_content,
            Err(_) => {
                Logger::invalid_config_file();
                panic!("Process exited")
            }
        }
    }
}

impl FileCopier {
    pub fn new(config_file_path: String) -> FileCopier {
        FileCopier { config_file_path }
    }

    pub fn run_copy(&self, from: &str, to: &str) {
        let result = fs::copy(from, to);
        match result {
            Ok(_) => Logger::copy_success(from, to),
            Err(_) => Logger::copy_failed(from),
        }
    }
}
