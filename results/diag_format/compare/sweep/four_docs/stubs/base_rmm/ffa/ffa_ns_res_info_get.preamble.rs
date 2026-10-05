use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type EndpointId = u16;

pub struct S {
    pub dummy: u64,
}

pub struct Region {
    pub base: u64,
    pub page_count: u32,
}

pub const NOT_SUPPORTED: u32 = 1;
pub const INVALID_PARAMETERS: u32 = 2;
pub const RETRY: u32 = 3;
pub const ABORTED: u32 = 4;
pub const FFA_SUCCESS: u32 = 5;

pub const FFA_NS_RES_INFO_GET: u32 = 0x8400_0090;

pub const X3: u32 = 3;
pub const X17: u32 = 17;

pub const target_id: u64 = 0;
pub const flags: u64 = 0;
pub const caller: EndpointId = 0;

pub open spec fn IsImplementedAtInstance(func_id: u32) -> bool;

pub open spec fn ResultEqual(result: u32, code: u32) -> bool;

pub open spec fn AreReservedRegistersZero(first: u32, last: u32) -> bool;

pub open spec fn IsValidSEndpointId(id: u64) -> bool;

pub open spec fn IsRxBufferMappedInCallee(ep: EndpointId) -> bool;

pub open spec fn IsRxBufferOwnedByCallee(ep: EndpointId) -> bool;

pub open spec fn IsCalleeBusy() -> bool;

pub open spec fn CanContinueRetrieval() -> bool;

pub open spec fn RxBuffer(ep: EndpointId) -> Seq<u8>;

pub open spec fn SpmcId() -> u16;

pub open spec fn MostPermissiveSpPermission(region: Region) -> u8;

pub open spec fn Stage2BasePermsToRap(region: Region) -> u8;

pub open spec fn IsNsPasInaccessibleFrom(id: u64) -> bool;

pub open spec fn IsNsPasInaccessibleFromAll() -> bool;

} // verus!
