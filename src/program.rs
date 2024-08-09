#[link(wasm_import_module = "turbo")]
extern "C" {
    #[link_name = "random_bytes"]
    fn turbo_os_random_bytes(ptr: *mut u8, len: usize) -> usize;

    #[link_name = "get_user_id_len"]
    fn turbo_os_get_user_id_len() -> usize;

    #[link_name = "get_user_id"]
    fn turbo_os_get_user_id(ptr: *mut u8) -> usize;

    #[link_name = "get_input_data_len"]
    fn turbo_os_get_input_data_len() -> usize;

    #[link_name = "get_input_data"]
    fn turbo_os_get_input_data(ptr: *mut u8) -> usize;

    #[link_name = "log"]
    fn turbo_os_log(ptr: *const u8, len: usize) -> usize;

    #[link_name = "read_file"]
    fn turbo_os_read_file(
        filepath_ptr: *const u8,
        filepath_len: usize,
        data_ptr: *mut u8,
        data_len: *mut usize,
    ) -> usize;

    #[link_name = "write_file"]
    fn turbo_os_write_file(
        filepath_ptr: *const u8,
        filepath_len: usize,
        data_ptr: *const u8,
        data_len: usize,
    ) -> usize;
}

pub const COMMIT: usize = 0;

pub const CANCEL: usize = 1;

pub fn get_user_id() -> String {
    let mut user_id = vec![0; unsafe { turbo_os_get_user_id_len() }];
    unsafe { turbo_os_get_user_id(user_id.as_mut_ptr()) };
    String::from_utf8(user_id).expect("Invalid UTF-8 sequence")
}

pub fn get_input_data() -> Vec<u8> {
    let mut input = vec![0; unsafe { turbo_os_get_input_data_len() }];
    unsafe { turbo_os_get_input_data(input.as_mut_ptr()) };
    input
}

pub fn log(message: &str) {
    unsafe { turbo_os_log(message.as_ptr(), message.len()) };
}

pub fn read_file(filepath: &str) -> Result<Vec<u8>, &'static str> {
    let mut data = vec![0; 4096];
    let mut data_len = 0;
    let err = unsafe {
        turbo_os_read_file(
            filepath.as_ptr(),
            filepath.len(),
            data.as_mut_ptr(),
            &mut data_len,
        )
    };
    if err != 0 {
        log(&format!("No data for file {}", filepath));
        return Err("File not found");
    }
    Ok(data[..data_len].to_vec())
}

pub fn write_file(filepath: &str, data: &[u8]) -> Result<(), &'static str> {
    let err = unsafe {
        turbo_os_write_file(filepath.as_ptr(), filepath.len(), data.as_ptr(), data.len())
    };
    if err != 0 {
        log(&format!("Could not update file {}", filepath));
        return Err("Failed to write file");
    }
    return Ok(());
}

pub fn random_number<T: Default + Copy>() -> T {
    let len = std::mem::size_of::<T>();
    let buf: &mut [u8; 32] = &mut [0u8; 32];
    unsafe { turbo_os_random_bytes(buf.as_mut_ptr(), len) };
    let mut arr = [0u8; 32];
    arr[..len].copy_from_slice(&buf[..len]);
    unsafe { std::ptr::read_unaligned(arr.as_ptr() as *const T) }
}
