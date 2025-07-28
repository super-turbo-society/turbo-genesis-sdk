pub fn emit(_: *const u8, _: u32, _: *const u8, _: u32) {}

pub fn tick() -> u32 {
    0
}

pub fn rand() -> u32 {
    0
}

pub fn log(_: *const u8, _: u32) {}

pub fn env_get(_: *const u8, _: u32, _: *mut u8, _: *mut u32) -> u32 {
    0
}

pub fn resolution() -> u32 {
    0
}

pub fn save(_: *const u8, _: u32) -> i32 {
    -1
}

pub fn load(_: *mut u8, _: *mut u32) -> i32 {
    -1
}

pub fn set_local_storage(_: *const u8, _: u32) -> i32 {
    -1
}

pub fn get_local_storage(_: *mut u8, _: *mut u32) -> i32 {
    -1
}

pub fn millis_since_unix_epoch() -> u64 {
    0
}
