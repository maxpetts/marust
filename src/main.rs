use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// TODO: use canonicalize function to get a safe path to file

fn main() {
    let in_filename = "tests/heading.md";
    let out_filename = "out/heading.html";

    let file = BufReader::new(match File::open(in_filename) {
        Ok(file) => file,
        Err(e) => panic!("Problem opening file: {}", e),
    });

    let reg_line_split = Regex::new(r"(^\s*)([#]*\s?)(.*)").expect("Error when creating Regex"); // TODO: add markdown 'operators' into regex

    for (line_index, r_line) in file.lines().enumerate() {
        let line = match r_line {
            Ok(line) => line,
            Err(e) => panic!("Error reading a line: {}", e),
        };

        if line.len() > 0 {
            // check if line begins with letters -> its just <p> then

            let blocks = reg_line_split
                .captures(&line)
                .expect(format!("Error parsing line number: {}", line_index).as_str()); // What about super long lines - will this overflow??

            let prfx_spaces: &str = &blocks[1];
            let mkd_ops: &str = &blocks[2];
            let text: &str = &blocks[3];

            println!(
                "\nLine {}:\n indent: {}, ops: {}, text: {}",
                line_index,
                prfx_spaces.len(),
                mkd_ops,
                text
            );
        };
    }
}
