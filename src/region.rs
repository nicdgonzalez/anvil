use crate::block::Block;
use crate::chunk::{CHUNK_LENGTH_IN_BLOCKS, Chunk};
use crate::floor_i64;

pub const REGION_LENGTH_IN_CHUNKS: i64 = 32;
pub const REGION_LENGTH_IN_BLOCKS: i64 = REGION_LENGTH_IN_CHUNKS * CHUNK_LENGTH_IN_BLOCKS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Region {
    x: i64,
    z: i64,
}

impl Region {
    /// Creates a new [`Region`] from the given coordinates.
    #[must_use]
    pub const fn new(x: i64, z: i64) -> Self {
        Self { x, z }
    }

    /// Creates a new [`Region`] from the given [`Chunk`].
    #[must_use]
    pub const fn from_chunk(chunk: Chunk) -> Self {
        Self {
            x: floor_i64(chunk.x(), REGION_LENGTH_IN_CHUNKS),
            z: floor_i64(chunk.z(), REGION_LENGTH_IN_CHUNKS),
        }
    }

    /// Creates a new [`Region`] from the given [`Block`].
    #[must_use]
    pub const fn from_block(block: Block) -> Self {
        Self {
            x: floor_i64(block.x(), REGION_LENGTH_IN_BLOCKS),
            z: floor_i64(block.z(), REGION_LENGTH_IN_BLOCKS),
        }
    }

    /// Returns the region's X coordinate.
    #[must_use]
    pub const fn x(&self) -> i64 {
        self.x
    }

    /// Returns the region's Z coordinate.
    #[must_use]
    pub const fn z(&self) -> i64 {
        self.z
    }
}
