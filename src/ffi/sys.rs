#[cfg(not(target_family = "wasm"))]
pub fn tick() -> u32 {
    0
}
#[cfg(target_family = "wasm")]
pub fn tick() -> u32 {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/sys")]
        extern "C" {
            fn tick() -> u32;
        }
        tick()
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn rand() -> u32 {
    0
}
#[cfg(target_family = "wasm")]
pub fn rand() -> u32 {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/sys")]
        extern "C" {
            fn rand() -> u32;
        }
        rand()
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn log(ptr: *const u8, len: u32) {}
#[cfg(target_family = "wasm")]
pub fn log(ptr: *const u8, len: u32) {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/sys")]
        extern "C" {
            fn log(ptr: *const u8, len: u32);
        }
        log(ptr, len)
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn env_get(
    key_ptr: *const u8,
    key_len: u32,
    out_var_ptr: *mut u8,
    out_var_len: *mut u32,
) -> u32 {
    0
}
#[cfg(target_family = "wasm")]
pub fn env_get(
    key_ptr: *const u8,
    key_len: u32,
    out_var_ptr: *mut u8,
    out_var_len: *mut u32,
) -> u32 {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/sys")]
        extern "C" {
            fn env_get(
                key_ptr: *const u8,
                key_len: u32,
                out_var_ptr: *mut u8,
                out_var_len: *mut u32,
            ) -> u32;
        }
        env_get(key_ptr, key_len, out_var_ptr, out_var_len)
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn resolution() -> u32 {
    0
}
#[cfg(target_family = "wasm")]
pub fn resolution() -> u32 {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/sys")]
        extern "C" {
            fn resolution() -> u32;
        }
        resolution()
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn save(ptr: *const u8, len: u32) -> i32 {
    -1
}
#[cfg(target_family = "wasm")]
pub fn save(ptr: *const u8, len: u32) -> i32 {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/sys")]
        extern "C" {
            fn save(ptr: *const u8, len: u32) -> i32;
        }
        save(ptr, len)
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn load(ptr: *mut u8, len: *mut u32) -> i32 {
    return -1;
}
#[cfg(target_family = "wasm")]
pub fn load(ptr: *mut u8, len: *mut u32) -> i32 {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/sys")]
        extern "C" {
            fn load(ptr: *mut u8, len: *mut u32) -> i32;
        }
        load(ptr, len)
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn set_local_storage(ptr: *const u8, len: u32) -> i32 {
    -1
}
#[cfg(target_family = "wasm")]
pub fn set_local_storage(ptr: *const u8, len: u32) -> i32 {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/sys")]
        extern "C" {
            fn set_local_storage(ptr: *const u8, len: u32) -> i32;
        }
        set_local_storage(ptr, len)
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn get_local_storage(ptr: *mut u8, len: *mut u32) -> i32 {
    return -1;
}
#[cfg(target_family = "wasm")]
pub fn get_local_storage(ptr: *mut u8, len: *mut u32) -> i32 {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/sys")]
        extern "C" {
            fn get_local_storage(ptr: *mut u8, len: *mut u32) -> i32;
        }
        get_local_storage(ptr, len)
    }
}
