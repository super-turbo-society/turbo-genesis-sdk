use borsh::BorshDeserialize;

#[derive(Debug, Clone, PartialEq, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct HttpRequest {
    pub method: String,
    pub uri: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub enum HttpRequestStatus {
    Idle,
    Pending,
    Success,
    Fail,
}

pub fn get(uri: &str) -> (u64, HttpRequestStatus, Option<HttpResponse>) {
    let req = borsh::to_vec(&HttpRequest {
        uri: uri.to_string(),
        method: "GET".to_string(),
        headers: vec![],
        body: None,
    })
    .unwrap();
    let res = &mut [0; 8192];
    let mut res_len = 0;
    let mut status = 0;
    let req_id = unsafe {
        #[link(wasm_import_module = "@turbo_genesis/http")]
        extern "C" {
            fn poll_request(
                req_ptr: *const u8,
                req_len: u32,
                res_ptr: *mut u8,
                res_len_ptr: *mut u32,
                status_ptr: *mut u32,
            ) -> u64;
        }
        poll_request(
            req.as_ptr(),
            req.len() as u32,
            res.as_mut_ptr(),
            &mut res_len,
            &mut status,
        )
    };
    let status = HttpRequestStatus::try_from_slice(&[status as u8]).unwrap();
    let res = if res_len > 0 {
        Some(HttpResponse::try_from_slice(&res[..res_len as usize]).unwrap())
    } else {
        None
    };
    (req_id, status, res)
}
