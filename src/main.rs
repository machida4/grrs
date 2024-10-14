use clap::Parser;
use std::io::BufRead;
use std::path;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");

    let file = std::fs::File::open(&args.path).expect("could not read file");

    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains(&args.pattern) {
                println!("{}", line);
            }
        }
    }
}
