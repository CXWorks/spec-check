use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int32 = i32;

pub struct S {
    pub regs: Seq<UInt64>,
    pub pending: Map<UInt32, UInt64>,
}

pub struct Instance {
    pub id: nat,
}

pub struct CallerId {
    pub id: nat,
}

pub spec const FFA_SUCCESS: UInt32 = 0x84000061;
pub spec const NOT_SUPPORTED: UInt32 = 0xFFFFFFFF;
pub spec const INVALID_PARAMETERS: UInt32 = 0xFFFFFFFE;
pub spec const DENIED: UInt32 = 0xFFFFFFFA;

pub spec const FFA_NOTIFICATION_GET: UInt32 = 0x84000082;

pub open spec fn IsImplementedAtInstance(func_id: UInt32, inst: Instance) -> bool;

pub open spec fn CurrentInstance() -> Instance;

pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;

pub open spec fn IsCallerAllowedToInvoke(caller: CallerId, func_id: UInt32) -> bool;

pub open spec fn Caller() -> CallerId;

pub open spec fn IsRecognizedPartitionId(id: UInt32) -> bool;

pub open spec fn Bits(x: UInt64, hi: int, lo: int) -> UInt32;

pub open spec fn receiver_id(s: S) -> UInt64;

pub open spec fn flags(s: S) -> UInt64;

pub open spec fn IsNonSecurePhysicalInstance(inst: Instance) -> bool;

pub open spec fn PendingSpNotifications(receiver: UInt32, vcpu: UInt32) -> UInt64;

pub open spec fn PendingVmNotifications(receiver: UInt32, vcpu: UInt32) -> UInt64;

pub open spec fn PendingSpmFrameworkNotifications(receiver: UInt32, vcpu: UInt32) -> UInt32;

pub open spec fn PendingHypervisorFrameworkNotifications(receiver: UInt32, vcpu: UInt32) -> UInt32;

} // verus!
