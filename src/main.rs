use clap::Parser;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct CliArguments{
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = CliArguments::parse();
    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => { content },
        Err(error) => { return Err(error.into()); }
    };

    println!("file content {}", content);
    Ok(())
}