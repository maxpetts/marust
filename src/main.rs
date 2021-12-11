use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // TODO: use canonicalize function to get a safe path to file

    let in_filename = "test_files/fulldoc.md";
    let out_filename = "out/heading.html";
    let debug = false;

    let file = BufReader::new(match File::open(in_filename) {
        Ok(file) => file,
        Err(e) => panic!("Problem opening file: {}", e),
    });

    let mut iter = file.lines().enumerate().peekable();

    let reg_line_split =
        Regex::new(r"(^\s*)([#_*~`\d.]*)\s?(.*)").expect("Error when creating Regex"); // TODO: add markdown 'operators' into regex

    while let Some((idx, r_line)) = iter.next() {
        let line = match r_line {
            Ok(line) => line,
            Err(e) => panic!("Error reading a line: {}", e),
        };

        if line.len() > 0 {
            let blocks = reg_line_split
                .captures(&line)
                .expect(format!("Error parsing line number: {}", idx).as_str()); // What about super long lines - will this overflow??

            let prfx_spaces: &str = &blocks[1];
            let mkd_ops: &str = &blocks[2];
            let text: &str = &blocks[3];

            if debug {
                println!(
                    "\n\n indent: \"{}\", ops: \"{}\", text: \"{}\"\n------------\n",
                    prfx_spaces.len(),
                    mkd_ops,
                    text
                );
            }

            match mkd_ops {
                "" => println!("{}", text),
                "_" => println!("<em>{}", text.replacen("_", "</em>", 1)),
                "**" => println!("<b>{}", text.replacen("**", "</b>", 1)),
                "`" => println!("<code>{}", text.replacen("`", "</code>", 1)),
                "~~" => println!("<s>{}", text.replacen("~~", "</s>", 1)),
                &_ => {
                    if mkd_ops.contains("#") {
                        println!("<h{len}>{}</h{len}>", text, len = mkd_ops.len());
                    } else if mkd_ops.chars().any(char::is_numeric) {
                        let next_line = if let Some(line) = iter.peek() {
                            match &line.1 {
                                Ok(line) => line,
                                Err(e) => panic!(
                                    "Error fetching string from next line result type: {}",
                                    e
                                ),
                            }
                        } else {
                            ""
                        };

                        if next_line == "" {
                            println!("<li>{}</li></ol>", text);
                        } else {
                            if mkd_ops == "1." {
                                println!("<ol><li>{}</li>", text);
                            } else {
                                println!("<li>{}</li>", text);
                            }
                        }
                    } else {
                        println!("ERROR in markdown / not implemented :)");
                    }
                }
            }
        } else {
            println!();
        }
    }
}
