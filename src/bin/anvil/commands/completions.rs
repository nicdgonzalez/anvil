use std::io;

use clap::CommandFactory as _;
use clap_complete::Shell;

use crate::commands::{CommandContext, Parser, Run};

#[derive(Debug, Clone, clap::Args)]
pub struct Completions {
    shell: Shell,
}

impl Run for Completions {
    fn run(self, _: CommandContext) -> anyhow::Result<()> {
        let mut command = Parser::command();

        clap_complete::generate(
            self.shell,
            &mut command,
            env!("CARGO_BIN_NAME"),
            &mut io::stdout(),
        );

        Ok(())
    }
}
