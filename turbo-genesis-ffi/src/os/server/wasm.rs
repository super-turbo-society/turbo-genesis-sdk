#[allow(clashing_extern_declarations)]
#[link(wasm_import_module = "turbo")]
extern "C" {
    #[link_name = "random_bytes"]
    fn _random_bytes(ptr: *mut u8, len: usize) -> usize;
    #[link_name = "secs_since_unix_epoch"]
    fn _secs_since_unix_epoch() -> u32;
    #[link_name = "get_user_id_len"]
    fn _get_user_id_len() -> usize;
    #[link_name = "get_user_id"]
    fn _get_user_id(ptr: *mut u8) -> usize;
    #[link_name = "get_input_data_len"]
    fn _get_input_data_len() -> usize;
    #[link_name = "get_input_data"]
    fn _get_input_data(ptr: *mut u8) -> usize;
    #[link_name = "log"]
    fn _log(ptr: *const u8, len: usize) -> usize;
    #[link_name = "read_file"]
    fn _read_file(
        filepath_ptr: *const u8,
        filepath_len: usize,
        data_ptr: *mut u8,
        data_len: *mut usize,
    ) -> usize;
    #[link_name = "write_file"]
    fn _write_file(
        filepath_ptr: *const u8,
        filepath_len: usize,
        data_ptr: *const u8,
        data_len: usize,
    ) -> usize;
    #[link_name = "emit_event"]
    fn _emit_event(
        type_ptr: *const u8,
        data_len: usize,
        data_ptr: *const u8,
        data_len: usize,
    ) -> usize;
    #[link_name = "enqueue_command"]
    fn _enqueue_command(
        program_id_ptr: *const u8,
        program_id_len: usize,
        command_ptr: *const u8,
        command_len: usize,
        data_ptr: *const u8,
        data_len: usize,
        nonce_ptr: *const u8,
        delay: u32,
        hash_out_ptr: *mut u8,
    ) -> usize;
    #[link_name = "invoke_command"]
    fn _invoke_command(
        program_id_ptr: *const u8,
        program_id_len: usize,
        command_ptr: *const u8,
        command_len: usize,
        data_ptr: *const u8,
        data_len: usize,
    ) -> usize;
    #[link_name = "channel_recv"]
    fn _channel_recv_with_timeout(
        msg_type_ptr: *mut u8,
        user_id_ptr: *mut u8,
        user_id_len_ptr: *mut usize,
        data_ptr: *mut u8,
        data_len_ptr: *mut usize,
        timeout_ms: u32,
    ) -> usize;
    #[link_name = "channel_send"]
    fn _channel_send(
        user_id_ptr: *const u8,
        user_id_len: usize,
        data_ptr: *const u8,
        data_len: usize,
    ) -> usize;
    #[link_name = "channel_broadcast"]
    fn _channel_broadcast(data_ptr: *const u8, data_len: usize) -> usize;
}

pub fn random_bytes(ptr: *mut u8, len: usize) -> usize {
    unsafe { _random_bytes(ptr, len) }
}

pub fn secs_since_unix_epoch() -> u32 {
    unsafe { _secs_since_unix_epoch() }
}

pub fn get_user_id_len() -> usize {
    unsafe { _get_user_id_len() }
}

pub fn get_user_id(ptr: *mut u8) -> usize {
    unsafe { _get_user_id(ptr) }
}

pub fn get_input_data_len() -> usize {
    unsafe { _get_input_data_len() }
}

pub fn get_input_data(ptr: *mut u8) -> usize {
    unsafe { _get_input_data(ptr) }
}

pub fn log(ptr: *const u8, len: usize) -> usize {
    unsafe { _log(ptr, len) }
}

pub fn read_file(
    filepath_ptr: *const u8,
    filepath_len: usize,
    data_ptr: *mut u8,
    data_len: *mut usize,
) -> usize {
    unsafe { _read_file(filepath_ptr, filepath_len, data_ptr, data_len) }
}

pub fn write_file(
    filepath_ptr: *const u8,
    filepath_len: usize,
    data_ptr: *const u8,
    data_len: usize,
) -> usize {
    unsafe { _write_file(filepath_ptr, filepath_len, data_ptr, data_len) }
}

pub fn emit_event(
    type_ptr: *const u8,
    data_len1: usize,
    data_ptr: *const u8,
    data_len2: usize,
) -> usize {
    unsafe { _emit_event(type_ptr, data_len1, data_ptr, data_len2) }
}

pub fn enqueue_command(
    program_id_ptr: *const u8,
    program_id_len: usize,
    command_ptr: *const u8,
    command_len: usize,
    data_ptr: *const u8,
    data_len: usize,
    nonce_ptr: *const u8,
    delay: u32,
    hash_out_ptr: *mut u8,
) -> usize {
    unsafe {
        _enqueue_command(
            program_id_ptr,
            program_id_len,
            command_ptr,
            command_len,
            data_ptr,
            data_len,
            nonce_ptr,
            delay,
            hash_out_ptr,
        )
    }
}

pub fn invoke_command(
    program_id_ptr: *const u8,
    program_id_len: usize,
    command_ptr: *const u8,
    command_len: usize,
    data_ptr: *const u8,
    data_len: usize,
) -> usize {
    unsafe {
        _invoke_command(
            program_id_ptr,
            program_id_len,
            command_ptr,
            command_len,
            data_ptr,
            data_len,
        )
    }
}

pub fn channel_recv_with_timeout(
    msg_type_ptr: *mut u8,
    user_id_ptr: *mut u8,
    user_id_len_ptr: *mut usize,
    data_ptr: *mut u8,
    data_len_ptr: *mut usize,
    timeout_ms: u32,
) -> usize {
    unsafe {
        _channel_recv_with_timeout(
            msg_type_ptr,
            user_id_ptr,
            user_id_len_ptr,
            data_ptr,
            data_len_ptr,
            timeout_ms,
        )
    }
}

pub fn channel_send(
    user_id_ptr: *const u8,
    user_id_len: usize,
    data_ptr: *const u8,
    data_len: usize,
) -> usize {
    unsafe { _channel_send(user_id_ptr, user_id_len, data_ptr, data_len) }
}

pub fn channel_broadcast(data_ptr: *const u8, data_len: usize) -> usize {
    unsafe { _channel_broadcast(data_ptr, data_len) }
}
