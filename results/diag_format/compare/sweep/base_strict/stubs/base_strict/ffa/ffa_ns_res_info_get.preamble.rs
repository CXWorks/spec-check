use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt64 = u64;
pub type UInt32 = u32;
pub type SEndpoint = u64;
pub type FuncId = u32;

pub struct S {
    pub cmd_input_target_id: u64,
    pub cmd_input_flags: u64,
    pub cmd_output_res_info_len: u64,
}

pub struct NsRegion {
    pub base: u64,
    pub size: u64,
}

pub struct Amd {
    pub flags: u64,
    pub component_id: SEndpoint,
    pub rap: u64,
    pub address: u64,
    pub page_count: u64,
}

pub struct ResInfoDescriptor {
    pub amd_array_offset: u64,
}

pub const FFA_NS_RES_INFO_GET: FuncId = 0x8400008F;

pub const FFA_SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const RETRY: Int32 = -7;
pub const ABORTED: Int32 = -8;

pub const SPMC_ID: SEndpoint = 0x8000;

pub open spec fn IsImplementedAtInstance(fid: FuncId) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn Bits64(v: u64, hi: int, lo: int) -> u64;
pub open spec fn ReservedRegistersAreZero(from: int, to: int) -> bool;
pub open spec fn IsValidSEndpointId(id: SEndpoint) -> bool;
pub open spec fn IsRxBufferMappedInCallee(s: S) -> bool;
pub open spec fn IsRxBufferOwnedByCallee(s: S) -> bool;
pub open spec fn IsCalleeBusy(s: S) -> bool;
pub open spec fn RetrievalAbortedImpDef(s: S) -> bool;
pub open spec fn ResInfoDescBytesWrittenToRx(s: S) -> u64;
pub open spec fn ResInfoDescBytesRemaining(s: S) -> u64;
pub open spec fn ResInfoDesc(s: S) -> ResInfoDescriptor;
pub open spec fn IsAccessible(e: SEndpoint, r: NsRegion) -> bool;
pub open spec fn RegionDescribedByAmd(d: ResInfoDescriptor, e: SEndpoint, r: NsRegion) -> bool;
pub open spec fn AmdInDesc(d: ResInfoDescriptor, a: Amd) -> bool;
pub open spec fn IsIndirectlyAccessibleRegion(a: Amd) -> bool;
pub open spec fn MostPermissiveSpPermission(a: Amd) -> u64;
pub open spec fn EntireNsPasAccessibleRwx(e: SEndpoint) -> bool;
pub open spec fn AmdRapForRegion(d: ResInfoDescriptor, e: SEndpoint, r: NsRegion) -> u64;
pub open spec fn IsPhysicalSEl1SpUnderSEl2Spmc(e: SEndpoint) -> bool;
pub open spec fn IsStaticNsRegion(a: Amd) -> bool;
pub open spec fn Stage2BasePermToAmdRap(a: Amd) -> u64;
pub open spec fn IsPhysicalSEl0Sp(e: SEndpoint) -> bool;
pub open spec fn UnprivStage1BasePermToAmdRap(a: Amd) -> u64;
pub open spec fn EntireNsPasInaccessibleFrom(e: SEndpoint) -> bool;
pub open spec fn EntireNsPasInaccessibleFromAll() -> bool;

} // verus!
