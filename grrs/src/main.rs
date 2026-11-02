use clap::Parser;
use grrs::utilities;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = utilities::Cli::parse();

    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            return Err(error.into());
        }
    };

    for line in content.lines() {
        if line.contains(&args.filter) {
            println!("{}", line);
        }
    }

    Ok(())
}
