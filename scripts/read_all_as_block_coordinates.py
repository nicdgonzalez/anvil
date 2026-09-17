#!/usr/bin/env python3

"""
Reads all of the files in a `region` directory using `anvil last-update`
and converts all of the relative chunk coordinates into absolute
block coordinates.

This example script is not very practical, as the goal is not (currently)
to purge individual chunks, but rather full region files.
"""

import argparse
import pathlib
import re
import subprocess
import sys
from typing import cast

FILE_NAME_PATTERN = re.compile(r"^r\.(-?\d+)\.(-?\d+)\.mca$")


def main() -> None:
    args = build_cli().parse_args()
    path = cast(pathlib.Path, args.path)

    chunks: list[tuple[int, int, str]] = []

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

        for line in lines:
            chunk_coordinates, timestamp = line.split(": ", maxsplit=1)
            x_chunk_raw, z_chunk_raw = chunk_coordinates.split(",", maxsplit=1)

            # Calculate the chunk's position within the Minecraft world.
            x_chunk = (x_region * 32) + int(x_chunk_raw, 10)
            z_chunk = (z_region * 32) + int(z_chunk_raw, 10)

            # Block coordinates are probably more intuitive.
            x_block = x_chunk * 16
            z_block = z_chunk * 16

            chunks.append((x_block, z_block, timestamp))

    chunks.sort(key=lambda d: (d[0], d[1]))
    sys.stdout.write("\n".join([f"{x},{z},{t}" for x, z, t in chunks]) + "\n")


def build_cli() -> argparse.ArgumentParser:
    """Defines the command-line interface's available options and arguments."""
    args = argparse.ArgumentParser()

    args.add_argument(
        "path",
        type=pathlib.Path,
        help="Path to a Minecraft server's `region` directory",
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
