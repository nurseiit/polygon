use clap::Parser;

#[derive(Parser)]
pub struct JsonParserCliArgs {
    pub path: std::path::PathBuf,
}
