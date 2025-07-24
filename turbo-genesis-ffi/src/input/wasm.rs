#[allow(clashing_extern_declarations)]
#[link(wasm_import_module = "@turbo_genesis/input")]
unsafe extern "C" {
    #[link_name = "gamepad"]
    unsafe fn _gamepad(player: u32, out_ptr: *mut u8);
    #[link_name = "mouse"]
    unsafe fn _mouse(out_ptr: *mut u8);
    #[link_name = "keyboard"]
    unsafe fn _keyboard(data_ptr: *mut u8, len_ptr: *mut u32);
}

pub fn gamepad(player: u32, out_ptr: *mut u8) {
    unsafe { _gamepad(player, out_ptr) }
}

pub fn mouse(out_ptr: *mut u8) {
    unsafe { _mouse(out_ptr) }
}

pub fn keyboard(data_ptr: *mut u8, len_ptr: *mut u32) {
    unsafe { _keyboard(data_ptr, len_ptr) }
}
