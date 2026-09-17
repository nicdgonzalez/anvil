use std::env;
use std::path::PathBuf;

mod completions;
mod last_update;
mod region;

/// Shared context for command execution.
pub(super) struct CommandContext {
    pub directory: PathBuf,
}

pub(super) trait Run
where
    Self: clap::Args,
{
    /// Executes the subcommand.
    fn run(self, ctx: CommandContext) -> anyhow::Result<()>;
}

#[derive(clap::Parser)]
pub struct Parser {
    #[clap(subcommand)]
    pub subcommand: Subcommand,

    /// Change to the specified directory prior to running the command.
    #[clap(long, short = 'C', global = true)]
    pub directory: Option<PathBuf>,
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// Generate auto-complete options for your preferred shell
    #[clap(hide = true)]
    Completions(completions::Completions),

    /// Converts coordinates into region coordinates.
    Region(region::RegionArgs),

    /// Get the `LastUpdate` timestamp for each chunk in a region.
    LastUpdate(last_update::LastUpdateArgs),
}

/// Executes the user-selected subcommand.
pub fn run(args: Parser) -> anyhow::Result<()> {
    let directory = args
        .directory
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from("/")));

    let ctx = CommandContext { directory };

    match args.subcommand {
        Subcommand::Completions(cmd) => cmd.run(ctx),
        Subcommand::Region(cmd) => cmd.run(ctx),
        Subcommand::LastUpdate(cmd) => cmd.run(ctx),
    }
}
