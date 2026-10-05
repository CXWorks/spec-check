use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt64 = u64;
pub type Array<T, const N: usize> = [T; N];

pub struct S {
    pub cmd_input_flags: UInt64,
    pub cmd_input_receiver_endpoint_id: UInt16,
    pub cmd_input_receiver_vcpu_id: UInt16,
    pub cmd_input_sp_bitmask: Array<UInt64, 6>,
    pub cmd_input_vm_bitmask: Array<UInt64, 6>,
    pub cmd_input_spmc_fw_bitmask: UInt64,
    pub cmd_input_hyp_fw_bitmask: UInt64,
    pub cmd_input_instance: UInt32,
}

pub const FFA_SUCCESS64: UInt32 = 0xC4000061;
pub const FFA_ERROR: UInt32 = 0x84000060;
pub const FFA_NOTIFICATION_GET2: UInt32 = 0xC4000097;
pub const NOT_SUPPORTED: UInt32 = 0x1;
pub const DENIED: UInt32 = 0x2;
pub const INVALID_PARAMETERS: UInt32 = 0x3;
pub const NON_SECURE_PHYSICAL: UInt32 = 0x10;
pub const SP_BITMAP: UInt32 = 0x20;
pub const VM_BITMAP: UInt32 = 0x21;
pub const SPMC_FW_BITMAP: UInt32 = 0x22;
pub const HYP_FW_BITMAP: UInt32 = 0x23;

pub open spec fn ExceedsSupportedNotifications(bitmask: Array<UInt64, 6>) -> bool;

pub open spec fn IsRecognizedPartitionId(id: UInt16) -> bool;

pub open spec fn IsImplementedAtInstance(fid: UInt32, instance: UInt32) -> bool;

pub open spec fn IsCallerAllowedToInvoke(fid: UInt32) -> bool;

pub open spec fn ResultEqual(a: UInt32, b: UInt32) -> bool;

pub open spec fn IsPending(endpoint: UInt16, vcpu: UInt16, kind: UInt32, j: int) -> bool;

pub open spec fn NotificationState(endpoint: UInt16, vcpu: UInt16, kind: UInt32, j: int) -> bool;

} // verus!
