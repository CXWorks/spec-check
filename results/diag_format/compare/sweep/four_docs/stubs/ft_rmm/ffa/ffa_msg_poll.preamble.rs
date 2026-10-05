use vstd::prelude::*;
verus! {

pub type Result = u64;
pub type Caller = u64;
pub type FfaFunction = u32;
pub type Instance = u16;

pub struct Buffer {
    pub id: u64,
}

pub struct S {
    pub state_id: u64,
}

pub const RETRY: Result = 1;
pub const DENIED: Result = 2;
pub const NOT_SUPPORTED: Result = 3;
pub const FFA_MSG_SEND: Result = 4;
pub const FFA_SUCCESS: Result = 5;

pub const FFA_MSG_POLL: FfaFunction = 0x8400006A;

pub const ffa_instance: Instance = 1;

pub open spec fn MessageAvailable(s: S, buf: Buffer) -> bool;

pub open spec fn RxBuffer(s: S, caller: Caller) -> Buffer;

pub open spec fn ResultEqual(r1: Result, r2: Result) -> bool;

pub open spec fn CalleeCanHandleRequest(s: S) -> bool;

pub open spec fn IsImplemented(s: S, func: FfaFunction, inst: Instance) -> bool;

} // verus!
