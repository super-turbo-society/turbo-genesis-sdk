use crate::ffi;

#[macro_export]
macro_rules! log {
    ($fmt:expr $(, $($arg:tt)*)?) => { $crate::sys::log(&format!($fmt, $($($arg)*)?)) };
}

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

pub fn emit(name: &str, data: &str) {
    let name_ptr = name.as_ptr();
    let name_len = name.len() as u32;
    let data_ptr = data.as_ptr();
    let data_len = data.len() as u32;
    ffi::sys::emit(name_ptr, name_len, data_ptr, data_len)
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

#[allow(static_mut_refs)]
pub fn load() -> Result<&'static [u8], i32> {
    unsafe {
        // Allocate a big buffer for reading/writing save data
        static mut TURBO_SAVE_DATA: [u8; 4096 * 1000] = [0; 4096 * 1000];
        let ptr = TURBO_SAVE_DATA.as_mut_ptr();
        let mut len = 0;
        let n = ffi::sys::load(ptr, &mut len);
        // If n is < 0, it's an error code
        if n < 0 {
            return Err(n);
        }
        Ok(&TURBO_SAVE_DATA[..len as usize])
    }
}

pub mod local {
    use crate::ffi;
    pub fn save(data: &[u8]) -> Result<i32, i32> {
        let ptr = data.as_ptr();
        let len = data.len() as u32;
        let n = ffi::sys::set_local_storage(ptr, len);
        // If n is > 0, it's an error code
        if n > 0 {
            return Err(n);
        }
        Ok(n)
    }
    pub fn load() -> Result<Vec<u8>, i32> {
        unsafe {
            // Allocate a 5mb buffer
            let mut data = vec![0; 1048576 * 5];
            let ptr = data.as_mut_ptr();
            let mut len = 0;
            let n = ffi::sys::get_local_storage(ptr, &mut len);
            // If n is > 0, it's an error code
            if n > 0 {
                return Err(n);
            }
            data.truncate(len as usize);
            Ok(data)
        }
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

pub mod env {
    use super::*;
    #[allow(static_mut_refs)]
    pub fn get(key: &str) -> String {
        unsafe {
            let key_ptr = key.as_ptr();
            let key_len = key.len() as u32;
            // Env data limit 1024 bytes
            let mut data = [0; 1024];
            let out_var_ptr = data.as_mut_ptr();
            let mut out_var_len = 0;
            ffi::sys::env_get(key_ptr, key_len, out_var_ptr, &mut out_var_len);
            if out_var_len == 0 {
                return String::new();
            }
            let s = String::from_utf8(data[..(out_var_len as usize)].to_vec()).unwrap_or_default();
            s
        }
    }
}
