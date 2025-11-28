use std::fs;

use rust_search::SearchBuilder;

fn main() {
    let gin_files: Vec<String> = SearchBuilder::default().ext("gin").build().collect();

    for file in gin_files {
        println!("{file}.md");
        for line in fs::read_to_string(file).unwrap().lines() {
            // Header, comment, or tag
            if line.starts_with("~#") {
                if line.trim().ends_with(" #~") {
                    // Header
                    let markdown = line.replace("~#", "#").replace(" #~", "");
                    println!("{markdown}");
                } else if line.trim().ends_with("~") {
                    // Tag
                    let markdown = line.replace("~#", "#").replace("~", "");
                    println!("{markdown}");
                } else {
                    // Comment
                    let mut markdown = line.replace("~#", "_(").trim().to_owned();
                    markdown.push_str(")_");
                    println!("{markdown}");
                }
            } else if line.starts_with("~|") {
                // Table
                // TODO: tables
                let markdown = line[1..].replace(":", "|");
                println!("{markdown}");
            } else if line.starts_with("~") {
                if line.chars().nth(1).unwrap().is_digit(10) {
                    // Ordered list
                    let num = line.chars().skip(1)
                        .map_while(|c| c.to_digit(10))
                        .fold(0, |acc, digit| acc * 10 + digit);
                    let markdown = format!("{num}. {}", line.split_once(' ').unwrap().1);
                    println!("{markdown}");
                } else {
                    // unordered list
                    let markdown = format!("- {}", line.split_once(' ').unwrap().1);
                    println!("{markdown}");
                }
            } else {
                println!("{line}");
            }
        }
    }
}
