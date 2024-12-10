use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_file_to_string_array(file_path: &str) -> io::Result<Vec<String>> {
    let full_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), file_path);
    let path = Path::new(&full_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_to_string_array() {
        let expected_output = vec![
            String::from("line 1"),
            String::from("line 2"),
            String::from("line 3"),
        ];

        // Read the file and compare the output
        let result = read_file_to_string_array("src/test_input.txt").unwrap();
        assert_eq!(result, expected_output);

    }
}