#!/usr/bin/env python3

import argparse
import datetime
import pathlib
import re
import subprocess
import sys
from typing import NamedTuple, cast

FILE_NAME_PATTERN = re.compile(r"^r\.(-?\d+)\.(-?\d+)\.mca$")


class Entry(NamedTuple):
    region_x: int
    region_z: int

    chunk_x: int
    chunk_z: int

    block_x: int
    block_z: int

    timestamp: datetime.datetime

    def to_row(self) -> str:
        return f"{self.region_x},{self.region_z},{self.chunk_x},{self.chunk_z},{self.block_x},{self.block_z},{self.timestamp.isoformat()}"  # noqa: E501


def main() -> None:
    args = build_cli().parse_args()
    path = cast(pathlib.Path, args.path)

    chunks: list[Entry] = []

    for file in path.iterdir():
        region = parse_file_name(file.name)

        if region is None:
            raise RuntimeError("failed to get region from file name")

        x_region, z_region = region

        result = subprocess.run(
            ["anvil", "last-update", "--input", file.as_posix()],
            capture_output=True,
            text=True,
        )

        lines = result.stdout.splitlines()

        latest: Entry | None = None

        for line in lines:
            chunk_coordinates, timestamp_raw = line.split(": ", maxsplit=1)
            x_chunk_raw, z_chunk_raw = chunk_coordinates.split(",", maxsplit=1)

            # Calculate the chunk's position within the Minecraft world.
            x_chunk = (x_region * 32) + int(x_chunk_raw, 10)
            z_chunk = (z_region * 32) + int(z_chunk_raw, 10)

            # Block coordinates are probably more intuitive.
            x_block = x_chunk * 16
            z_block = z_chunk * 16

            timestamp = datetime.datetime.fromisoformat(timestamp_raw)

            if latest is None or timestamp > latest.timestamp:
                latest = Entry(
                    x_region,
                    z_region,
                    x_chunk,
                    z_chunk,
                    x_block,
                    z_block,
                    timestamp,
                )

        assert latest is not None, "expected at least one line in lines"
        chunks.append(latest)

    if args.last_update:
        chunks.sort(key=lambda e: e.timestamp, reverse=args.reverse)
    else:
        chunks.sort(
            key=lambda e: (e.block_x, e.block_x),
            reverse=args.reverse,
        )

    sys.stdout.write(
        "Region X,Region Z,Chunk X,Chunk Z,Block X,Block Z,Last Update\n"
    )
    sys.stdout.write("\n".join([e.to_row() for e in chunks]) + "\n")


def build_cli() -> argparse.ArgumentParser:
    """Defines the command-line interface's available options and arguments."""
    args = argparse.ArgumentParser()

    args.add_argument(
        "path",
        type=pathlib.Path,
        help="Path to a Minecraft server's `region` directory",
    )

    args.add_argument(
        "--last-update",
        action="store_true",
        help="Sort by last update instead of region.",
    )

    args.add_argument(
        "--reverse",
        action="store_false",
        help="Reverse all of the elements",
    )

    return args


def parse_file_name(file_name: str) -> tuple[int, int] | None:
    """Extracts the region's X and Z coordinates from the file name.

    Returns:
        A tuple containing the X and Z coordinates, or `None` if `file_name`
        was not in the correct format (i.e., `r.<x>.<z>.mca`).
    """
    if (match := FILE_NAME_PATTERN.match(file_name)) is None:
        return None

    raw_x = match.group(1)

    try:
        x = int(raw_x, 10)
    except ValueError as err:
        raise RuntimeError("invalid file name format: r.<x>.<z>.mca") from err

    raw_z = match.group(2)

    try:
        z = int(raw_z, 10)
    except ValueError as err:
        raise RuntimeError("invalid file name format: r.<x>.<z>.mca") from err

    return (x, z)


if __name__ == "__main__":
    sys.exit(main())
