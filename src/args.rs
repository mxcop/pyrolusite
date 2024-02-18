use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "pyro")]
#[command(author = "Max (mxcop) <mxcop.dev@gmail.com>")]
#[command(about = "A basic static html blog generator", long_about = None)]
pub struct Args {
  /// Command to execute
  #[command(subcommand)]
  pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Parse a pyrolusite markdown blog.
    Build {
        /// Path to the markdown files to compile.
        #[arg(default_value = ".")]
        path: String,
        /// Build directory.
        #[arg(short, long, default_value = "./build/")]
        output: String,
        /// Copied into the build directory if it exists.
        #[arg(short, long, default_value = "./public/")]
        styles: String,
    }
}
