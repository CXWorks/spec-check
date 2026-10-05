use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_SUPPORTED: int32 = (-1int) as int32;
pub spec const INVALID_PARAMETERS: int32 = (-2int) as int32;
pub spec const DENIED: int32 = (-3int) as int32;
pub spec const NOT_FOUND: int32 = (-4int) as int32;

pub spec const domain_id: uint32 = 7;
pub spec const flags: uint32 = 11;
pub spec const voltage_level: int32 = 1000;

pub uninterp spec fn DomainExists(s: S, domain_id: uint32) -> bool;
pub uninterp spec fn VoltageLevelSupported(s: S, domain_id: uint32, voltage_level: int32) -> bool;
pub uninterp spec fn VoltageLevelSetSupported(s: S, domain_id: uint32) -> bool;
pub uninterp spec fn AgentAllowedToSetVoltage(s: S, domain_id: uint32) -> bool;
pub uninterp spec fn VoltageLevelSet(old_s: S, domain_id: uint32, voltage_level: int32, flags: uint32, new_s: S) -> bool;

} // verus!
