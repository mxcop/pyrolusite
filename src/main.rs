use args::Commands;
use clap::Parser;

mod args;
mod post;
mod blog;
mod builder;

fn main() {
    let args = args::Args::parse();

    match args.cmd {
        Commands::Build { path, output, styles } => {
            builder::build(&path, &output, &styles)
        },
    }.expect("command failed");
}
