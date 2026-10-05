use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub enum RsiCommandReturnCode {
    Success,
    NotSupported,
    NotFound,
    InvalidParameters,
    Denied,
}

pub struct S {
    pub dummy: int,
}

pub spec const BASE_SET_PROTOCOL_PERMISSIONS: UInt32 = 0x0Bu32;

pub spec const caller: UInt32 = 0u32;

pub spec const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());
pub spec const SUCCESS: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::Success);
pub spec const NOT_SUPPORTED: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::NotSupported);
pub spec const NOT_FOUND: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::NotFound);
pub spec const INVALID_PARAMETERS: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::InvalidParameters);
pub spec const DENIED: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::Denied);

pub open spec fn IsCommandSupported(s: S, cmd: UInt32) -> bool;
pub open spec fn AgentExists(s: S, agent_id: UInt32) -> bool;
pub open spec fn DeviceExists(s: S, device_id: UInt32) -> bool;
pub open spec fn ProtocolExists(s: S, protocol_id: UInt32) -> bool;
pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn IsValidFlags(s: S, flags: UInt32) -> bool;
pub open spec fn CallerMaySetProtocolPermissions(s: S, caller_id: UInt32, agent_id: UInt32) -> bool;
pub open spec fn ProtocolAccessAllowed(s: S, agent_id: UInt32, device_id: UInt32, protocol_id: UInt32) -> bool;
pub open spec fn DeviceAccessAllowed(s: S, agent_id: UInt32, device_id: UInt32) -> bool;

} // verus!
