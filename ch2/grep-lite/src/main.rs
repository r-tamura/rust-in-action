use std::{fs::File, io::BufRead, io::BufReader};

use clap::{Arg, Command};
use regex::Regex;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line in reader.lines() {
        let line = line.unwrap();
        let contains_substring = re.find(&line);
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

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
        .arg(
            Arg::new("file")
                .help("The file to search")
                .value_name("file")
                .required(false),
        )
        .get_matches();

    let pattern = args.get_one::<String>("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    let defaultfile = &"-".to_string();
    let filepath = args.get_one::<String>("file").unwrap_or(defaultfile);
    match &filepath[..] {
        "-" => {
            // Read from stdin
            let reader = std::io::stdin().lock();
            process_lines(reader, re);
        }
        filepath => {
            // Read from file
            let f = File::open(filepath).unwrap();
            let reader = BufReader::new(f);
            process_lines(reader, re);
        }
    }
}
