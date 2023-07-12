use clap::Parser;
use std::path::PathBuf;
use std::process::Command;

/// Simple program to delete files by moving them to the trash.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of file(s) to delete.
    #[arg(required = true, short, long, num_args = 1..,)]
    file_paths: Vec<PathBuf>,
}

#[cfg(target_os = "macos")]
fn move_to_trash(file_path: &std::path::Path) -> Result<(), String> {
    match Command::new("sh")
        .arg("-c")
        .arg(format!("mv -f {} ~/.Trash", file_path.to_string_lossy()))
        .status()
    {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(format!("Failed to move file to trash. Exit status: {:?}", status)),
        Err(err) => Err(format!("Error executing command: {}", err)),
    }
}

fn main() {
    let args = Args::parse();

    if args.file_paths.is_empty() {
        eprintln!("No files specified.");
        return;
    }

    for file_path in args.file_paths {
        match move_to_trash(&file_path) {
            Ok(()) => println!("File '{}' deleted successfully.", file_path.display()),
            Err(err) => eprintln!("Error deleting file '{}': {}", file_path.display(), err),
        }
    }
}
