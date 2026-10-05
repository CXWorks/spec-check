use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn IsValidPowerDomain(domain_id: UInt32) -> bool;

pub open spec fn domain_id(s: S) -> UInt32;

pub open spec fn caller(s: S) -> UInt32;

pub open spec fn notify_enable(s: S) -> Seq<UInt32>;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn PowerStateChangeRequestedNotifyEnabled(agent_id: UInt32, domain_id: UInt32) -> UInt32;

} // verus!
