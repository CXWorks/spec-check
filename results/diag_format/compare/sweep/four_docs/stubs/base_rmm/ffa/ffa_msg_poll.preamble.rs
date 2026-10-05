use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type PartitionId = u32;

pub struct S {
    pub state_id: nat,
}

pub struct Buffer {
    pub owner: PartitionId,
}

pub struct FfaInstance {
    pub instance_id: nat,
}

pub spec const caller: PartitionId = 1;

pub spec const RETRY: UInt32 = 0xFFFF_FFF9;

pub spec const DENIED: UInt32 = 0xFFFF_FFFA;

pub spec const NOT_SUPPORTED: UInt32 = 0xFFFF_FFFF;

pub spec const FFA_MSG_SEND: UInt32 = 0x8400_006E;

pub spec const FFA_MSG_POLL: UInt32 = 0x8400_006A;

pub open spec fn MessageAvailable(s: S, buf: Buffer) -> bool;

pub open spec fn RxBuffer(id: PartitionId) -> Buffer;

pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;

pub open spec fn CalleeCanHandleRequest(s: S) -> bool;

pub open spec fn IsImplemented(func_id: UInt32, inst: FfaInstance) -> bool;

pub open spec fn ffa_instance(s: S) -> FfaInstance;

} // verus!
