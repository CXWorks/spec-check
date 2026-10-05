use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt64 = u64;
pub type UInt32 = u32;
pub type EndpointId = u16;
pub type FfaInstance = u8;

pub struct S {
    pub dummy: int,
}

pub struct IdList {
    pub endpoint: EndpointId,
    pub vcpu_count: int,
}

pub const NO_DATA: Int32 = -7;
pub const NOT_SUPPORTED: Int32 = -1;

pub const FFA_NOTIFICATION_INFO_GET: UInt32 = 0x84000083;
pub const FFA_SUCCESS: UInt32 = 0x84000061;

pub const caller: EndpointId = 1;

pub const ffa_instance: FfaInstance = 0;

pub const fid: UInt64 = 0x84000083;
pub const W3: UInt64 = 3;
pub const X3: UInt64 = 4;

pub open spec fn PendingNotificationInfoAvailable(s: S, ep: EndpointId) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsImplementedAtInstance(s: S, func: UInt32, inst: FfaInstance) -> bool;

pub open spec fn ReturnedFunction() -> UInt32;

pub open spec fn Bits(x: UInt64, hi: int, lo: int) -> int;

pub open spec fn ListCount(flags: UInt64) -> int;

pub open spec fn IsSmc32(f: UInt64) -> bool;

pub open spec fn IsSmc64(f: UInt64) -> bool;

pub open spec fn IdCountInList(flags: UInt64, i: UInt64) -> int;

pub open spec fn TotalIdCount(flags: UInt64) -> int;

pub open spec fn FirstIdOfList(id_lists: [UInt64; 8], flags: UInt64, i: UInt64) -> int;

pub open spec fn IdListsTightlyPacked(id_lists: [UInt64; 8], flags: UInt64) -> bool;

pub open spec fn ListEndpoint(id_lists: [UInt64; 8], flags: UInt64, i: UInt64) -> EndpointId;

pub open spec fn HasOnlyGlobalPending(ep: EndpointId) -> bool;

pub open spec fn ListIsEndpointThenVcpuIds(id_lists: [UInt64; 8], flags: UInt64, i: UInt64) -> bool;

pub open spec fn AllPendingIdListsRetrieved(ep: EndpointId) -> bool;

pub open spec fn ListAt(id_lists: [UInt64; 8], flags: UInt64, i: UInt64) -> IdList;

pub open spec fn RetrievableAgain(l: IdList) -> bool;

pub open spec fn IsNonSecureVirtualInstance(inst: FfaInstance) -> bool;

pub open spec fn SchedulerImplementedInVm(ep: EndpointId, vm: EndpointId) -> bool;

} // verus!
