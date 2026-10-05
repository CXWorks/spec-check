use vstd::prelude::*;
verus! {

pub type UInt32 = Seq<u32>;
pub type Int32 = i32;

pub struct CallerAgent {
    pub id: u32,
}

pub struct S {
    pub notify_flags: Map<u32, bool>,
}

pub const SUCCESS: Int32 = 0;
pub const INVALID_PARAMETERS: Int32 = 1;

pub open spec fn IsValidNotifyEnable(s: S, notify_enable: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn ErrorNotifyEnabled(s: S, caller_agent: CallerAgent) -> bool;

} // verus!
