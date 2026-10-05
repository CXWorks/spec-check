use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct DomainAttributes {
    pub attributes: UInt32,
    pub sustained_perf_level: UInt32,
    pub sustained_freq: UInt32,
    pub level_min: UInt32,
    pub level_max: UInt32,
    pub current_level: UInt32,
    pub limit_min: UInt32,
    pub limit_max: UInt32,
}

pub struct S {
    pub cmd_input_domain_id: UInt32,
    pub cmd_input_sustained_level: UInt32,
    pub num_domains: UInt32,
    pub domains: Seq<UInt32>,
    pub domain_attrs: Seq<DomainAttributes>,
}

pub spec const SCMI_SUCCESS: int32 = 0;
pub spec const SCMI_NOT_SUPPORTED: int32 = -1;
pub spec const SCMI_INVALID_PARAMETERS: int32 = -2;
pub spec const SCMI_DENIED: int32 = -3;
pub spec const SCMI_NOT_FOUND: int32 = -4;
pub spec const SCMI_OUT_OF_RANGE: int32 = -5;
pub spec const SCMI_BUSY: int32 = -6;
pub spec const SCMI_COMMS_ERROR: int32 = -7;
pub spec const SCMI_GENERIC_ERROR: int32 = -8;
pub spec const SCMI_HARDWARE_ERROR: int32 = -9;
pub spec const SCMI_PROTOCOL_ERROR: int32 = -10;

pub open spec fn DomainAt(s: S, domain_id: UInt32) -> DomainAttributes;

} // verus!
