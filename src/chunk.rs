use crate::block::{BLOCK_Y_MAX, BLOCK_Y_MIN, Block};
use crate::floor_i64;
use crate::region::Region;

pub const CHUNK_LENGTH_IN_BLOCKS: i64 = 16;

pub const CHUNK_Y_MIN: i64 = BLOCK_Y_MIN / CHUNK_LENGTH_IN_BLOCKS;
pub const CHUNK_Y_MAX: i64 = BLOCK_Y_MAX / CHUNK_LENGTH_IN_BLOCKS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Chunk {
    x: i64,
    y: i64,
    z: i64,
}

impl Chunk {
    /// Creates a new [`Chunk`] that is known to be at a valid coordinate.
    ///
    /// Returns [`None`] if `y` is outside of Minecraft's build limit.
    #[must_use]
    pub const fn new(x: i64, y: i64, z: i64) -> Option<Self> {
        if y < CHUNK_Y_MIN || y > CHUNK_Y_MAX {
            return None;
        }

        Some(Self { x, y, z })
    }

    /// Creates a new [`Chunk`] from the given [`Block`].
    #[must_use]
    pub const fn from_block(block: Block) -> Self {
        Self {
            x: floor_i64(block.x(), CHUNK_LENGTH_IN_BLOCKS),
            y: floor_i64(block.y(), CHUNK_LENGTH_IN_BLOCKS),
            z: floor_i64(block.z(), CHUNK_LENGTH_IN_BLOCKS),
        }
    }

    /// Creates a new [`Region`] from `self`.
    #[must_use]
    pub const fn into_region(self) -> Region {
        Region::from_chunk(self)
    }

    /// Returns the chunk's X coordinate.
    #[must_use]
    pub const fn x(&self) -> i64 {
        self.x
    }

    /// Returns the chunk's Y coordinate.
    #[must_use]
    pub const fn y(&self) -> i64 {
        self.y
    }

    /// Returns the chunk's Z coordinate.
    #[must_use]
    pub const fn z(&self) -> i64 {
        self.z
    }
}
