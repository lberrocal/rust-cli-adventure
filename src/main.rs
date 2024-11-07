use clap::Parser;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct CliArguments{
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = CliArguments::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}