use vstd::prelude::*;

verus! {

pub type Int32 = u32;
pub type UInt32 = u32;
pub type PartitionId = u16;

pub struct S {
    pub negotiated_versions: Map<PartitionId, UInt32>,
    pub ffa_in_use: Map<PartitionId, bool>,
}

pub spec const NOT_SUPPORTED: Int32 = 0xFFFF_FFFFu32;
pub spec const INVALID_PARAMETER: Int32 = 0xFFFF_FFFEu32;
pub spec const NULL_VERSION: Int32 = 0u32;

pub spec const caller: PartitionId = 1u16;
pub spec const SPMC: PartitionId = 2u16;

pub open spec fn CalleeImplementsFfa() -> bool;
pub open spec fn ResultEqual(r: Int32, code: Int32) -> bool;
pub open spec fn IsValidVersionQueryType(t: UInt32) -> bool;
pub open spec fn CallerIsSpmcAtSeparateEl() -> bool;
pub open spec fn FfaInUse(id: PartitionId) -> bool;
pub open spec fn NegotiatedVersion(id: PartitionId) -> UInt32;
pub open spec fn VersionLessThan(a: UInt32, b: UInt32) -> bool;
pub open spec fn CalleeAllowsDowngrade() -> bool;
pub open spec fn CalleeImplementsCompatibleVersion(v: UInt32) -> bool;
pub open spec fn IsCompatible(r: Int32, v: UInt32) -> bool;
pub open spec fn CalleeIncompatibleVersionsOnlyLower(v: UInt32) -> bool;
pub open spec fn CalleeIncompatibleVersionsOnlyHigher(v: UInt32) -> bool;
pub open spec fn CalleeIncompatibleVersionsHigherAndLower(v: UInt32) -> bool;
pub open spec fn HighestIncompatibleVersion(v: UInt32) -> Int32;
pub open spec fn LowestIncompatibleVersion(v: UInt32) -> Int32;
pub open spec fn NegotiatedVersionAtStart(id: PartitionId) -> Int32;
pub open spec fn VersionNegotiated(id: PartitionId) -> bool;

} // verus!
