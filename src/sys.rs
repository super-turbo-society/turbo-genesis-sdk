use crate::ffi;

pub fn tick() -> usize {
    ffi::sys::tick() as usize
}

pub fn rand() -> u32 {
    ffi::sys::rand()
}

pub fn log(text: &str) {
    let ptr = text.as_ptr();
    let len = text.len() as u32;
    ffi::sys::log(ptr, len)
}

#[macro_export]
macro_rules! log {
    ($fmt:expr $(, $($arg:tt)*)?) => { $crate::sys::log(&format!($fmt, $($($arg)*)?)) };
}

/// @deprecated - use $crate::canvas::canvas_size
pub fn resolution() -> [u32; 2] {
    let res = ffi::sys::resolution();
    let w = res & 0xffff;
    let h = res >> 16;
    [w, h]
}

pub fn save(data: &[u8]) -> Result<i32, i32> {
    let ptr = data.as_ptr();
    let len = data.len() as u32;
    let n = ffi::sys::save(ptr, len);
    // If n is < 0, it's an error code
    if n < 0 {
        return Err(n);
    }
    // Otherwise, it's remaining storage bytes
    Ok(n)
}

pub fn load() -> Result<&'static [u8], i32> {
    unsafe {
        // Allocate a big buffer for reading/writing save data
        static mut TURBO_SAVE_DATA: [u8; 4096 * 1000] = [0; 4096 * 1000];
        let ptr = TURBO_SAVE_DATA.as_mut_ptr();
        let mut len = 0;
        let n = ffi::sys::load(ptr, &mut len);
        // crate::println!("len: {}", len);
        // If n is < 0, it's an error code
        if n < 0 {
            return Err(n);
        }
        Ok(&TURBO_SAVE_DATA[..len as usize])
    }
}

pub mod time {
    pub fn now() -> u64 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/sys")]
            extern "C" {
                fn millis_since_unix_epoch() -> u64;
            }
            millis_since_unix_epoch()
        }
    }
}
