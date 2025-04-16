use clap::{Parser, Subcommand};
use classify::{ClassifyArgs, classify};

mod classify;

#[cfg(feature = "train")]
mod train;

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(args_conflicts_with_subcommands = true)]
struct Cli {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[command(subcommand)]
    command: Option<Commands>,
    #[command(flatten)]
    classify_args: ClassifyArgs,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Classify(ClassifyArgs),
    #[cfg(feature = "train")]
    Train(train::TrainArgs),
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    // init logging
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    match args.command {
        Some(Commands::Classify(args)) => classify(args),
        #[cfg(feature = "train")]
        Some(Commands::Train(args)) => train::train(args),
        None => classify(args.classify_args),
    }
}
