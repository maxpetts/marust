use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

fn main() {
    // TODO: use canonicalize function to get a safe path to file

    let in_filename = "test_files/list.md";
    let out_filename = "fulldoc.html";
    let debug = true;

    let file = BufReader::new(match File::open(in_filename) {
        Ok(file) => file,
        Err(e) => panic!("Problem opening file: {}", e),
    });

    // TODO: Only use a BufWriter when file sizes are medium-large
    // Should we use a line writer instead?
    let mut out_file = BufWriter::new(match File::create(out_filename) {
        Ok(file) => file,
        Err(e) => panic!("Error creating file: {}", e),
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
                "" => output(out_file.by_ref(), format!("{}", text)),
                "_" => output(
                    out_file.by_ref(),
                    format!("<em>{}", text.replacen("_", "</em>", 1)),
                ),
                "**" => output(
                    out_file.by_ref(),
                    format!("<b>{}", text.replacen("**", "</b>", 1)),
                ),
                "`" => output(
                    out_file.by_ref(),
                    format!("<code>{}", text.replacen("`", "</code>", 1)),
                ),
                "~~" => output(
                    out_file.by_ref(),
                    format!("<s>{}", text.replacen("~~", "</s>", 1)),
                ),
                &_ => {
                    if mkd_ops.contains("#") {
                        output(
                            out_file.by_ref(),
                            format!("<h{len}>{}</h{len}>", text, len = mkd_ops.len()),
                        );
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
                            output(out_file.by_ref(), format!("<li>{}</li></ol>", text));
                        } else if mkd_ops == "1." {
                            // LISTS START AT ONE
                            output(out_file.by_ref(), format!("<ol><li>{}</li>", text));
                        } else {
                            output(out_file.by_ref(), format!("<li>{}</li>", text));
                        }
                    } else {
                        output(
                            out_file.by_ref(),
                            "error in markdwon/ not implemented".to_string(),
                        );
                        // output(out_file, format!("ERROR in markdown / not implemented :)"));
                    }
                }
            }
        } else {
            output(out_file.by_ref(), "<br>".to_string())
        }
    }
    out_file.flush().unwrap();
}

fn output<W: Write>(buffer: &mut W, content: String) {
    buffer.write(content.as_bytes()).unwrap();
}
