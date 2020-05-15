use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn load_file(filename: &String) -> Result<Vec<String>, io::Error> {
    // Read all the lines from the file and enter them into a vector of strings.
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut strings: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        strings.push(line.trim().to_string());
    }

    Ok(strings)
}
