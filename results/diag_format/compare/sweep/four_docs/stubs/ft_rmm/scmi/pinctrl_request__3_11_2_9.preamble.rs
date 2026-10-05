use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type AgentId = u32;

pub struct UInt32 {
    pub v: u32,
}

impl UInt32 {
    pub open spec fn spec_index(self, r: core::ops::Range<int>) -> int;
}

pub struct S {
    pub dummy: int,
}

pub enum PsmmStatusCode {
    InvalidParameters,
    NotFound,
    Denied,
    InUse,
}

pub spec const caller: AgentId = 7;

pub spec const SUCCESS: Result<Int32, PsmmStatusCode> = Result::Ok(0);

pub spec const INVALID_PARAMETERS: Result<Int32, PsmmStatusCode> = Result::Err(PsmmStatusCode::InvalidParameters);

pub spec const NOT_FOUND: Result<Int32, PsmmStatusCode> = Result::Err(PsmmStatusCode::NotFound);

pub spec const DENIED: Result<Int32, PsmmStatusCode> = Result::Err(PsmmStatusCode::Denied);

pub spec const IN_USE: Result<Int32, PsmmStatusCode> = Result::Err(PsmmStatusCode::InUse);

pub open spec fn ResultEqual(a: Result<Int32, PsmmStatusCode>, b: Result<Int32, PsmmStatusCode>) -> bool;

pub open spec fn IsValidPinOrGroup(s: S, identifier: UInt32, selector: int) -> bool;

pub open spec fn AgentMayRequestPinOrGroup(s: S, agent: AgentId, identifier: UInt32, selector: int) -> bool;

pub open spec fn IsUnderExclusiveControl(s: S, identifier: UInt32, selector: int) -> bool;

pub open spec fn ExclusiveOwner(s: S, identifier: UInt32, selector: int) -> AgentId;

} // verus!
