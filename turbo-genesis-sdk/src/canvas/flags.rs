// Repeats the sprite within the containing quad
pub const SPRITE_REPEAT: u32 = 1 << 0;

// Scales a sprite to fit the dimensions of the containing quad
pub const SPRITE_COVER: u32 = 1 << 1;

// Elements drawn with this flag will ignore camera position and zoom settings
pub const POSITION_FIXED: u32 = 1 << 2;
