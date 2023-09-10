use std::{fs::File, io::BufRead, io::BufReader};

use clap::{Arg, Command};
use regex::Regex;

fn main() {
    let args = Command::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for")
                .value_name("pattern")
                .required(true),
        )
        .get_matches();

    let pattern = args.get_one::<String>("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    let f = File::open("Cargo.toml").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        let contains_substring = re.find(&line);
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
