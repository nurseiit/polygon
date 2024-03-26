use anyhow::{Context, Result};
use clap::Parser;

/// a simple wc
#[derive(Parser)]
struct Cli {
    /// print the byte counts
    #[arg(short = 'c', long)]
    bytes: bool,

    /// print the newline counts
    #[arg(short, long)]
    lines: bool,

    /// print the word counts
    #[arg(short, long)]
    words: bool,

    /// print the character counts
    #[arg(short = 'm', long, default_value_t = false)]
    chars: bool,

    path: std::path::PathBuf,
}

fn log10(x: usize) -> usize {
    let mut result = 0;
    let mut num = x;
    while num > 0 {
        num /= 10;
        result += 1;
    }
    result
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("{}: No such file or directory", args.path.display()))?;

    let show_all = !args.chars & [args.lines, args.words, args.bytes].iter().all(|&x| !x);

    let mut counts_to_show: Vec<usize> = Vec::new();

    if args.lines || show_all {
        let lines_count = content.lines().count();
        counts_to_show.push(lines_count);
    }

    if args.words || show_all {
        let words_count = content.split_whitespace().count();
        counts_to_show.push(words_count);
    }

    if args.chars {
        let char_count = content.chars().count();
        counts_to_show.push(char_count);
    }

    if args.bytes || show_all {
        let bytes_count = content.bytes().len();
        counts_to_show.push(bytes_count);
    }

    if !counts_to_show.is_empty() {
        let largest_count = *counts_to_show.iter().max().unwrap_or(&0);
        let largest_width = log10(largest_count);
        let out = counts_to_show
            .iter()
            .map(|count| format!("{:>width$}", count, width = largest_width))
            .collect::<Vec<String>>()
            .join(" ");

        print!("{} ", out)
    }

    println!("{}", args.path.display());

    Ok(())
}
