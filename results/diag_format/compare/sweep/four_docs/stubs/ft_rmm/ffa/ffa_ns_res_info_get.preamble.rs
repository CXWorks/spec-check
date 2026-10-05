use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int32 = i32;

pub enum FFAReturnCode {
    Success,
}

pub struct Region {
    pub base: UInt64,
    pub page_count: UInt32,
}

pub struct S {
    pub dummy: UInt64,
}

pub const FFA_NS_RES_INFO_GET: UInt32 = 0x84000090;

pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const RETRY: Int32 = -7;
pub const ABORTED: Int32 = -8;

pub const caller: UInt16 = 0;

pub spec const FFA_SUCCESS: Result<FFAReturnCode, Int32> = Ok(FFAReturnCode::Success);

pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Result<FFAReturnCode, Int32>, code: Int32) -> bool;

pub open spec fn AreReservedRegistersZero(s: S, from: int, to: int) -> bool;

pub open spec fn IsValidSEndpointId(s: S, id: UInt16) -> bool;

pub open spec fn IsRxBufferMappedInCallee(s: S, endpoint: UInt16) -> bool;

pub open spec fn IsRxBufferOwnedByCallee(s: S, endpoint: UInt16) -> bool;

pub open spec fn IsCalleeBusy(s: S) -> bool;

pub open spec fn CanContinueRetrieval(s: S) -> bool;

pub open spec fn RxBuffer(s: S, endpoint: UInt16) -> Seq<u8>;

pub open spec fn SpmcId() -> UInt16;

pub open spec fn MostPermissiveSpPermission(s: S, region: Region) -> u8;

pub open spec fn Stage2BasePermsToRap(s: S, region: Region) -> u8;

pub open spec fn IsNsPasInaccessibleFrom(s: S, id: UInt16) -> bool;

pub open spec fn IsNsPasInaccessibleFromAll(s: S) -> bool;

pub open spec fn ResourceInfoRetrievalState(s: S, endpoint: UInt16) -> UInt32;

} // verus!
