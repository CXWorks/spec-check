use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type DomainId = u32;
pub type AgentId = u64;
pub type VoltageMode = u8;

pub struct VoltageConfig {
    pub mode: VoltageMode,
}

pub struct VoltageDomainState {
    pub mode: VoltageMode,
    pub level: i32,
}

pub struct S {
    pub domains: Map<DomainId, VoltageDomainState>,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1int) as Int32;
pub spec const INVALID_PARAMETERS: Int32 = (-2int) as Int32;
pub spec const DENIED: Int32 = (-3int) as Int32;
pub spec const NOT_FOUND: Int32 = (-4int) as Int32;

pub spec const domain_id: DomainId = 7;
pub spec const caller: AgentId = 3;
pub spec const config: VoltageConfig = VoltageConfig { mode: 1 };

pub uninterp spec fn VoltageDomainExists(s: S, domain_id: DomainId) -> bool;

pub uninterp spec fn VoltageDomainSupportsConfig(s: S, domain_id: DomainId, config: VoltageConfig) -> bool;

pub uninterp spec fn IsRequestSupported() -> bool;

pub uninterp spec fn AgentMaySetVoltageConfig(caller: AgentId, domain_id: DomainId) -> bool;

pub uninterp spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub uninterp spec fn VoltageDomain(s: S, domain_id: DomainId) -> VoltageDomainState;

} // verus!
