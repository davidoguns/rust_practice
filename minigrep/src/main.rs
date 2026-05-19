use std::env;
use minigrep::search::SearchConfig;

fn parse_config(args: &[String]) -> Option<SearchConfig> {
    match args.len() {
        2 => Some(SearchConfig::new(args[2].clone(), args[1].clone())),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = parse_config(&args);

    println!("Hello, world!");
}
