use clap::Parser;

/// a simple wc
#[derive(Parser)]
struct Cli {
    /// print the byte counts
    #[arg(short = 'c', long, default_value_t = false)]
    bytes: bool,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    let bytes_count = content.bytes().len();

    println!("{} {}", bytes_count, args.path.to_str().unwrap())
}
