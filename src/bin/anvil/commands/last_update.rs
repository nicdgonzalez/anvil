use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom, Write as _};
use std::mem;
use std::path::PathBuf;

use anyhow::Context;
use chrono::{DateTime, Utc};

use crate::commands::{CommandContext, Run};

const SECTOR_SIZE: u64 = 4096;
const TABLE_SIZE: usize = 1024;
const ENTRY_SIZE: usize = mem::size_of::<u32>();

#[derive(clap::Args)]
pub struct LastUpdateArgs {
    /// Path to an Anvil file (.mca) with region data.
    #[clap(long, short)]
    input: PathBuf,
}

struct LastUpdate {
    relative_x: u8,
    relative_z: u8,
    timestamp: DateTime<Utc>,
}

impl Run for LastUpdateArgs {
    fn run(self, ctx: CommandContext) -> anyhow::Result<()> {
        let input = ctx.directory.join(self.input);
        let path = fs::canonicalize(input).context("failed to canonicalize input")?;
        let file = File::open(&path).context("failed to open file")?;
        let chunks = parse_chunks(file).context("failed to get chunk information")?;

        for chunk in chunks {
            let x = chunk.relative_x;
            let z = chunk.relative_z;

            writeln!(io::stdout(), "{x},{z}: {}", chunk.timestamp.to_rfc3339()).ok();
        }

        writeln!(io::stderr(), "Done!").ok();

        Ok(())
    }
}

fn parse_chunks<R>(mut reader: R) -> anyhow::Result<Vec<LastUpdate>>
where
    R: Read + Seek,
{
    // The Anvil file format is built-up of 4 KiB "sectors." The first sector contains the location
    // table for the chunks and the second contains timestamps for the last update of said chunks.
    //
    // Both tables have 1024 entries. Each entry is a big-endian, 32-bit integer.
    seek_to_timestamp_table(&mut reader).context("failed to seek to timestamp table")?;

    let mut buffer = vec![0u8; TABLE_SIZE * ENTRY_SIZE];
    reader
        .read_exact(&mut buffer)
        .context("failed to read timestamp entries")?;

    debug_assert!(buffer.len().is_multiple_of(ENTRY_SIZE));
    let (entries, remainder) = buffer.as_chunks::<ENTRY_SIZE>();
    debug_assert!(remainder.is_empty());

    let mut chunks = Vec::with_capacity(TABLE_SIZE);

    for (index, bytes) in entries.iter().copied().enumerate() {
        let seconds = u32::from_be_bytes(bytes);

        if is_empty_entry(seconds) {
            continue;
        }

        let timestamp = DateTime::from_timestamp(i64::from(seconds), 0)
            .context("invalid timestamp; seconds out of bounds")?;

        let relative_x = u8::try_from(index % 32).unwrap();
        let relative_z = u8::try_from(index / 32).unwrap();

        chunks.push(LastUpdate {
            relative_x,
            relative_z,
            timestamp,
        });
    }

    Ok(chunks)
}

fn seek_to_timestamp_table<R>(reader: &mut R) -> Result<u64, io::Error>
where
    R: Read + Seek,
{
    // Skip the location table (first 4 KiB sector).
    reader.seek(SeekFrom::Start(SECTOR_SIZE))
}

fn is_empty_entry(seconds: u32) -> bool {
    // A zero timestamp indicates an empty entry (4 null bytes were read).
    seconds == 0
}
