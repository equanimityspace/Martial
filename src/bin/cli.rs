use clap::Parser;
use std::fs;
use std::path::PathBuf;

use martial::core::verify::verify_file;

// CLI tool for checking C2PA manifests for generative AI use
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // path to media
    #[arg(short, long)]
    file: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // read file into Vec<u8>
    let file_bytes = match fs::read(&args.file) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", args.file.display(), e);
            std::process::exit(1);
        }
    };

    // detect mime type
    let mime_type = match infer::get(&file_bytes) {
        Some(kind) => kind.mime_type(),
        None => {
            eprintln!("Error: Could not determine the MIME type of this file");
            std::process::exit(1);
        }
    };

    println!("Analyzing '{}' ({})", args.file.display(), mime_type);
    let summary = verify_file(file_bytes, mime_type).await;

    println!("{:#?}", summary);
}
