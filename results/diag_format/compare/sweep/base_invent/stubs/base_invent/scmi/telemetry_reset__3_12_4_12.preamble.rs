use vstd::prelude::*;
verus! {

pub type int32 = i64;

pub struct S {
    pub telemetry_flags: u64,
    pub telemetry_data: u64,
    pub telemetry_config: u64,
}

} // verus!
