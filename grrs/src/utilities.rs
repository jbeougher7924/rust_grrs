use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
pub struct Cli {
    /// The filter to look for
    #[arg(short = 'f', long = "filter", required = true)]
    pub filter: String,
    /// The path to the file to read
    #[arg(short = 'p', long = "path", required = true)]
    pub path: std::path::PathBuf,
    /// output to stream
    #[arg(short = 'o', long = "output")]
    pub output: Option<String>,
}
