use std::collections::HashSet;
use std::fmt::Write as _;
use std::{env, fs};

fn main() {
    let filename = env::args().nth(1).unwrap();
    let mut output = String::new();
    let input = fs::read_to_string(filename).unwrap();
    let mut section = None;
    let mut current_array = vec![];
    let mut seen = HashSet::new();
    let mut skipped = 0;
    for line in input.lines() {
        if line.starts_with("pub const") {
            assert!(section.is_none());
            assert!(line.ends_with("&[&str] = &["));
            section = Some(line.to_owned());
            continue;
        }

        if line == "];" {
            if let Some(start) = section {
                section = None;
                current_array.sort();
                let sorted = current_array
                    .iter()
                    .filter_map(|x: &&str| {
                        if !seen.insert((*x).to_string()) {
                            skipped += 1;
                            println!("{x}");
                            None
                        } else {
                            Some(format!("    \"{x}\",\n"))
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("");
                writeln!(output, "{start}\n{sorted}];\n").unwrap();
                current_array.clear();
                seen.clear();
                continue;
            } else {
                panic!()
            }
        }

        if line.is_empty() {
            assert!(section.is_none());
            continue;
        }

        assert!(
            line.starts_with("    \"") && line.ends_with("\","),
            "line {line}"
        );
        if line.trim().contains(" ") {
            println!("> {line}")
        }
        current_array.push(&line[5..line.len() - 2]);
    }

    println!("skipped {skipped}");
    fs::write("out", output.trim()).unwrap();
}
