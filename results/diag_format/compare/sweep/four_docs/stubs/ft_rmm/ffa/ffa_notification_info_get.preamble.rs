use vstd::prelude::*;

verus! {

pub type UInt64 = u64;
pub type UInt32 = u32;

pub enum FFAStatus {
    Success,
    Error,
    NoData,
    NotSupported,
}

pub struct S {
    pub dummy: u64,
}

pub spec const FFA_SUCCESS: Result<FFAStatus, ()> = Ok(FFAStatus::Success);
pub spec const FFA_ERROR: Result<FFAStatus, ()> = Ok(FFAStatus::Error);

pub spec const NO_DATA: FFAStatus = FFAStatus::NoData;
pub spec const NOT_SUPPORTED: FFAStatus = FFAStatus::NotSupported;

pub spec const FFA_NOTIFICATION_INFO_GET: u32 = 0x84000083u32;

pub open spec fn PendingNotificationInfoAvailable(s: S) -> bool;

pub open spec fn ResultEqual(result: Result<FFAStatus, ()>, status: FFAStatus) -> bool;

pub open spec fn IsImplementedAtInstance(s: S, func_id: u32) -> bool;

pub open spec fn ReturnedFunction(s: S) -> Result<FFAStatus, ()>;

pub open spec fn AllIdListsRetrieved(s: S) -> bool;

pub open spec fn NumIdListsReturned(s: S) -> u64;

pub open spec fn IsSmc32(s: S) -> bool;

pub open spec fn IsSmc64(s: S) -> bool;

pub open spec fn IdListSize(s: S, i: u64) -> u64;

pub open spec fn TotalIdsReturned(s: S) -> u64;

pub open spec fn IsNonSecureVirtualInstance(s: S) -> bool;

} // verus!
