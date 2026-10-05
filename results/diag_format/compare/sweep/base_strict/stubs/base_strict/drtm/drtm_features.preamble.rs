use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type UInt64 = u64;
pub type Bits64 = u64;

pub struct S {
    pub dummy: u64,
}

pub const NOT_SUPPORTED: Int64 = -1;

pub open spec fn Bits(x: u64, hi: int, lo: int) -> u64;

pub open spec fn IsImplementedDrtmFunction(fid: u64) -> bool;

pub open spec fn IsSupportedDrtmFeature(fid: u64) -> bool;

pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;

pub open spec fn SupportedPcrUsageSchemas() -> u64;

pub open spec fn TpmBasedHashSupported() -> u64;

pub open spec fn FirmwareHashAlgorithmId() -> u64;

pub open spec fn NormalWorldDceMinPages() -> u64;

pub open spec fn DlmeDataMinPages() -> u64;

pub open spec fn UsesNormalWorldDce() -> bool;

pub open spec fn DmaProtectionSupportBitmap() -> u64;

pub open spec fn RegionBasedDmaProtectionSupported() -> bool;

pub open spec fn MaxDmaProtectedRegions() -> u64;

pub open spec fn BootPeTargetCpuEncoding() -> Bits64;

pub open spec fn MaxTcbHashes() -> u64;

pub open spec fn DlmeImageAuthSupported() -> u64;

} // verus!
