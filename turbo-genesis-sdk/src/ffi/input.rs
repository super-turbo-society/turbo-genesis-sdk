#[cfg(not(target_family = "wasm"))]
pub fn gamepad(player: u32, out_ptr: *mut u8) {}
#[cfg(target_family = "wasm")]
pub fn gamepad(player: u32, out_ptr: *mut u8) {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/input")]
        extern "C" {
            fn gamepad(player: u32, out_ptr: *mut u8);
        }
        return gamepad(player, out_ptr);
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn mouse(out_ptr: *mut u8) {}
#[cfg(target_family = "wasm")]
pub fn mouse(out_ptr: *mut u8) {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/input")]
        extern "C" {
            fn mouse(out_ptr: *mut u8);
        }
        mouse(out_ptr)
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn wheel(out_ptr: *mut u8) {}
#[cfg(target_family = "wasm")]
pub fn wheel(out_ptr: *mut u8) {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/input")]
        extern "C" {
            fn wheel(out_ptr: *mut u8);
        }
        wheel(out_ptr)
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn keyboard(data_ptr: *mut u8, len_ptr: *mut u32) {}
#[cfg(target_family = "wasm")]
pub fn keyboard(data_ptr: *mut u8, len_ptr: *mut u32) {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/input")]
        extern "C" {
            fn keyboard(data_ptr: *mut u8, len_ptr: *mut u32);
        }
        return keyboard(data_ptr, len_ptr);
    }
}
