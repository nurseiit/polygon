use anyhow::Result;
use clap::Parser;
use json::{cli, validator::is_file_a_valid_json};

fn main() -> Result<()> {
    let args = cli::JsonParserCliArgs::parse();
    let path = args.path;

    if is_file_a_valid_json(&path).is_ok_and(|is_valid| is_valid) {
        println!("'{}' is a VALID json", path.display());
    } else {
        println!("'{}' is NOT a VALID json", path.display());
    }

    Ok(())
}
