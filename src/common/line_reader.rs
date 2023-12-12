use std::io::BufRead;

pub fn read_lines(path: &str) -> std::io::Result<Vec<String>>{
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    Ok(reader.lines().flat_map(|line|{line}).collect())
}