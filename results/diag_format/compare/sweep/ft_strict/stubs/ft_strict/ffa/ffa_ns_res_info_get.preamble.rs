use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type SEndpoint = u64;

pub type FfaFunctionId = u32;

pub type AmdRap = u8;

pub enum FfaStatusCode {
    NotSupported,
    InvalidParameters,
    NoMemory,
    Busy,
    Interrupted,
    Denied,
    Retry,
    Aborted,
    NoData,
}

pub struct S {
    pub dummy: u64,
}

pub struct NsRegion {
    pub base: u64,
    pub size: u64,
}

pub struct Amd {
    pub flags: u64,
    pub component_id: SEndpoint,
    pub rap: AmdRap,
    pub address: u64,
    pub page_count: u64,
}

pub struct ResInfoDescriptor {
    pub amd_array_offset: u64,
}

pub spec const FFA_NS_RES_INFO_GET: FfaFunctionId = 0x8400008B;

pub spec const SPMC_ID: SEndpoint = 0x8000;

pub spec const caller: SEndpoint = 0x8001;

pub spec const FFA_SUCCESS: Result<(), FfaStatusCode> = Ok(());

pub spec const NOT_SUPPORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::NotSupported);

pub spec const INVALID_PARAMETERS: Result<(), FfaStatusCode> = Err(FfaStatusCode::InvalidParameters);

pub spec const RETRY: Result<(), FfaStatusCode> = Err(FfaStatusCode::Retry);

pub spec const ABORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::Aborted);

pub open spec fn IsImplementedAtInstance(s: S, func: FfaFunctionId) -> bool;

pub open spec fn ResultEqual(a: Result<(), FfaStatusCode>, b: Result<(), FfaStatusCode>) -> bool;

pub open spec fn Bits(x: u64, hi: int, lo: int) -> u64;

pub open spec fn ReservedRegistersAreZero(s: S, from: int, to: int) -> bool;

pub open spec fn IsValidSEndpointId(s: S, id: SEndpoint) -> bool;

pub open spec fn IsRxBufferMappedInCallee(s: S, c: SEndpoint) -> bool;

pub open spec fn IsRxBufferOwnedByCallee(s: S, c: SEndpoint) -> bool;

pub open spec fn IsCalleeBusy(s: S) -> bool;

pub open spec fn RetrievalAbortedImpDef(s: S, c: SEndpoint) -> bool;

pub open spec fn ResInfoDescBytesWrittenToRx(s: S, c: SEndpoint) -> u64;

pub open spec fn ResInfoDescBytesRemaining(s: S, c: SEndpoint) -> u64;

pub open spec fn ResInfoDesc(s: S, c: SEndpoint) -> ResInfoDescriptor;

pub open spec fn IsAccessible(e: SEndpoint, r: NsRegion) -> bool;

pub open spec fn RegionDescribedByAmd(d: ResInfoDescriptor, e: SEndpoint, r: NsRegion) -> bool;

pub open spec fn AmdInDesc(d: ResInfoDescriptor, a: Amd) -> bool;

pub open spec fn IsIndirectlyAccessibleRegion(a: Amd) -> bool;

pub open spec fn MostPermissiveSpPermission(a: Amd) -> AmdRap;

pub open spec fn EntireNsPasAccessibleRwx(e: SEndpoint) -> bool;

pub open spec fn AmdRapForRegion(d: ResInfoDescriptor, e: SEndpoint, r: NsRegion) -> AmdRap;

pub open spec fn IsPhysicalSEl1SpUnderSEl2Spmc(e: SEndpoint) -> bool;

pub open spec fn IsStaticNsRegion(a: Amd) -> bool;

pub open spec fn Stage2BasePermToAmdRap(a: Amd) -> AmdRap;

pub open spec fn IsPhysicalSEl0Sp(e: SEndpoint) -> bool;

pub open spec fn UnprivStage1BasePermToAmdRap(a: Amd) -> AmdRap;

pub open spec fn EntireNsPasInaccessibleFrom(s: S, e: SEndpoint) -> bool;

pub open spec fn EntireNsPasInaccessibleFromAll(s: S) -> bool;

} // verus!
