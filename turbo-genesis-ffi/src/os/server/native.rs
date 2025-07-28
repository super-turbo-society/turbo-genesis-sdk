pub fn random_bytes(ptr: *mut u8, len: usize) -> usize {
    0
}

pub fn secs_since_unix_epoch() -> u32 {
    0
}

pub fn get_user_id_len() -> usize {
    0
}

pub fn get_user_id(ptr: *mut u8) -> usize {
    0
}

pub fn get_input_data_len() -> usize {
    0
}

pub fn get_input_data(ptr: *mut u8) -> usize {
    0
}

pub fn log(ptr: *const u8, len: usize) -> usize {
    0
}

pub fn read_file(
    filepath_ptr: *const u8,
    filepath_len: usize,
    data_ptr: *mut u8,
    data_len: *mut usize,
) -> usize {
    0
}

pub fn write_file(
    filepath_ptr: *const u8,
    filepath_len: usize,
    data_ptr: *const u8,
    data_len: usize,
) -> usize {
    0
}

pub fn emit_event(
    type_ptr: *const u8,
    data_len1: usize,
    data_ptr: *const u8,
    data_len2: usize,
) -> usize {
    0
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
    0
}

pub fn invoke_command(
    program_id_ptr: *const u8,
    program_id_len: usize,
    command_ptr: *const u8,
    command_len: usize,
    data_ptr: *const u8,
    data_len: usize,
) -> usize {
    0
}

pub fn channel_recv_with_timeout(
    msg_type_ptr: *mut u8,
    user_id_ptr: *mut u8,
    user_id_len_ptr: *mut usize,
    data_ptr: *mut u8,
    data_len_ptr: *mut usize,
    timeout_ms: u32,
) -> usize {
    0
}

pub fn channel_send(
    user_id_ptr: *const u8,
    user_id_len: usize,
    data_ptr: *const u8,
    data_len: usize,
) -> usize {
    0
}

pub fn channel_broadcast(data_ptr: *const u8, data_len: usize) -> usize {
    0
}
