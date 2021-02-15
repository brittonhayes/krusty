use clap::{App, Arg};
use colored::*;
use error_chain::error_chain;
use std::fs;
use std::path::Path;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        SystemTimeError(std::time::SystemTimeError);
    }
}

fn main() {
    let matches = App::new("krusty")
        .version("0.1.0")
        .about("Finds those krusty old files cluttering up your system.")
        .subcommand(
            App::new("crawl")
                .about("looks around for anything worth deleting")
                .arg(
                    Arg::new("directory")
                        .about("the folder to read")
                        .index(1)
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("crawl") {
        let dir = matches.value_of("directory").unwrap();
        let _ = crawl(dir.to_string());
    }
}

// crawl digs through the current directory
// for old files
fn crawl(dir: String) -> Result<()> {
    let current_dir = Path::new(&dir);
    println!("Krusty files found in {:?}:\n", current_dir);

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs() / 60;
        let fname = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .red();

        // TODO make this a configurable time parameter
        if last_modified < 1 * 1440 && metadata.is_file() {
            let lastmod: String = last_modified.to_string().cyan().to_string();
            println!(
                "Filename: {}\nSize: {:?} KB\nLast modified: {} min ago\n---",
                fname,
                metadata.len() / 1024,
                lastmod
            );
        }
    }

    Ok(())
}
