use minigrep::{search, search_insensitive, SearchConfig};
use std::fs;
use std::{env, error::Error};

pub fn run(cfg: &SearchConfig) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cfg.file_path())
        .expect(format!("Could not read contents of file: {}", cfg.file_path()).as_str());
    if cfg.ignore_case() {
        for (line_no, line) in search_insensitive(cfg.search_term().as_str(), contents.as_str()) {
            println!("{line_no}:{line}");
        }
    } else {
        for (line_no, line) in search(cfg.search_term().as_str(), contents.as_str()) {
            println!("{line_no}:{line}");
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let cfg = SearchConfig::build(&args)?;
    match run(&cfg) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error running minigrep");
            Err(e)
        }
    }
}
