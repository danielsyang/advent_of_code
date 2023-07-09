use std::fs;

pub fn read_file(path: &str) -> String {
    return match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_err) => panic!("Error reading file: {}", path),
    };
}
