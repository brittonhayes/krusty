use clap::App;
use colored::*;
use error_chain::error_chain;
use std::path::Path;
use std::{env, fs};

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
        .subcommand(App::new("crawl").about("looks around for anything worth deleting"))
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("crawl") {
        crawl();
    }
}

// crawl digs through the current directory
// for old files
fn crawl() -> Result<()> {
    let current_dir = env::current_dir()?;
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

        if last_modified < 1 * 3600 && metadata.is_file() {
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
