use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type ScmiStatus = i32;

pub const SUCCESS: ScmiStatus = 0i32;
pub const NOT_SUPPORTED: ScmiStatus = -1i32;
pub const INVALID_PARAMETERS: ScmiStatus = -2i32;
pub const DENIED: ScmiStatus = -3i32;
pub const NOT_FOUND: ScmiStatus = -4i32;
pub const IN_USE: ScmiStatus = -7i32;
pub const PROTOCOL_ERROR: ScmiStatus = -10i32;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsValidPinctrlConfigType(t: UInt32) -> bool;

pub open spec fn IsValidPinctrlIdentifier(s: S, selector: UInt32, identifier: UInt32) -> bool;

pub open spec fn PinctrlConfigsSupported(s: S, selector: UInt32, identifier: UInt32, configs: Seq<(UInt32, UInt32)>) -> bool;

pub open spec fn PinctrlFunctionSupported(s: S, selector: UInt32, identifier: UInt32, function_id: UInt32) -> bool;

pub open spec fn PinctrlAgentPermitted(s: S, selector: UInt32, identifier: UInt32) -> bool;

pub open spec fn PinctrlInUseByOtherAgent(s: S, selector: UInt32, identifier: UInt32) -> bool;

pub open spec fn PinctrlConfigsExceedTransport(s: S, num_configs: UInt32) -> bool;

pub open spec fn PinctrlConfigValue(s: S, selector: UInt32, identifier: UInt32, config_type: UInt32) -> UInt32;

pub open spec fn PinctrlFunctionSelected(s: S, selector: UInt32, identifier: UInt32) -> UInt32;

} // verus!
