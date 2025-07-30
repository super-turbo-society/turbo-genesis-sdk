#[allow(clashing_extern_declarations)]
#[link(wasm_import_module = "@turbo_genesis/sys")]
unsafe extern "C" {
    #[link_name = "emit"]
    unsafe fn _emit(name_ptr: *const u8, name_len: u32, data_ptr: *const u8, data_len: u32);
    #[link_name = "tick"]
    unsafe fn _tick() -> u32;
    #[link_name = "rand"]
    unsafe fn _rand() -> u32;
    #[link_name = "log"]
    unsafe fn _log(ptr: *const u8, len: u32);
    #[link_name = "env_get"]
    unsafe fn _env_get(
        key_ptr: *const u8,
        key_len: u32,
        out_var_ptr: *mut u8,
        out_var_len: *mut u32,
    ) -> u32;
    #[link_name = "resolution"]
    unsafe fn _resolution() -> u32;
    #[link_name = "save"]
    unsafe fn _save(ptr: *const u8, len: u32) -> i32;
    #[link_name = "load"]
    unsafe fn _load(ptr: *mut u8, len: *mut u32) -> i32;
    #[link_name = "set_local_storage"]
    unsafe fn _set_local_storage(ptr: *const u8, len: u32) -> i32;
    #[link_name = "get_local_storage"]
    unsafe fn _get_local_storage(ptr: *mut u8, len: *mut u32) -> i32;
    #[link_name = "set_internal_storage"]
    unsafe fn _set_internal_storage(key_ptr: *const u8, key_len: u32, data_ptr: *const u8, data_len: u32) -> i32;
    #[link_name = "get_internal_storage"]
    unsafe fn _get_internal_storage(key_ptr: *const u8, key_len: u32, data_ptr: *mut u8, data_len: *mut u32) -> i32;
    #[link_name = "millis_since_unix_epoch"]
    unsafe fn _millis_since_unix_epoch() -> u64;
}

pub fn emit(name_ptr: *const u8, name_len: u32, data_ptr: *const u8, data_len: u32) {
    unsafe { _emit(name_ptr, name_len, data_ptr, data_len) }
}

pub fn tick() -> u32 {
    unsafe { _tick() }
}

pub fn rand() -> u32 {
    unsafe { _rand() }
}

pub fn log(ptr: *const u8, len: u32) {
    unsafe { _log(ptr, len) }
}

pub fn env_get(
    key_ptr: *const u8,
    key_len: u32,
    out_var_ptr: *mut u8,
    out_var_len: *mut u32,
) -> u32 {
    unsafe { _env_get(key_ptr, key_len, out_var_ptr, out_var_len) }
}

pub fn resolution() -> u32 {
    unsafe { _resolution() }
}

pub fn save(ptr: *const u8, len: u32) -> i32 {
    unsafe { _save(ptr, len) }
}

pub fn load(ptr: *mut u8, len: *mut u32) -> i32 {
    unsafe { _load(ptr, len) }
}

pub fn set_local_storage(ptr: *const u8, len: u32) -> i32 {
    unsafe { _set_local_storage(ptr, len) }
}

pub fn get_local_storage(ptr: *mut u8, len: *mut u32) -> i32 {
    unsafe { _get_local_storage(ptr, len) }
}

pub fn set_internal_storage(key_ptr: *const u8, key_len: u32, data_ptr: *const u8, data_len: u32) -> i32 {
    unsafe { _set_internal_storage(key_ptr, key_len, data_ptr, data_len) }
}

pub fn get_internal_storage(key_ptr: *const u8, key_len: u32, data_ptr: *mut u8, data_len: *mut u32) -> i32 {
    unsafe { _get_internal_storage(key_ptr, key_len, data_ptr, data_len) }
}

pub fn millis_since_unix_epoch() -> u64 {
    unsafe { _millis_since_unix_epoch() }
}
