use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub spec const FFA_SUCCESS: int = 0;
pub spec const FFA_ERROR_INVALID_PARAMETERS: int = -2;
pub spec const FFA_ERROR_DENIED: int = -6;
pub spec const FFA_ERROR_NOT_SUPPORTED: int = -1;
pub spec const FFA_ERROR_BUSY: int = -4;
pub spec const FFA_ERROR_ABORTED: int = -8;
pub spec const FFA_ERROR_NOT_READY: int = -9;

pub spec const FFA_STATE_READY: u32 = 0;
pub spec const FFA_STATE_RUNNING: u32 = 1;
pub spec const FFA_STATE_BLOCKED: u32 = 2;
pub spec const FFA_STATE_PREEMPTED: u32 = 3;
pub spec const FFA_STATE_ABORTED: u32 = 4;
pub spec const FFA_STATE_NOT_READY: u32 = 5;

pub spec const FFA_INSTANCE_NON_SECURE_PHYSICAL: u32 = 10;
pub spec const FFA_INSTANCE_SECURE_PHYSICAL: u32 = 11;
pub spec const FFA_INSTANCE_NON_SECURE_VIRTUAL: u32 = 12;
pub spec const FFA_INSTANCE_SECURE_VIRTUAL: u32 = 13;

pub struct S {
    pub ffa_msg_send_direct_req2_sender_id: u32,
    pub ffa_msg_send_direct_req2_receiver_id: u32,
    pub ffa_msg_send_direct_req2_uuid_lo: u64,
    pub ffa_msg_send_direct_req2_uuid_hi: u64,
    pub ffa_msg_send_direct_req2_state: u32,
    pub ffa_msg_send_direct_req2_instance: u32,
    pub ffa_msg_send_direct_req2_receiver_supports_direct: bool,
    pub ffa_msg_send_direct_req2_supported: bool,
}

} // verus!
