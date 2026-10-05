use vstd::prelude::*;
verus! {

pub const SBI_SUCCESS: i64 = 0;

pub struct sbiret {
    pub error: i64,
    pub uvalue: int,
}

pub struct S {
    pub mpxy_response_data_len: u64,
    pub mpxy_response_data: Seq<u8>,
    pub mpxy_request_data: Seq<u8>,
}

} // verus!
