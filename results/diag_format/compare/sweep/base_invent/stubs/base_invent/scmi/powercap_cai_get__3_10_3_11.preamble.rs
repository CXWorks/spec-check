use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct PowercapCaiGetState {
    pub domain_id: uint32,
    pub cpli: uint32,
    pub cai: uint32,
    pub status: int32,
}

pub struct S {
    pub powercap_cai_get: PowercapCaiGetState,
}

} // verus!
