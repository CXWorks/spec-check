use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int = int;
pub type Endpoint = int;
pub type FuncId = u32;

pub struct S {
    pub dummy: u64,
}

pub const NO_DATA: Int32 = -7;
pub const NOT_SUPPORTED: Int32 = -1;

pub const FFA_NOTIFICATION_INFO_GET: FuncId = 0x84000083;
pub const FFA_SUCCESS: FuncId = 0x84000061;

pub open spec fn PendingNotificationInfoAvailable(s: S) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsImplementedAtInstance(s: S, f: FuncId) -> bool;

pub open spec fn ReturnedFunction(s: S) -> FuncId;

pub open spec fn AllIdListsRetrieved(s: S) -> bool;

pub open spec fn NumIdListsReturned(s: S) -> int;

pub open spec fn IsSmc32(s: S) -> bool;

pub open spec fn IsSmc64(s: S) -> bool;

pub open spec fn IdListSize(s: S, i: int) -> int;

pub open spec fn PendingGlobalNotifications(s: S, e: Endpoint) -> bool;

pub open spec fn PendingPerVcpuNotifications(s: S, e: Endpoint) -> bool;

pub open spec fn FirstElement(l: Endpoint) -> Endpoint;

pub open spec fn Size(l: Endpoint) -> int;

pub open spec fn first_id_of_first_list(s: S) -> int;

pub open spec fn TotalIdsReturned(s: S) -> int;

pub open spec fn IsNonSecureVirtualInstance(s: S) -> bool;

pub open spec fn ReturnedEndpoint(s: S, e: Endpoint) -> bool;

pub open spec fn SchedulerImplementedInCallingVm(s: S, e: Endpoint) -> bool;

pub open spec fn ReturnedAgainInLaterInvocation(s: S, l: Endpoint) -> bool;

} // verus!
