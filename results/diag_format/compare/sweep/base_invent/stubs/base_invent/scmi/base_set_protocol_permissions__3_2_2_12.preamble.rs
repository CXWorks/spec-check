use vstd::prelude::*;

verus! {

pub type int32 = u32;

pub type UInt32 = u32;

pub struct S {
    pub agent_id: UInt32,
    pub device_id: UInt32,
    pub protocol_id: UInt32,
    pub flags: UInt32,
    pub command_id: UInt32,
}

impl S {
    pub open spec fn agent_exists(self, agent_id: UInt32) -> bool;

    pub open spec fn device_exists(self, device_id: UInt32) -> bool;

    pub open spec fn protocol_exists(self, protocol_id: UInt32) -> bool;

    pub open spec fn flags_valid(self, flags: UInt32) -> bool;

    pub open spec fn command_supported(self, command_id: UInt32) -> bool;

    pub open spec fn agent_allowed_to_set_permissions(self, caller_id: UInt32, agent_id: UInt32) -> bool;

    pub open spec fn agent_protocol_permissions(self, agent_id: UInt32, device_id: UInt32, protocol_id: UInt32) -> UInt32;
}

} // verus!
