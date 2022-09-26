use crate::commands::GenerateSubcommands;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version)]
pub struct Cli {
    #[clap(action, global = true, short, long, help = "Show debug messages")]
    pub verbose: bool,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, PartialEq, Eq, Debug)]
pub enum Commands {
    #[clap(about = "Codegen related commands")]
    #[clap(arg_required_else_help = true)]
    Generate {
        #[clap(subcommand)]
        command: GenerateSubcommands,
    },
}
