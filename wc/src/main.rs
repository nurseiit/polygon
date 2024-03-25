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

    /// print the character counts
    #[arg(short = 'm', long, default_value_t = false)]
    chars: bool,

    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    let lines_count = content.lines().count();
    let words_count = content.split_whitespace().count();
    let char_count = content.chars().count();
    let bytes_count = content.bytes().len();

    if args.lines {
        print!("{:>6} ", lines_count)
    }

    if args.words {
        print!("{:>6} ", words_count)
    }

    if args.chars {
        print!("{:>6} ", char_count)
    }

    if args.bytes {
        print!("{:>6} ", bytes_count)
    }

    println!("{}", args.path.to_str().unwrap())
}
