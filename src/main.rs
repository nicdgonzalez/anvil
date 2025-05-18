use anyhow::Context;
use clap::Parser;
use colored::Colorize;

#[derive(Debug, clap::Parser)]
struct Args {
    /// Path to an Anvil file of a Minecraft server's `region` directory.
    #[arg(long, short)]
    input: std::path::PathBuf,
}

const HOMES: [&'static str; 10] = [
    "r.0.0.mca",  // Hilo, -268 -441
    "r.0.0.mca",  // Nic (old), -133 -478
    "r.0.-3.mca", // Nic, 175 -1141
    "r.1.-3.mca", // Wari, 634 -1081
    "r.0.-2.mca", // Cheska, 428 -713
    "r.0.0.mca",  // Aria (old), 197 330
    "r.0.1.mca",  // Miya, -353 550
    "r.0.-2.mca", // Sonny, -226 -737
    "r.0.0.mca",  // Baryonyx, -298 -146
    "r.-7.2.mca", // Kait, -3090 1087
];

fn main() {
    tracing_subscriber::fmt::init();

    if let Err(err) = try_main() {
        eprintln!("{}: {}", "error".bold().red(), err);
        std::process::exit(1);
    }
}

fn try_main() -> anyhow::Result<()> {
    let Args { input } = Args::parse();

    let path = std::path::PathBuf::from(&input)
        .canonicalize()
        .context("failed to canonicalize input filepath")?;

    let file_name = path
        .file_name()
        .map(|name| name.to_str().unwrap_or(""))
        .context("expected input to point to a file")?;

    if HOMES.contains(&file_name) {
        return Ok(());
    }

    let file = std::fs::File::open(&path).context("failed to read input file")?;
    let mut reader = std::io::BufReader::new(&file);

    let chunk_timestamps = get_timestamps(&mut reader)?;

    let seven_days = 60 * 60 * 24 * 365;
    let delta = chrono::TimeDelta::from_std(std::time::Duration::from_secs(seven_days))
        .context("expected hard-coded duration to be within range")?;

    let mut timestamps = chunk_timestamps.iter();
    let mut timestamp_newest = timestamps
        .by_ref()
        .next()
        .context("expected at least one timestamp")?;

    for timestamp in timestamps.by_ref() {
        if timestamp > timestamp_newest {
            timestamp_newest = &timestamp;
        }
    }

    if chrono::Utc::now() - timestamp_newest > delta {
        println!(
            "File: {:#} (Last updated: {:#}",
            path.display(),
            timestamp_newest.with_timezone(&chrono::Local).to_rfc3339()
        );
    }

    Ok(())
}

fn get_timestamps<R>(reader: &mut R) -> anyhow::Result<Vec<chrono::DateTime<chrono::Utc>>>
where
    R: std::io::BufRead + std::io::Seek,
{
    // An Anvil file (extension `.mca`) has an 8 KiB header containing (2) 4 KiB tables.
    // The timestamps are located in the second table of the file's header as a 4 byte, big-endian
    // integer, representing the last modification time of a chunk in epoch seconds.

    _ = reader
        .seek(std::io::SeekFrom::Start(4096))
        .context("failed to seek to the second table in the header")?;

    // 1024 entries in the table, 4 bytes each.
    let mut buffer = vec![0u8; 1024 * 4];
    reader
        .read_exact(&mut buffer)
        .context("failed to read all timestamp entries")?;

    let timestamps = buffer
        .chunks(4)
        .filter_map(|chunk| {
            let array: [u8; 4] = chunk.try_into().expect("expected chunk to be 4 bytes");
            let seconds = u32::from_be_bytes(array);

            if seconds == 0 {
                return None;
            }

            chrono::DateTime::from_timestamp(seconds as i64, 0)
                .context("expected valid timestamp in header")
                .ok()
        })
        .collect();

    Ok(timestamps)
}
