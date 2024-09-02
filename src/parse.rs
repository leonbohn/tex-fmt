//! Utilities for reading the command line arguments

use clap::Parser;

/// Acceptable file extensions
const EXTENSIONS: [&str; 4] = [".tex", ".bib", ".sty", ".cls"];

/// Command line arguments
#[allow(missing_docs)]
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(long, short, help = "Check formatting, do not modify files")]
    pub check: bool,
    #[arg(long, short, help = "Print to STDOUT, do not modify files")]
    pub print: bool,
    #[arg(long, short, help = "Keep lines, do not wrap")]
    pub keep: bool,
    #[arg(long, short, help = "Show info log messages")]
    pub verbose: bool,
    #[arg(long, short, help = "Show trace log messages")]
    pub trace: bool,
    #[arg(required = true)]
    pub files: Vec<String>,
}

impl Cli {
    /// Ensure the provided arguments are consistent
    pub fn resolve(&mut self) {
        if self.trace {
            self.verbose = true;
        }
    }

    #[cfg(test)]
    pub const fn new() -> Self {
        Self {
            check: false,
            print: false,
            keep: false,
            verbose: false,
            trace: false,
            files: Vec::<String>::new(),
        }
    }
}

/// Verify the file extension
pub fn check_extension_valid(file: &str) -> bool {
    EXTENSIONS.iter().any(|e| file.ends_with(e))
}
