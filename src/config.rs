use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct ConfigReader {
    contents: Vec<String>,
    file: File,
    num_of_lines: u32,
}

impl ConfigReader {
    pub fn new(file: &str) -> Self {
        Self {
            contents: Vec::new(),
            file: File::open(file).unwrap(),
            num_of_lines: 0,
        }
    }

    pub fn read(&mut self) {
        let reader = BufReader::new(&self.file);
        // Read by each line
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // ignore errors
            self.contents.push(line);
            self.num_of_lines = (index as u32) + 1;
        }

        let mut count = 0;
        for i in 0..self.num_of_lines {
            println!("Content {}: {}", i + 1, self.contents[count]);
            count += 1;
        }
    }
}
