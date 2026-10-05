use vstd::prelude::*;

verus! {

pub type uint32 = Seq<u32>;
pub type int32 = i32;
pub type ReturnCode = u64;
pub type AgentId = u64;

pub struct S {
    pub reset_notify: Map<(Seq<u32>, u64), bool>,
}

pub spec const NOT_FOUND: ReturnCode = 1;
pub spec const INVALID_PARAMETERS: ReturnCode = 2;
pub spec const SUCCESS: ReturnCode = 3;
pub spec const RSI_SUCCESS: ReturnCode = 4;

pub spec const result: ReturnCode = 99;
pub spec const calling_agent: AgentId = 7;

pub open spec fn IsValidResetDomain(s: S, domain_id: uint32) -> bool;

pub open spec fn IsValidNotifyEnable(s: S, notify_enable: uint32) -> bool;

pub open spec fn ResultEqual(a: ReturnCode, b: ReturnCode) -> bool;

pub open spec fn ResetNotifyEnabled(s: S, domain_id: uint32, agent: AgentId) -> bool;

} // verus!
