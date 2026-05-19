use minigrep::search::SearchConfig;
use std::{env, error::Error};

fn parse_config(args: &[String]) -> Option<SearchConfig> {
    match args.len() {
        2 => Some(SearchConfig::new(&args[2], &args[1])),
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let cfg = parse_config(&args);

    match cfg {
        Some(cfg) => {
            println!(
                "Searching file [{}] for expression/term: [{}]",
                cfg.file_path(),
                cfg.search_term()
            );
            Ok(())
        }
        None => Err(Box::<std::io::Error>::new(std::io::Error::other(
            "Failed to parse CLI args",
        ))),
    }
}
