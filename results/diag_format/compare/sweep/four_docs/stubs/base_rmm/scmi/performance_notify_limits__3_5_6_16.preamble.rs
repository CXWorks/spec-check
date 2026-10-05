use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = Seq<u32>;

pub type AgentId = u32;

pub struct S {
    pub dummy: u32,
}

pub spec const agent: AgentId = 0;

pub spec const SUCCESS: Int32 = 0i32;

pub spec const NOT_SUPPORTED: Int32 = -1i32;

pub spec const INVALID_PARAMETERS: Int32 = -2i32;

pub spec const NOT_FOUND: Int32 = -4i32;

pub open spec fn IsValidPerfDomain(domain_id: UInt32) -> bool;

pub open spec fn PerfDomainSupportsLimitsNotify(domain_id: UInt32) -> bool;

pub open spec fn IsValidNotifyEnable(notify_enable: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn PerfLimitsNotifyEnabled(a: AgentId, domain_id: UInt32) -> bool;

} // verus!
