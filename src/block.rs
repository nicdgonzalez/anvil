use crate::chunk::Chunk;
use crate::region::Region;

// Minecraft's build limit ranges from [-64, 320).
pub const BLOCK_Y_MIN: i64 = -64;
pub const BLOCK_Y_MAX: i64 = 319;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Block {
    x: i64,
    y: i64,
    z: i64,
}

impl Block {
    /// Creates a new [`Block`] that is known to be at a valid coordinate.
    ///
    /// Returns [`None`] if `y` is outside of Minecraft's build limit.
    #[must_use]
    pub const fn new(x: i64, y: i64, z: i64) -> Option<Self> {
        if y < BLOCK_Y_MIN || y > BLOCK_Y_MAX {
            return None;
        }

        Some(Self { x, y, z })
    }

    /// Creates a new [`Chunk`] from `self`.
    #[must_use]
    pub const fn into_chunk(self) -> Chunk {
        Chunk::from_block(self)
    }

    /// Creates a new [`Region`] from `self`.
    #[must_use]
    pub const fn into_region(self) -> Region {
        Region::from_block(self)
    }

    /// Returns the block's X coordinate.
    #[must_use]
    pub const fn x(&self) -> i64 {
        self.x
    }

    /// Returns the block's Y coordinate.
    #[must_use]
    pub const fn y(&self) -> i64 {
        self.y
    }

    /// Returns the block's Z coordinate.
    #[must_use]
    pub const fn z(&self) -> i64 {
        self.z
    }
}
