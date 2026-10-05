use vstd::prelude::*;
verus! {

pub type PsciReturnCode = i32;
pub type Address = u64;
pub type ContextId = u64;
pub type Platform = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const SUCCESS: PsciReturnCode = 0;
pub spec const INVALID_ADDRESS: PsciReturnCode = (-9int) as i32;

pub spec const entry_point_address: Address = 0x1000;
pub spec const context_id: ContextId = 0x2000;
pub spec const platform: Platform = 0x3000;

pub open spec fn IsKnownUnavailableToCaller(s: S, addr: Address) -> bool;
pub open spec fn ResultEqual(result: PsciReturnCode, code: PsciReturnCode) -> bool;
pub open spec fn ReturnedAtNextInstruction() -> bool;
pub open spec fn ResumedAt(addr: Address) -> bool;
pub open spec fn ContextIdPresented(id: ContextId) -> bool;
pub open spec fn AllCoresInDefaultSuspend() -> bool;
pub open spec fn ThermallyCritical(p: Platform) -> bool;

} // verus!
