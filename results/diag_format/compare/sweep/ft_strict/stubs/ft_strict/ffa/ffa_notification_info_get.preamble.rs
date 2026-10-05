use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt64 = u64;
pub type FunctionId = u32;
pub type EndpointId = u16;
pub type FfaInstance = u8;
pub type IdList = Seq<u16>;

pub struct S {
    pub dummy: int,
}

pub const FFA_SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const NO_DATA: Int32 = -7;

pub const FFA_NOTIFICATION_INFO_GET: FunctionId = 0x8400007D;
pub const fid: FunctionId = 0xC400007D;

pub const ffa_instance: FfaInstance = 1;
pub const caller: EndpointId = 2;

pub const W3: UInt64 = 3;
pub const X3: UInt64 = 4;

pub open spec fn PendingNotificationInfoAvailable(s: S, ep: EndpointId) -> bool;
pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, f: FunctionId, inst: FfaInstance) -> bool;
pub open spec fn ReturnedFunction() -> Int32;
pub open spec fn Bits(x: UInt64, hi: int, lo: int) -> int;
pub open spec fn ListCount(pending_flags: UInt64) -> int;
pub open spec fn IsSmc32(f: FunctionId) -> bool;
pub open spec fn IsSmc64(f: FunctionId) -> bool;
pub open spec fn IdCountInList(pending_flags: UInt64, i: UInt64) -> int;
pub open spec fn TotalIdCount(pending_flags: UInt64) -> int;
pub open spec fn FirstIdOfList(id_lists: [UInt64; 4], pending_flags: UInt64, i: UInt64) -> int;
pub open spec fn IdListsTightlyPacked(id_lists: [UInt64; 4], pending_flags: UInt64) -> bool;
pub open spec fn HasOnlyGlobalPending(ep: EndpointId) -> bool;
pub open spec fn ListEndpoint(id_lists: [UInt64; 4], pending_flags: UInt64, i: UInt64) -> EndpointId;
pub open spec fn ListIsEndpointThenVcpuIds(id_lists: [UInt64; 4], pending_flags: UInt64, i: UInt64) -> bool;
pub open spec fn AllPendingIdListsRetrieved(ep: EndpointId) -> bool;
pub open spec fn RetrievableAgain(l: IdList) -> bool;
pub open spec fn ListAt(id_lists: [UInt64; 4], pending_flags: UInt64, i: UInt64) -> IdList;
pub open spec fn IsNonSecureVirtualInstance(inst: FfaInstance) -> bool;
pub open spec fn SchedulerImplementedInVm(ep: EndpointId, vm: EndpointId) -> bool;

} // verus!
