use std::{fs::File, io::Read};

pub fn read(file_name: &str) -> Vec<String> {
    let mut f = File::open(file_name).expect(&format!("file not found: {}", file_name));

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect(&format!("cannot read file {}", file_name));

    contents
        .trim_end()
        .split("\n")
        .map(|s| s.to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading_a_file_returns_its_content() {
        let content = read("src/lib.rs");
        assert!(content.contains(&"use std::{fs::File, io::Read};".to_owned()))
    }
}
