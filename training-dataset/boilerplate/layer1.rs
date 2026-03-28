use vstd::prelude::*;

verus! {

type UInt2 = u8;
type UInt3 = u8;
type UInt4 = u8;
type UInt5 = u8;
type UInt6 = u8;
type UInt8 = u8;
type UInt15 = u16;
type UInt16 = u16;
type UInt32 = u32;
type UInt64 = u64;
type Int8 = i8;
type Int64 = i64;
type Bits64 = u64;
type Address = usize;
type RmmRealmMeasurement = [u8; 64];

use crate::RmiDataMeasureContent::*;
use crate::RmiEmulatedMmio::*;
use crate::RmiFeature::*;
use crate::RmiHashAlgorithm::*;
use crate::RmiInjectSea::*;
use crate::RmiPmuOverflowStatus::*;
use crate::RmiRecExitReason::*;
use crate::RmiRecRunnable::*;
use crate::RmiResponse::*;
use crate::RmiRipas::*;
use crate::RmiRttEntryState::*;
use crate::RmiStatusCode::*;
use crate::RmiTrap::*;
use crate::RsiCommandReturnCode::*;
use crate::RsiHashAlgorithm::*;
use crate::RsiResponse::*;
use crate::RsiRipas::*;
use crate::RsiRipasChangeDestroyed::*;
use crate::PsciReturnCode::*;
use crate::RmmDataMeasureContent::*;
use crate::RmmGptEntry::*;
use crate::RmmGranuleState::*;
use crate::RmmHashAlgorithm::*;
use crate::RmmHipas::*;
use crate::RmmHostCallPending::*;
use crate::RmmPhysicalAddressSpace::*;
use crate::RmmPsciPending::*;
use crate::RmmRealmState::*;
use crate::RmmRecAttestState::*;
use crate::RmmRecEmulatableAbort::*;
use crate::RmmRecResponse::*;
use crate::RmmRecRunnable::*;
use crate::RmmRecState::*;
use crate::RmmRipas::*;
use crate::RmmRipasChangeDestroyed::*;
use crate::RmmRttEntryState::*;
use vstd::arithmetic::power2::pow2;

use vstd::arithmetic::power::pow;

pub spec const RMM_RTT_PAGE_LEVEL:int = 3;
pub spec const RMM_RTT_TREE_PRIMARY:int = 0;
pub spec const RMM_NUM_PERM_OVERLAY_INDICES:int = 15;
pub spec const RMM_GRANULE_SIZE:usize  = 4096;
pub spec const RMM_GRANULE_SIZE_ORDER:nat  = 10;
pub spec const FID_PSCI_CPU_ON:int = 0xC4000003;
pub spec const FID_PSCI_AFFINITY_INFO:int = 0xC4000004;

struct RmmSystemRegisters{}
struct S {
    pub mem: [u8; 64],
}

pub open spec fn VersionEqualRmi(ver1: RmiInterfaceVersion, ver2: RmiInterfaceVersion) -> bool;
pub open spec fn RttWalk_(s: S, rd: Address, addr: Address, level: int) -> RmmRttWalkResult;
