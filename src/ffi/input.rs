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
pub fn mouse(player: u32, out_ptr: *mut u8) {}
#[cfg(target_family = "wasm")]
pub fn mouse(player: u32, out_ptr: *mut u8) {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/input")]
        extern "C" {
            fn mouse(player: u32, out_ptr: *mut u8);
        }
        mouse(player, out_ptr)
    }
}
