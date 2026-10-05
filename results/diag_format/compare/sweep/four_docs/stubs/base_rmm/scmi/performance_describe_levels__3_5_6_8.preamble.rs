use vstd::prelude::*;

verus! {

pub type UInt = u64;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int32 = i32;

pub struct Array<T, const N: usize> {
    pub seq: Seq<T>,
}

pub enum RmiStatusCode {
    RmiSuccess,
    RmiErrorInput,
    RmiErrorRealm,
    RmiErrorRec,
    RmiErrorRtt,
    RmiErrorInUse,
}

pub struct S {
    pub num_perf_domains: UInt32,
    pub perf_levels: Seq<UInt32>,
    pub regs: Seq<UInt>,
}

pub spec const RMI_SUCCESS: RmiStatusCode = RmiStatusCode::RmiSuccess;
pub spec const RMI_ERROR_INPUT: RmiStatusCode = RmiStatusCode::RmiErrorInput;
pub spec const RMI_ERROR_REALM: RmiStatusCode = RmiStatusCode::RmiErrorRealm;
pub spec const RMI_ERROR_REC: RmiStatusCode = RmiStatusCode::RmiErrorRec;
pub spec const RMI_ERROR_RTT: RmiStatusCode = RmiStatusCode::RmiErrorRtt;
pub spec const RMI_ERROR_IN_USE: RmiStatusCode = RmiStatusCode::RmiErrorInUse;

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = -1;
pub spec const INVALID_PARAMETERS: Int32 = -2;
pub spec const DENIED: Int32 = -3;
pub spec const NOT_FOUND: Int32 = -4;
pub spec const OUT_OF_RANGE: Int32 = -5;
pub spec const BUSY: Int32 = -6;
pub spec const COMMS_ERROR: Int32 = -7;
pub spec const GENERIC_ERROR: Int32 = -8;
pub spec const HARDWARE_ERROR: Int32 = -9;
pub spec const PROTOCOL_ERROR: Int32 = -10;

pub open spec fn IsValidPerfDomain(domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

} // verus!
