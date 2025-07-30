#[allow(clashing_extern_declarations)]
#[link(wasm_import_module = "@turbo_genesis/turbo_os")]
unsafe extern "C" {
    #[link_name = "get_user_id"]
    unsafe fn _get_user_id(out_user_id_ptr: *mut u8, out_user_id_len_ptr: *mut u32) -> u32;
    #[link_name = "channel_is_connected"]
    unsafe fn _channel_is_connected(
        program_id_ptr: *const u8,
        program_id_len: u32,
        channel_kind_ptr: *const u8,
        channel_kind_len: u32,
        channel_id_ptr: *const u8,
        channel_id_len: u32,
    ) -> u32;
    #[link_name = "channel_connect"]
    unsafe fn _channel_connect(
        program_id_ptr: *const u8,
        program_id_len: u32,
        channel_kind_ptr: *const u8,
        channel_kind_len: u32,
        channel_id_ptr: *const u8,
        channel_id_len: u32,
    ) -> u32;
    #[link_name = "channel_recv"]
    unsafe fn _channel_recv(
        program_id_ptr: *const u8,
        program_id_len: u32,
        channel_kind_ptr: *const u8,
        channel_kind_len: u32,
        channel_id_ptr: *const u8,
        channel_id_len: u32,
        out_data_ptr: *const u8,
        out_data_len_ptr: *mut u32,
        out_err_ptr: *const u8,
        out_err_len_ptr: *mut u32,
    ) -> u32;
    #[link_name = "channel_send"]
    unsafe fn _channel_send(
        program_id_ptr: *const u8,
        program_id_len: u32,
        channel_kind_ptr: *const u8,
        channel_kind_len: u32,
        channel_id_ptr: *const u8,
        channel_id_len: u32,
        data_ptr: *const u8,
        data_len: u32,
        out_err_ptr: *const u8,
        out_err_len_ptr: *mut u32,
    ) -> u32;
    #[link_name = "watch_events"]
    unsafe fn _watch_events(
        program_id_ptr: *const u8,
        program_id_len: u32,
        event_type_ptr: *const u8,
        event_type_len: u32,
        out_data_ptr: *const u8,
        out_data_len_ptr: *mut u32,
        out_err_ptr: *const u8,
        out_err_len_ptr: *mut u32,
    ) -> u32;
    #[link_name = "read_file"]
    unsafe fn _read_file(
        program_id_ptr: *const u8,
        program_id_len: u32,
        filepath_ptr: *const u8,
        filepath_len: u32,
        query_ptr: *const u8,
        query_len: u32,
        out_data_ptr: *mut u8,
        out_data_len_ptr: *mut u32,
        out_err_ptr: *mut u8,
        out_err_len_ptr: *mut u32,
    ) -> u32;
    #[link_name = "exec"]
    unsafe fn _exec(
        program_id_ptr: *const u8,
        program_id_len: u32,
        command_ptr: *const u8,
        command_len: u32,
        data_ptr: *const u8,
        data_len: u32,
        tx_hash_ptr: *mut u8,
    ) -> u32;
}

pub fn get_user_id(out_user_id_ptr: *mut u8, out_user_id_len_ptr: *mut u32) -> u32 {
    unsafe { _get_user_id(out_user_id_ptr, out_user_id_len_ptr) }
}

pub fn channel_is_connected(
    program_id_ptr: *const u8,
    program_id_len: u32,
    channel_kind_ptr: *const u8,
    channel_kind_len: u32,
    channel_id_ptr: *const u8,
    channel_id_len: u32,
) -> u32 {
    unsafe {
        _channel_is_connected(
            program_id_ptr,
            program_id_len,
            channel_kind_ptr,
            channel_kind_len,
            channel_id_ptr,
            channel_id_len,
        )
    }
}

pub fn channel_connect(
    program_id_ptr: *const u8,
    program_id_len: u32,
    channel_kind_ptr: *const u8,
    channel_kind_len: u32,
    channel_id_ptr: *const u8,
    channel_id_len: u32,
) -> u32 {
    unsafe {
        _channel_connect(
            program_id_ptr,
            program_id_len,
            channel_kind_ptr,
            channel_kind_len,
            channel_id_ptr,
            channel_id_len,
        )
    }
}

pub fn channel_recv(
    program_id_ptr: *const u8,
    program_id_len: u32,
    channel_kind_ptr: *const u8,
    channel_kind_len: u32,
    channel_id_ptr: *const u8,
    channel_id_len: u32,
    out_data_ptr: *const u8,
    out_data_len_ptr: *mut u32,
    out_err_ptr: *const u8,
    out_err_len_ptr: *mut u32,
) -> u32 {
    unsafe {
        _channel_recv(
            program_id_ptr,
            program_id_len,
            channel_kind_ptr,
            channel_kind_len,
            channel_id_ptr,
            channel_id_len,
            out_data_ptr,
            out_data_len_ptr,
            out_err_ptr,
            out_err_len_ptr,
        )
    }
}

pub fn channel_send(
    program_id_ptr: *const u8,
    program_id_len: u32,
    channel_kind_ptr: *const u8,
    channel_kind_len: u32,
    channel_id_ptr: *const u8,
    channel_id_len: u32,
    data_ptr: *const u8,
    data_len: u32,
    out_err_ptr: *const u8,
    out_err_len_ptr: *mut u32,
) -> u32 {
    unsafe {
        _channel_send(
            program_id_ptr,
            program_id_len,
            channel_kind_ptr,
            channel_kind_len,
            channel_id_ptr,
            channel_id_len,
            data_ptr,
            data_len,
            out_err_ptr,
            out_err_len_ptr,
        )
    }
}

pub fn watch_events(
    program_id_ptr: *const u8,
    program_id_len: u32,
    event_type_ptr: *const u8,
    event_type_len: u32,
    out_data_ptr: *mut u8,
    out_data_len_ptr: *mut u32,
    out_err_ptr: *mut u8,
    out_err_len_ptr: *mut u32,
) -> u32 {
    unsafe {
        _watch_events(
            program_id_ptr,
            program_id_len,
            event_type_ptr,
            event_type_len,
            out_data_ptr,
            out_data_len_ptr,
            out_err_ptr,
            out_err_len_ptr,
        )
    }
}

pub fn read_file(
    program_id_ptr: *const u8,
    program_id_len: u32,
    filepath_ptr: *const u8,
    filepath_len: u32,
    query_ptr: *const u8,
    query_len: u32,
    out_data_ptr: *mut u8,
    out_data_len_ptr: *mut u32,
    out_err_ptr: *mut u8,
    out_err_len_ptr: *mut u32,
) -> u32 {
    unsafe {
        _read_file(
            program_id_ptr,
            program_id_len,
            filepath_ptr,
            filepath_len,
            query_ptr,
            query_len,
            out_data_ptr,
            out_data_len_ptr,
            out_err_ptr,
            out_err_len_ptr,
        )
    }
}

pub fn exec(
    program_id_ptr: *const u8,
    program_id_len: u32,
    command_ptr: *const u8,
    command_len: u32,
    data_ptr: *const u8,
    data_len: u32,
    tx_hash_ptr: *mut u8,
) -> u32 {
    unsafe {
        _exec(
            program_id_ptr,
            program_id_len,
            command_ptr,
            command_len,
            data_ptr,
            data_len,
            tx_hash_ptr,
        )
    }
}
