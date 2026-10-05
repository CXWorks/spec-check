use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub enum RmiStatusCode {
    RmiSuccess,
    RmiErrorInput,
    RmiErrorRealm,
    RmiErrorRec,
    RmiErrorRtt,
}

pub struct S {
    pub dummy: u64,
}

pub spec const RMI_ERROR_INPUT: Result<(), RmiStatusCode> = Err(RmiStatusCode::RmiErrorInput);

pub spec const RMI_ERROR_REALM: Result<(), RmiStatusCode> = Err(RmiStatusCode::RmiErrorRealm);

pub spec const RMI_ERROR_REC: Result<(), RmiStatusCode> = Err(RmiStatusCode::RmiErrorRec);

pub spec const RMI_ERROR_RTT: Result<(), RmiStatusCode> = Err(RmiStatusCode::RmiErrorRtt);

} // verus!
