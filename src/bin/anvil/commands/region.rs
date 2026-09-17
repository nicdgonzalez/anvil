use std::io::Write as _;
use std::{fmt, io};

use anvil::{Block, Chunk, Region};
use anyhow::Context as _;

use crate::commands::{CommandContext, Run};

#[derive(clap::Args)]
pub struct RegionArgs {
    /// The X coordinate of a block, chunk, or region.
    x: i64,

    /// The Z coordinate of a block, chunk, or region.
    z: i64,

    /// Determines how to interpret the given coordinates.
    #[clap(long, default_value_t = CoordinateKind::Block)]
    kind: CoordinateKind,
}

#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
enum CoordinateKind {
    #[default]
    Block,
    Chunk,
    Region,
}

impl fmt::Display for CoordinateKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Block => "block".fmt(f),
            Self::Chunk => "chunk".fmt(f),
            Self::Region => "region".fmt(f),
        }
    }
}

impl Run for RegionArgs {
    fn run(self, _: CommandContext) -> anyhow::Result<()> {
        let region = match self.kind {
            CoordinateKind::Block => Block::new(self.x, 0, self.z).unwrap().into_region(),
            CoordinateKind::Chunk => Chunk::new(self.x, 0, self.z).unwrap().into_region(),
            CoordinateKind::Region => Region::new(self.x, self.z),
        };

        writeln!(io::stdout(), "r.{}.{}.mca", region.x(), region.z())
            .context("failed to write to stdout")?;

        Ok(())
    }
}
