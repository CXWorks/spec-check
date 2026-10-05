use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type AgentId = u32;
pub type DeviceId = u32;
pub type Permission = u32;

pub struct PermissionTable {
    pub id: u64,
}

impl PermissionTable {
    pub open spec fn contains(self, agent: AgentId, device: DeviceId, perm: Permission) -> bool;
}

pub struct S {
    pub agent_permissions: PermissionTable,
    pub agent_id: AgentId,
    pub device_id: DeviceId,
}

} // verus!
