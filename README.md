# Anvil

**Anvil** is a command-line tool with utility scripts for working with a
Minecraft server's region files.

## Overview

The `anvil` command-line interface is written in Rust. It is responsible for
exposing data from the region files and outputting it in a way that other
programs can then use that data.

This data is collected by various Python scripts for analysis/post-processing.

Scripts are written in Python to demonstrate that any language can handle the
data output by the CLI, and to make it easier for others to modify it to fit
their needs.

## Getting Started

Clone the repository onto your machine:

```bash
git clone --depth=1 https://github.com/nicdgonzalez/anvil.git -- anvil
```

Change into the new `anvil` directory:

```bash
cd ./anvil
```

Use cargo to install the CLI:

```bash
cargo install --path . -- anvil
```

Now, for example, run the `latest_update_per_region.py` Python script to get
the latest update for all of the regions:

```bash
./scripts/latest_update_per_region.py \
    --last-update \
    --reverse \
    -- \
    <path_to_region_directory> \
    > ./region_data.csv
```

Example output:

```csv
Region X,Region Z,Chunk X,Chunk Z,Block X,Block Z,Last Update
1,-7,41,-193,656,-3088,2026-07-27T03:46:52+00:00
-3,-6,-65,-166,-1040,-2656,2026-07-31T00:55:34+00:00
-2,-7,-34,-224,-544,-3584,2026-08-08T21:00:25+00:00
0,-7,3,-224,48,-3584,2026-08-08T21:03:31+00:00
0,-8,0,-256,0,-4096,2026-08-08T21:12:08+00:00
```

## To do

- [ ] CLI should output all available region information.
- [ ] Try to write a script to reset individual chunks.
- [ ] Rewrite each script in Rust.
