use minigrep::search::SearchConfig;
use std::{env, error::Error};

fn parse_config(args: &[String]) -> Result<SearchConfig, Box<dyn Error>> {
    match args.len() {
        3 => Ok(SearchConfig::new(&args[2], &args[1])),
        _ => Err(Box::new(std::io::Error::other("Must specify exactly 2 CLI arguments! minigrep <file> <search>"))),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let cfg = parse_config(&args)?;
    println!("Searching file [{}] for expression/term: [{}]", cfg.file_path(), cfg.search_term());
    Ok(())
}
