use clap::Parser;
use clap::builder::styling::{AnsiColor, Effects, Style};
use std::env;
use std::fmt::Write;
use std::fs;
use std::io::{self, IsTerminal};
use std::path::PathBuf;

use martial::core::verify::verify_file;
use martial::types::ManifestSummary;

/// CLI tool for checking C2PA manifests for generative AI use
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to media to verify
    #[arg(short, long)]
    path: PathBuf,
}

// https://no-color.org/
// if NO_COLOR env exists, do not use color
//
// Also prevents ansi escape codes when output does not go directly to terminal
// e.g. if its piped into another command
fn use_color() -> bool {
    env::var_os("NO_COLOR").is_none() && io::stdout().is_terminal()
}

fn format_output(summary: Vec<ManifestSummary>) -> String {
    let mut out = String::new();
    let use_color = use_color();

    // styles
    let header_style = Style::new()
        .fg_color(Some(AnsiColor::Cyan.into()))
        .effects(Effects::BOLD);
    let success_style = Style::new().fg_color(Some(AnsiColor::Green.into()));
    let error_style = Style::new()
        .fg_color(Some(AnsiColor::Red.into()))
        .effects(Effects::BOLD);

    for (idx, item) in summary.iter().enumerate() {
        if idx > 0 {
            out.push_str("\n---\n\n");
        }

        // Header
        if use_color {
            let _ = writeln!(out, "{header_style}Issuer: {}{header_style:#}", item.issuer);
        } else {
            let _ = writeln!(out, "Issuer: {}", item.issuer);
        }

        // ai present
        if item.ai_present {
            let present = if use_color {
                format!("{success_style}Detected{success_style:#}")
            } else {
                format!("Detected")
            };
            let _ = writeln!(out, "Generative AI: {}", present);

            for desc in &item.ai_description {
                let _ = writeln!(out, "  • {}", desc);
            }
        } else {
            let _ = writeln!(out, "Generative AI: None");
        }

        // error
        if !item.error.is_empty() {
            if use_color {
                let _ = writeln!(out, "{error_style}Error: {}{error_style:#}", item.error);
            } else {
                let _ = writeln!(out, "Error: {}", item.error);
            }
        }
    }

    out
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // read file into Vec<u8>
    let file_bytes = match fs::read(&args.path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", args.path.display(), e);
            std::process::exit(1);
        }
    };

    // detect mime type
    let mime_type = match infer::get(&file_bytes) {
        Some(kind) => kind.mime_type(),
        None => {
            eprintln!("Error: Could not determine the mime type of this file");
            std::process::exit(1);
        }
    };

    println!("Analyzing '{}' ({})\n", args.path.display(), mime_type);
    let summary = verify_file(file_bytes, mime_type).await;

    let output_formatted = format_output(summary);

    println!("{}", output_formatted);
}
