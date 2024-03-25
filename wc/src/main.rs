use clap::Parser;

/// a simple wc
#[derive(Parser)]
struct Cli {
    /// print the byte counts
    #[arg(short = 'c', long, default_value_t = false)]
    bytes: bool,

    /// print the newline counts
    #[arg(short, long, default_value_t = false)]
    lines: bool,

    /// print the word counts
    #[arg(short, long, default_value_t = false)]
    words: bool,

    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    if args.bytes {
        let bytes_count = content.bytes().len();
        print!("{:>7} ", bytes_count)
    }

    if args.lines {
        let lines_count = content.lines().count();
        print!("{:>7} ", lines_count)
    }

    if args.words {
        let words_count = content.split_whitespace().count();

        print!("{:>7} ", words_count)
    }

    println!("{}", args.path.to_str().unwrap())
}
