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
use crate::RmmFeature::*;
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

pub enum RmiDataMeasureContent {
  RMI_NO_MEASURE_CONTENT,
  RMI_MEASURE_CONTENT,
}

pub enum RmiEmulatedMmio {
  RMI_NOT_EMULATED_MMIO,
  RMI_EMULATED_MMIO,
}

pub enum RmiFeature {
  RMI_FEATURE_FALSE,
  RMI_FEATURE_TRUE,
}

pub enum RmiHashAlgorithm {
  RMI_HASH_SHA_256,
  RMI_HASH_SHA_512,
}

pub enum RmiInjectSea {
  RMI_NO_INJECT_SEA,
  RMI_INJECT_SEA,
}

pub enum RmiPmuOverflowStatus {
  RMI_PMU_OVERFLOW_NOT_ACTIVE,
  RMI_PMU_OVERFLOW_ACTIVE,
}

pub enum RmiRecExitReason {
  RMI_EXIT_SYNC,
  RMI_EXIT_IRQ,
  RMI_EXIT_FIQ,
  RMI_EXIT_PSCI,
  RMI_EXIT_RIPAS_CHANGE,
  RMI_EXIT_HOST_CALL,
  RMI_EXIT_SERROR,
}

pub enum RmiRecRunnable {
  RMI_NOT_RUNNABLE,
  RMI_RUNNABLE,
}

pub enum RmiResponse {
  RMI_ACCEPT,
  RMI_REJECT,
}

pub enum RmiRipas {
  RMI_EMPTY,
  RMI_RAM,
  RMI_DESTROYED,
}

pub enum RmiRttEntryState {
  RMI_UNASSIGNED,
  RMI_ASSIGNED,
  RMI_TABLE,
}

pub enum RmiStatusCode {
  RMI_ERROR_INPUT,
  RMI_ERROR_REALM(int),
  RMI_ERROR_REC,
  RMI_ERROR_RTT(int),
}

pub enum RmiTrap {
  RMI_NO_TRAP,
  RMI_TRAP,
}

pub enum RsiCommandReturnCode {
  RSI_SUCCESS,
  RSI_ERROR_INPUT,
  RSI_ERROR_STATE,
  RSI_INCOMPLETE,
  RSI_ERROR_UNKNOWN,
}

pub enum RsiHashAlgorithm {
  RSI_HASH_SHA_256,
  RSI_HASH_SHA_512,
}

pub enum RsiResponse {
  RSI_ACCEPT,
  RSI_REJECT,
}

pub enum RsiRipas {
  RSI_EMPTY,
  RSI_RAM,
  RSI_DESTROYED,
  RSI_DEV,
}

pub enum RsiRipasChangeDestroyed {
  RSI_NO_CHANGE_DESTROYED,
  RSI_CHANGE_DESTROYED,
}

pub enum PsciReturnCode {
  PSCI_INVALID_ADDRESS,
  PSCI_DISABLED,
  PSCI_NOT_PRESENT,
  PSCI_INTERNAL_FAILURE,
  PSCI_ON_PENDING,
  PSCI_ALREADY_ON,
  PSCI_DENIED,
  PSCI_INVALID_PARAMETERS,
  PSCI_NOT_SUPPORTED,
  PSCI_SUCCESS,
  PSCI_OFF,
}

pub enum RmmDataMeasureContent {
  NO_MEASURE_CONTENT,
  MEASURE_CONTENT,
}

pub enum RmmFeature {
  FEATURE_FALSE,
  FEATURE_TRUE,
}

pub enum RmmGptEntry {
  GPT_AAP,
  GPT_NS,
  GPT_REALM,
  GPT_ROOT,
  GPT_SECURE,
}

pub enum RmmGranuleState {
  DATA,
  DELEGATED,
  RD,
  REC,
  REC_AUX,
  RTT,
  UNDELEGATED,
}

pub enum RmmHashAlgorithm {
  HASH_SHA_256,
  HASH_SHA_512,
}

pub enum RmmHipas {
  HIPAS_ASSIGNED,
  HIPAS_ASSIGNED_NS,
  HIPAS_UNASSIGNED,
  HIPAS_UNASSIGNED_NS,
}

pub enum RmmHostCallPending {
  HOST_CALL_PENDING,
  NO_HOST_CALL_PENDING,
}

pub enum RmmPhysicalAddressSpace {
  PAS_NS,
  PAS_REALM,
  PAS_ROOT,
  PAS_SECURE,
}

pub enum RmmPsciPending {
  NO_PSCI_REQUEST_PENDING,
  PSCI_REQUEST_PENDING,
}

pub enum RmmRealmState {
  REALM_ACTIVE,
  REALM_NEW,
  REALM_SYSTEM_OFF,
}

pub enum RmmRecAttestState {
  ATTEST_IN_PROGRESS,
  NO_ATTEST_IN_PROGRESS,
}

pub enum RmmRecEmulatableAbort {
  EMULATABLE_ABORT,
  NOT_EMULATABLE_ABORT,
}

pub enum RmmRecResponse {
  ACCEPT,
  REJECT,
}

pub enum RmmRecRunnable {
  NOT_RUNNABLE,
  RUNNABLE,
}

pub enum RmmRecState {
  REC_READY,
  REC_RUNNING,
}

pub enum RmmRipas {
  DESTROYED,
  DEV,
  EMPTY,
  RAM,
}

pub enum RmmRipasChangeDestroyed {
  CHANGE_DESTROYED,
  NO_CHANGE_DESTROYED,
}

pub enum RmmRttEntryState {
  ASSIGNED,
  ASSIGNED_NS,
  TABLE,
  UNASSIGNED,
  UNASSIGNED_NS,
}

struct RmiCommandReturnCode {
  pub status: RmiStatusCode,
  pub index: UInt8,
}

struct RmiDataFlags {
  pub measure: RmiDataMeasureContent,
}

struct RmiFeatureRegister0 {
  pub S2SZ: UInt8,
  pub LPA2: RmiFeature,
  pub SVE_EN: RmiFeature,
  pub SVE_VL: UInt4,
  pub NUM_BPS: UInt6,
  pub NUM_WPS: UInt6,
  pub PMU_EN: RmiFeature,
  pub PMU_NUM_CTRS: UInt5,
  pub HASH_SHA_256: RmiFeature,
  pub HASH_SHA_512: RmiFeature,
  pub GICV3_NUM_LRS: UInt4,
}

struct RmiInterfaceVersion {
  pub minor: UInt16,
  pub major: UInt15,
}

struct RmiRealmFlags {
  pub lpa2: RmiFeature,
  pub sve: RmiFeature,
  pub pmu: RmiFeature,
}

struct RmiRealmParams {
  pub flags: RmiRealmFlags,
  pub s2sz: UInt8,
  pub sve_vl: UInt8,
  pub num_bps: UInt8,
  pub num_wps: UInt8,
  pub pmu_num_ctrs: UInt8,
  pub hash_algo: RmiHashAlgorithm,
  pub rpv: [UInt64; 8],
  pub vmid: UInt16,
  pub rtt_base: Address,
  pub rtt_level_start: Int64,
  pub rtt_num_start: UInt32,
}

struct RmiRecCreateFlags {
  pub runnable: RmiRecRunnable,
}

struct RmiRecEnter {
  pub flags: RmiRecEnterFlags,
  pub gicv3_hcr: UInt64,
  pub gprs: [UInt64; 31],
  pub gicv3_lrs: [UInt64; 16],
}

struct RmiRecEnterFlags {
  pub emul_mmio: RmiEmulatedMmio,
  pub inject_sea: RmiInjectSea,
  pub trap_wfi: RmiTrap,
  pub trap_wfe: RmiTrap,
  pub ripas_response: RmiResponse,
}

struct RmiRecExit {
  pub exit_reason: RmiRecExitReason,
  pub esr: UInt64,
  pub far: UInt64,
  pub hpfar: UInt64,
  pub gicv3_hcr: UInt64,
  pub gicv3_misr: UInt64,
  pub gicv3_vmcr: UInt64,
  pub cntp_ctl: UInt64,
  pub cntp_cval: UInt64,
  pub cntv_ctl: UInt64,
  pub cntv_cval: UInt64,
  pub ripas_base: UInt64,
  pub ripas_top: UInt64,
  pub ripas_value: RmiRipas,
  pub imm: UInt16,
  pub pmu_ovf_status: RmiPmuOverflowStatus,
  pub gprs: [UInt64; 31],
  pub gicv3_lrs: [UInt64; 16],
}

struct RmiRecMpidr {
  pub aff0: UInt4,
  pub aff1: UInt8,
  pub aff2: UInt8,
  pub aff3: UInt8,
}

struct RmiRecParams {
  pub flags: RmiRecCreateFlags,
  pub mpidr: RmiRecMpidr,
  pub pc: UInt64,
  pub num_aux: UInt64,
  pub gprs: [UInt64; 8],
  pub aux: [Address; 16],
}

struct RmiRecRun {
  pub enter: RmiRecEnter,
  pub exit: RmiRecExit,
}

struct RsiHostCall {
  pub imm: UInt16,
  pub gprs: [UInt64; 31],
}

struct RsiInterfaceVersion {
  pub minor: UInt16,
  pub major: UInt15,
}

struct RsiRealmConfig {
  pub ipa_width: UInt64,
  pub hash_algo: RsiHashAlgorithm,
  pub rpv: [UInt64; 8],
}

struct RsiRipasChangeFlags {
  pub destroyed: RsiRipasChangeDestroyed,
}

struct PsciInterfaceVersion {
  pub minor: UInt16,
  pub major: UInt15,
}

struct RmmDataFlags {
  pub measure: RmmDataMeasureContent,
}

struct RmmFeatures {
  pub max_ipa_width: UInt64,
  pub feat_lpa2: RmmFeature,
  pub feat_sve: RmmFeature,
  pub max_sve_vl: UInt64,
  pub num_bps: UInt64,
  pub num_wps: UInt64,
  pub feat_pmu: RmmFeature,
  pub pmu_num_ctrs: UInt64,
  pub feat_sha_256: RmmFeature,
  pub feat_sha_512: RmmFeature,
  pub max_recs_order: UInt64,
}

struct RmmGranule {
  pub gpt: RmmGptEntry,
  pub state: RmmGranuleState,
}

struct RmmMeasurementDescriptorData {
  pub desc_type: UInt8,
  pub len: UInt64,
  pub rim: RmmRealmMeasurement,
  pub ipa: Address,
  pub flags: RmmDataFlags,
  pub content: RmmRealmMeasurement,
}

struct RmmMeasurementDescriptorRec {
  pub desc_type: UInt8,
  pub len: UInt64,
  pub rim: RmmRealmMeasurement,
  pub content: RmmRealmMeasurement,
}

struct RmmMeasurementDescriptorRipas {
  pub desc_type: UInt8,
  pub len: UInt64,
  pub rim: RmmRealmMeasurement,
  pub base: Address,
  pub top: Address,
}

struct RmmRealm {
  pub feat_lpa2: RmmFeature,
  pub ipa_width: UInt8,
  pub measurements: [RmmRealmMeasurement; 5],
  pub hash_algo: RmmHashAlgorithm,
  pub rec_index: UInt64,
  pub rtt_base: Address,
  pub rtt_level_start: Int64,
  pub rtt_num_start: UInt64,
  pub state: RmmRealmState,
  pub vmid: UInt16,
  pub rpv: [UInt64; 8],
  pub num_recs: UInt64,
}

struct RmmRec {
  pub attest_state: RmmRecAttestState,
  pub attest_challenge: [UInt64; 8],
  pub aux: [Address; 16],
  pub emulatable_abort: RmmRecEmulatableAbort,
  pub flags: RmmRecFlags,
  pub gprs: [UInt64; 32],
  pub mpidr: UInt64,
  pub owner: Address,
  pub pc: UInt64,
  pub psci_pending: RmmPsciPending,
  pub state: RmmRecState,
  pub sysregs: RmmSystemRegisters,
  pub ripas_addr: Address,
  pub ripas_top: Address,
  pub ripas_value: RmmRipas,
  pub ripas_destroyed: RmmRipasChangeDestroyed,
  pub ripas_response: RmmRecResponse,
  pub host_call_pending: RmmHostCallPending,
}

struct RmmRecFlags {
  pub runnable: RmmRecRunnable,
}

struct RmmRtt {
  pub entries: [RmmRttEntry; 512],
}

struct RmmRttEntry {
  pub addr: Address,
  pub ripas: RmmRipas,
  pub state: RmmRttEntryState,
  pub MemAttr: UInt3,
  pub S2AP: UInt2,
}

struct RmmRttWalkResult {
  pub level: int,
  pub rtt_addr: Address,
  pub rtte: RmmRttEntry,
}

pub open spec fn AddrInRange(s: S, addr: Address, base: Address, size: int) -> bool;

pub open spec fn AddrIsAligned(s: S, addr: Address, n: int) -> bool;

pub open spec fn AddrIsGranuleAligned(s: S, addr: Address) -> bool;

pub open spec fn AddrIsProtected(s: S, addr: Address, realm: RmmRealm) -> bool;

pub open spec fn AddrIsRttLevelAligned(s: S, addr: Address, level: int) -> bool;

pub open spec fn AddrRangeIsProtected(s: S, base: Address, top: Address, realm: RmmRealm) -> bool;

pub open spec fn AlignDownToRttLevel(s: S, addr: Address, level: int) -> Address;

pub open spec fn AlignUpToRttLevel(s: S, addr: Address, level: int) -> Address;

pub open spec fn AuxAlias(s: S, rec: Address, aux: [Address; 16], count: int) -> bool;

pub open spec fn AuxAligned(s: S, aux: [Address; 16], count: int) -> bool;

pub open spec fn AuxEqual(aux1: [Address; 16], aux2: [Address; 16], count: int) -> bool;

pub open spec fn AuxSort(s: S, addrs: [Address; 16], count: int) -> [Address; 16];

pub open spec fn AuxStateEqual(aux: [Address; 16], count: int, state: RmmGranuleState) -> bool;

pub open spec fn AuxStates(s: S, aux: [Address; 16], count: int);

pub open spec fn CurrentRealm(s: S) -> RmmRealm;

pub open spec fn CurrentRec(s: S) -> RmmRec;

pub open spec fn Equal(abstract_: RmmFeature, concrete: RmiFeature) -> bool;

pub open spec fn Gicv3ConfigIsValid(s: S, gicv3_hcr: u64, gicv3_lrs: [u64; 16]) -> bool;

pub open spec fn Granule(s: S, addr: Address) -> RmmGranule;

pub open spec fn GranuleAccessPermitted(s: S, addr: Address, pas: RmmPhysicalAddressSpace) -> bool;

pub open spec fn ImplFeatures(s: S) -> RmmFeatures;

pub open spec fn MinAddress(s: S, addr1: Address, addr2: Address) -> Address;

pub open spec fn MpidrEqual(rmm_mpidr: u64, rmi_mpidr: RmiRecMpidr) -> bool;

pub open spec fn MpidrIsUsed(s: S, mpidr: u64) -> bool;

pub open spec fn PaIsDelegable(s: S, addr: Address) -> bool;

pub open spec fn PsciReturnCodeEncode(s: S, value: PsciReturnCode) -> u64;

pub open spec fn PsciReturnCodePermitted(s: S, calling_rec: RmmRec, target_rec: RmmRec, value: PsciReturnCode) -> bool;

pub open spec fn ReadMemory(s: S, addr: u64, offset: int, size: int) -> [u8; 1];

pub open spec fn Realm(s: S, addr: Address) -> RmmRealm;

pub open spec fn RealmConfig(s: S, addr: Address) -> RsiRealmConfig;

pub open spec fn RealmHostCall(s: S, addr: Address) -> RsiHostCall;

pub open spec fn RealmIsLive(s: S, addr: Address) -> bool;

pub open spec fn RealmParams(s: S, addr: Address) -> RmiRealmParams;

pub open spec fn RealmParamsSupported(s: S, value: RmiRealmParams) -> bool;

pub open spec fn Rec(s: S, addr: Address) -> RmmRec;

pub open spec fn RecAuxCount(s: S, rd: Address) -> int;

pub open spec fn RecFromMpidr(s: S, mpidr: u64) -> RmmRec;

pub open spec fn RecIndex(s: S, mpidr: RmiRecMpidr) -> int;

pub open spec fn RecParams(s: S, addr: Address) -> RmiRecParams;

pub open spec fn RecRipasChangeResponse(s: S, rec: RmmRec) -> RsiResponse;

pub open spec fn RecRun(s: S, addr: Address) -> RmiRecRun;

pub open spec fn RemExtend(s: S, hash_algo: RmmHashAlgorithm, old_value: RmmRealmMeasurement, new_value: RmmRealmMeasurement, size: int) -> RmmRealmMeasurement;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, status: RmiStatusCode) -> bool {
    result.is_Err() && result.get_Err_0() == status
}

pub open spec fn RimExtendData(s: S, realm: RmmRealm, ipa: Address, data: Address, flags: RmiDataFlags) -> RmmRealmMeasurement;

pub open spec fn RimExtendRec(s: S, realm: RmmRealm, params: RmiRecParams) -> RmmRealmMeasurement;

pub open spec fn RimExtendRipas(s: S, realm: RmmRealm, base: Address, top: Address, level: int) -> RmmRealmMeasurement;

pub open spec fn RimExtendRipasForEntry(s: S, rim: RmmRealmMeasurement, ipa: Address, level: int) -> RmmRealmMeasurement;

pub open spec fn RimInit(s: S, hash_algo: RmmHashAlgorithm, params: RmiRealmParams) -> RmmRealmMeasurement;

pub open spec fn RipasToRmi(s: S, ripas: RmmRipas) -> RmiRipas;

pub open spec fn RmiRealmParamsIsValid(s: S, addr: Address) -> bool;

pub open spec fn Rtt(s: S, addr: Address) -> RmmRtt;

pub open spec fn RttAllEntriesContiguous(s: S, rtt: RmmRtt, addr: Address, level: int) -> bool;

pub open spec fn RttAllEntriesRipas(s: S, rtt: RmmRtt, ripas: RmmRipas) -> bool;

pub open spec fn RttAllEntriesState(s: S, rtt: RmmRtt, state: RmmRttEntryState) -> bool;

pub open spec fn RttConfigIsValid(s: S, ipa_width: int, rtt_level_start: int, rtt_num_start: int) -> bool;

pub open spec fn RttDescriptorIsValidForUnprotected(s: S, desc: u64) -> bool;

pub open spec fn RttEntriesInRangeRipas(s: S, rtt: RmmRtt, level: int, base: Address, top: Address, ripas: RmmRipas) -> bool;

pub open spec fn RttEntry(s: S, rtt: Address, i: int) -> RmmRttEntry;

pub open spec fn RttEntryFromDescriptor(s: S, desc: u64) -> RmmRttEntry;

pub open spec fn RttEntryIndex(s: S, addr: Address, level: int) -> int;

pub open spec fn RttEntryState(s: S, state: RmmRttEntryState) -> RmiRttEntryState;

pub open spec fn RttFold(s: S, rtt: RmmRtt) -> RmmRttEntry;

pub open spec fn RttIsHomogeneous(s: S, rtt: RmmRtt) -> bool;

pub open spec fn RttIsLive(s: S, rtt: RmmRtt) -> bool;

pub open spec fn RttLevelIsBlockOrPage(s: S, rd: Address, level: int) -> bool;

pub open spec fn RttLevelIsStarting(s: S, rd: Address, level: int) -> bool;

pub open spec fn RttLevelIsValid(s: S, rd: Address, level: int) -> bool;

pub open spec fn RttLevelSize(s: S, level: int) -> int;

pub open spec fn RttsAllProtectedEntriesRipas(s: S, rtt_base: Address, rtt_num_start: int, ripas: RmmRipas) -> bool;

pub open spec fn RttsAllProtectedEntriesState(s: S, rtt_base: Address, rtt_num_start: int, state: RmmRttEntryState) -> bool;

pub open spec fn RttsAllUnprotectedEntriesState(s: S, rtt_base: Address, rtt_num_start: int, state: RmmRttEntryState) -> bool;

pub open spec fn RttsGranuleState(s: S, rtt_base: Address, rtt_num_start: int);

pub open spec fn RttSkipEntriesUnlessRipas(s: S, rtt: RmmRtt, level: int, ipa: Address, ripas: RmmRipas) -> Address;

pub open spec fn RttSkipEntriesUnlessState(s: S, rtt: RmmRtt, level: int, ipa: Address, state: RmmRttEntryState) -> Address;

pub open spec fn RttSkipEntriesWithRipas(s: S, rtt: RmmRtt, level: int, base: Address, top: Address, stop_at_destroyed: bool) -> Address;

pub open spec fn RttSkipNonLiveEntries(s: S, rtt: RmmRtt, level: int, ipa: Address) -> Address;

pub open spec fn RttsStateEqual(rtt_base: Address, rtt_num_start: int, state: RmmGranuleState) -> bool;

pub open spec fn RttWalk(s: S, rd: Address, addr: Address) -> RmmRttWalkResult;

pub open spec fn ToAddress(value: int) -> Address;

pub open spec fn ToBits64(value: int) -> u64;

pub open spec fn VmidIsFree(s: S, vmid: u16) -> bool;

pub open spec fn VmidIsValid(s: S, vmid: u16) -> bool;

pub open spec fn rmi_data_create_spec(rd: Address, data: Address, ipa: Address, src: Address, flags: RmiDataFlags, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, src) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, src) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, src, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((data) >= 2^48)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Realm(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level < RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (result.is_Ok() ==> Granule(new_s, data).state == DATA)
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state == ASSIGNED)
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == RAM)
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr == data)
  && (result.is_Ok() ==> Realm(new_s, rd).measurements[0] == RimExtendData(new_s, Realm(new_s, rd), ipa, data, flags))
  && ((AddrIsGranuleAligned(old_s, src) &&
       PaIsDelegable(old_s, src) &&
       GranuleAccessPermitted(old_s, src, PAS_NS) &&
       AddrIsGranuleAligned(old_s, data) &&
       PaIsDelegable(old_s, data) &&
       !(Granule(old_s, data).state != DELEGATED) &&
       !(((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((data) >= 2^48))) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, Realm(old_s, rd)) &&
       !(Realm(old_s, rd).state != REALM_NEW) &&
       !(RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level < RMM_RTT_PAGE_LEVEL) &&
       !(RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state != UNASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Granule(new_s, data).state == Granule(old_s, data).state)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state == RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr == RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr)
  && (result.is_Err()
    ==> Realm(new_s, rd).measurements[0] == Realm(old_s, rd).measurements[0])
}

pub open spec fn rmi_data_create_unknown_spec(rd: Address, data: Address, ipa: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((data) >= 2^48)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level < RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (result.is_Ok() ==> Granule(new_s, data).state == DATA)
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state == ASSIGNED)
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr == data)
  && ((AddrIsGranuleAligned(old_s, data) &&
       PaIsDelegable(old_s, data) &&
       !(Granule(old_s, data).state != DELEGATED) &&
       !(((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((data) >= 2^48))) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, Realm(old_s, rd)) &&
       !(RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level < RMM_RTT_PAGE_LEVEL) &&
       !(RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state != UNASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Granule(new_s, data).state == Granule(old_s, data).state)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state == RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr == RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr)
  && (RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas)
}

pub open spec fn rmi_data_destroy_spec(rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, data: Address, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level < RMM_RTT_PAGE_LEVEL ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level,ipa))))
  && (RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state != ASSIGNED ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level,ipa))))
  && (result.is_Ok() ==> Granule(new_s, RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr).state == DELEGATED)
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state == UNASSIGNED)
  && (result.is_Ok() && RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == RAM ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == DESTROYED)
  && (result.is_Ok() ==> data == RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level,ipa))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, Realm(old_s, rd)) &&
       !(RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level < RMM_RTT_PAGE_LEVEL) &&
       !(RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state != ASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Granule(new_s, RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr).state == Granule(old_s, RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr).state)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state == RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas)
  && (!(result.is_Ok() && (RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == RAM)) ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas)
}

pub open spec fn rmi_features_spec(index: UInt64, result: Result<(), RmiStatusCode>, value: Bits64, old_s: S, new_s: S) -> bool {
  (result.is_Ok() && index != 0 ==> value == 0)
}

pub open spec fn rmi_granule_delegate_spec(addr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, addr).state != UNDELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, addr).gpt != GPT_NS ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> Granule(new_s, addr).state == DELEGATED)
  && (result.is_Ok() ==> Granule(new_s, addr).gpt == GPT_REALM)
  && ((AddrIsGranuleAligned(old_s, addr) &&
       PaIsDelegable(old_s, addr) &&
       !(Granule(old_s, addr).state != UNDELEGATED) &&
       !(Granule(old_s, addr).gpt != GPT_NS))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Granule(new_s, addr).state == Granule(old_s, addr).state)
  && (result.is_Err()
    ==> Granule(new_s, addr).gpt == Granule(old_s, addr).gpt)
}

pub open spec fn rmi_granule_undelegate_spec(addr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, addr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> Granule(new_s, addr).gpt == GPT_NS)
  && (result.is_Ok() ==> Granule(new_s, addr).state == UNDELEGATED)
  && ((AddrIsGranuleAligned(old_s, addr) &&
       PaIsDelegable(old_s, addr) &&
       !(Granule(old_s, addr).state != DELEGATED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Granule(new_s, addr).gpt == Granule(old_s, addr).gpt)
  && (result.is_Err()
    ==> Granule(new_s, addr).state == Granule(old_s, addr).state)
}

pub open spec fn rmi_psci_complete_spec(calling_rec: Address, target_rec: Address, status: PsciReturnCode, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (calling_rec == target_rec ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, calling_rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, calling_rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, calling_rec).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, target_rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, target_rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, target_rec).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Rec(old_s, calling_rec).psci_pending != PSCI_REQUEST_PENDING ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Rec(old_s, target_rec).owner != Rec(old_s, calling_rec).owner ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Rec(old_s, target_rec).mpidr != Rec(old_s, calling_rec).gprs[1] ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PsciReturnCodePermitted(old_s, Rec(old_s, calling_rec), Rec(old_s, target_rec), status) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> Rec(new_s, calling_rec).psci_pending == NO_PSCI_REQUEST_PENDING)
  && (result.is_Ok() && (status == PSCI_SUCCESS && Rec(old_s, calling_rec).gprs[0] == FID_PSCI_CPU_ON && Rec(old_s, target_rec).flags.runnable == RUNNABLE) ==> (Rec(new_s, calling_rec).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_ALREADY_ON)))
  && (result.is_Ok() && (status == PSCI_SUCCESS && Rec(old_s, calling_rec).gprs[0] == FID_PSCI_CPU_ON && Rec(old_s, target_rec).flags.runnable != RUNNABLE) ==> (Rec(new_s, target_rec).gprs[0] == Rec(new_s, calling_rec).gprs[3] && Rec(new_s, target_rec).gprs[1] == 0 && Rec(new_s, target_rec).gprs[2] == 0 && Rec(new_s, target_rec).gprs[3] == 0 && Rec(new_s, target_rec).gprs[4] == 0 && Rec(new_s, target_rec).gprs[5] == 0 && Rec(new_s, target_rec).gprs[6] == 0 && Rec(new_s, target_rec).gprs[7] == 0 && Rec(new_s, target_rec).gprs[8] == 0 && Rec(new_s, target_rec).gprs[9] == 0 && Rec(new_s, target_rec).gprs[10] == 0 && Rec(new_s, target_rec).gprs[11] == 0 && Rec(new_s, target_rec).gprs[12] == 0 && Rec(new_s, target_rec).gprs[13] == 0 && Rec(new_s, target_rec).gprs[14] == 0 && Rec(new_s, target_rec).gprs[15] == 0 && Rec(new_s, target_rec).gprs[16] == 0 && Rec(new_s, target_rec).gprs[17] == 0 && Rec(new_s, target_rec).gprs[18] == 0 && Rec(new_s, target_rec).gprs[19] == 0 && Rec(new_s, target_rec).gprs[20] == 0 && Rec(new_s, target_rec).gprs[21] == 0 && Rec(new_s, target_rec).gprs[22] == 0 && Rec(new_s, target_rec).gprs[23] == 0 && Rec(new_s, target_rec).gprs[24] == 0 && Rec(new_s, target_rec).gprs[25] == 0 && Rec(new_s, target_rec).gprs[26] == 0 && Rec(new_s, target_rec).gprs[27] == 0 && Rec(new_s, target_rec).gprs[28] == 0 && Rec(new_s, target_rec).gprs[29] == 0 && Rec(new_s, target_rec).gprs[30] == 0 && Rec(new_s, target_rec).gprs[31] == 0 && Rec(new_s, target_rec).pc == Rec(new_s, calling_rec).gprs[2] && Rec(new_s, target_rec).flags.runnable == RUNNABLE && Rec(new_s, calling_rec).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_SUCCESS)))
  && (result.is_Ok() && (status == PSCI_SUCCESS && Rec(old_s, calling_rec).gprs[0] == FID_PSCI_AFFINITY_INFO && Rec(old_s, target_rec).flags.runnable == RUNNABLE) ==> (Rec(new_s, calling_rec).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_SUCCESS)))
  && (result.is_Ok() && (status == PSCI_SUCCESS && Rec(old_s, calling_rec).gprs[0] == FID_PSCI_AFFINITY_INFO && Rec(old_s, target_rec).flags.runnable != RUNNABLE) ==> (Rec(new_s, calling_rec).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_OFF)))
  && (result.is_Ok() && status != PSCI_SUCCESS ==> (Rec(new_s, calling_rec).gprs[0] == PsciReturnCodeEncode(new_s, status)))
  && (result.is_Ok() ==> (Rec(new_s, calling_rec).gprs[1] == 0 && Rec(new_s, calling_rec).gprs[2] == 0 && Rec(new_s, calling_rec).gprs[3] == 0))
  && ((!(calling_rec == target_rec) &&
       AddrIsGranuleAligned(old_s, calling_rec) &&
       PaIsDelegable(old_s, calling_rec) &&
       !(Granule(old_s, calling_rec).state != REC) &&
       AddrIsGranuleAligned(old_s, target_rec) &&
       PaIsDelegable(old_s, target_rec) &&
       !(Granule(old_s, target_rec).state != REC) &&
       !(Rec(old_s, calling_rec).psci_pending != PSCI_REQUEST_PENDING) &&
       !(Rec(old_s, target_rec).owner != Rec(old_s, calling_rec).owner) &&
       !(Rec(old_s, target_rec).mpidr != Rec(old_s, calling_rec).gprs[1]) &&
       PsciReturnCodePermitted(old_s, Rec(old_s, calling_rec), Rec(old_s, target_rec), status))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Rec(new_s, calling_rec).psci_pending == Rec(old_s, calling_rec).psci_pending)
  && (result.is_Err()
    ==> Rec(new_s, calling_rec).gprs[0] == Rec(old_s, calling_rec).gprs[0])
  && (result.is_Err()
    ==> Rec(new_s, calling_rec).gprs[0] == Rec(old_s, calling_rec).gprs[0])
  && (result.is_Err()
    ==> Rec(new_s, calling_rec).gprs[0] == Rec(old_s, calling_rec).gprs[0])
  && (result.is_Err()
    ==> Rec(new_s, calling_rec).gprs[0] == Rec(old_s, calling_rec).gprs[0])
}

pub open spec fn rmi_realm_activate_spec(rd: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Realm(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (result.is_Ok() ==> Realm(new_s, rd).state == REALM_ACTIVE)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       !(Realm(old_s, rd).state != REALM_NEW))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Realm(new_s, rd).state == Realm(old_s, rd).state)
}

pub open spec fn rmi_realm_create_spec(rd: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RmiRealmParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RealmParamsSupported(old_s, RealmParams(old_s, params_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AddrInRange(old_s, rd, RealmParams(old_s, params_ptr).rtt_base,(RealmParams(old_s, params_ptr).rtt_num_start - 1) * RMM_GRANULE_SIZE) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, RealmParams(old_s, params_ptr).rtt_base,RealmParams(old_s, params_ptr).rtt_num_start * RMM_GRANULE_SIZE) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttConfigIsValid(old_s, RealmParams(old_s, params_ptr).s2sz as int,RealmParams(old_s, params_ptr).rtt_level_start as int, RealmParams(old_s, params_ptr).rtt_num_start as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttsStateEqual(RealmParams(old_s, params_ptr).rtt_base, RealmParams(old_s, params_ptr).rtt_num_start as int, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!VmidIsValid(old_s, RealmParams(old_s, params_ptr).vmid) || !VmidIsFree(old_s, RealmParams(old_s, params_ptr).vmid) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> Granule(new_s, rd).state == RD)
  && (result.is_Ok() ==> Realm(new_s, rd).state == REALM_NEW)
  && (result.is_Ok() ==> Realm(new_s, rd).rec_index == 0)
  && (result.is_Ok() ==> Realm(new_s, rd).rtt_base == RealmParams(new_s, params_ptr).rtt_base)
  && (result.is_Ok() ==> RttsStateEqual( Realm(new_s, rd).rtt_base, Realm(new_s, rd).rtt_num_start as int, RTT))
  && (result.is_Ok() ==> RttsAllProtectedEntriesState(new_s,  Realm(new_s, rd).rtt_base, Realm(new_s, rd).rtt_num_start as int, UNASSIGNED))
  && (result.is_Ok() ==> RttsAllUnprotectedEntriesState(new_s,  Realm(new_s, rd).rtt_base, Realm(new_s, rd).rtt_num_start as int, UNASSIGNED_NS))
  && (result.is_Ok() ==> RttsAllProtectedEntriesRipas(new_s,  Realm(new_s, rd).rtt_base, Realm(new_s, rd).rtt_num_start as int, EMPTY))
  && (result.is_Ok() ==> Equal(Realm(new_s, rd).feat_lpa2, RealmParams(new_s, params_ptr).flags.lpa2))
  && (result.is_Ok() ==> Realm(new_s, rd).ipa_width == RealmParams(new_s, params_ptr).s2sz)
  //&& (result.is_Ok() ==> Equal(Realm(new_s, rd).hash_algo, RealmParams(new_s, params_ptr).hash_algo))
  && (result.is_Ok() ==> Realm(new_s, rd).measurements[0] == RimInit(new_s,  Realm(new_s, rd).hash_algo, RealmParams(new_s, params_ptr)))
  //&& (result.is_Ok() ==> (Realm(new_s, rd).measurements[1] == 0 && Realm(new_s, rd).measurements[2] == 0 && Realm(new_s, rd).measurements[3] == 0 && Realm(new_s, rd).measurements[4] == 0))
  && (result.is_Ok() ==> Realm(new_s, rd).rtt_level_start == RealmParams(new_s, params_ptr).rtt_level_start)
  && (result.is_Ok() ==> Realm(new_s, rd).rtt_num_start == RealmParams(new_s, params_ptr).rtt_num_start)
  && (result.is_Ok() ==> Realm(new_s, rd).vmid == RealmParams(new_s, params_ptr).vmid)
  && (result.is_Ok() ==> Realm(new_s, rd).rpv == RealmParams(new_s, params_ptr).rpv)
  && (result.is_Ok() ==> Realm(new_s, rd).num_recs == 0)
  && ((AddrIsGranuleAligned(old_s, params_ptr) &&
       PaIsDelegable(old_s, params_ptr) &&
       GranuleAccessPermitted(old_s, params_ptr, PAS_NS) &&
       RmiRealmParamsIsValid(old_s, params_ptr) &&
       RealmParamsSupported(old_s, RealmParams(old_s, params_ptr)) &&
       !(AddrInRange(old_s, rd, RealmParams(old_s, params_ptr).rtt_base,(RealmParams(old_s, params_ptr).rtt_num_start - 1) * RMM_GRANULE_SIZE)) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != DELEGATED) &&
       AddrIsAligned(old_s, RealmParams(old_s, params_ptr).rtt_base,RealmParams(old_s, params_ptr).rtt_num_start * RMM_GRANULE_SIZE) &&
       RttConfigIsValid(old_s, RealmParams(old_s, params_ptr).s2sz as int,RealmParams(old_s, params_ptr).rtt_level_start as int, RealmParams(old_s, params_ptr).rtt_num_start as int) &&
       RttsStateEqual(RealmParams(old_s, params_ptr).rtt_base, RealmParams(old_s, params_ptr).rtt_num_start as int, DELEGATED) &&
       VmidIsValid(old_s, RealmParams(old_s, params_ptr).vmid) || !VmidIsFree(old_s, RealmParams(old_s, params_ptr).vmid))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Granule(new_s, rd).state == Granule(old_s, rd).state)
  && (result.is_Err()
    ==> Realm(new_s, rd).state == Realm(old_s, rd).state)
  && (result.is_Err()
    ==> Realm(new_s, rd).rec_index == Realm(old_s, rd).rec_index)
  && (result.is_Err()
    ==> Realm(new_s, rd).rtt_base == Realm(old_s, rd).rtt_base)
  && (result.is_Err()
    ==> Realm(new_s, rd).ipa_width == Realm(old_s, rd).ipa_width)
  && (result.is_Err()
    ==> Realm(new_s, rd).measurements[0] == Realm(old_s, rd).measurements[0])
  && (result.is_Err()
    ==> Realm(new_s, rd).rtt_level_start == Realm(old_s, rd).rtt_level_start)
  && (result.is_Err()
    ==> Realm(new_s, rd).rtt_num_start == Realm(old_s, rd).rtt_num_start)
  && (result.is_Err()
    ==> Realm(new_s, rd).vmid == Realm(old_s, rd).vmid)
  && (result.is_Err()
    ==> Realm(new_s, rd).rpv == Realm(old_s, rd).rpv)
  && (result.is_Err()
    ==> Realm(new_s, rd).num_recs == Realm(old_s, rd).num_recs)
}

pub open spec fn rmi_realm_destroy_spec(rd: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmIsLive(old_s, rd) ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (result.is_Ok() ==> RttsStateEqual(Realm(new_s, rd).rtt_base, Realm(new_s, rd).rtt_num_start as int, DELEGATED))
  && (result.is_Ok() ==> Granule(new_s, rd).state == DELEGATED)
  && (result.is_Ok() ==> VmidIsFree(new_s, Realm(new_s, rd).vmid))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       !(RealmIsLive(old_s, rd)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Granule(new_s, rd).state == Granule(old_s, rd).state)
}

pub open spec fn rmi_rec_aux_count_spec(rd: Address, result: Result<(), RmiStatusCode>, aux_count: UInt64, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> aux_count == RecAuxCount(new_s, rd))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD))
    ==> result.is_Ok())
}

pub open spec fn rmi_rec_create_spec(rd: Address, rec: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rec).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Realm(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (Realm(old_s, rd).num_recs == pow2(ImplFeatures(old_s).max_recs_order as nat) - 1 ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (RecIndex(old_s, RecParams(old_s, params_ptr).mpidr) != Realm(old_s, rd).rec_index ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecParams(old_s, params_ptr).num_aux != RecAuxCount(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AuxAligned(old_s, RecParams(old_s, params_ptr).aux, RecParams(old_s, params_ptr).num_aux as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AuxAlias(old_s, rec, RecParams(old_s, params_ptr).aux, RecParams(old_s, params_ptr).num_aux as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AuxStateEqual(RecParams(old_s, params_ptr).aux, RecParams(old_s, params_ptr).num_aux as int, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> Realm(new_s, rd).rec_index == Realm(new_s, rd).rec_index + 1)
  && (result.is_Ok() ==> Granule(new_s, rec).state == REC)
  && (result.is_Ok() ==> Rec(new_s, rec).owner == rd)
  && (result.is_Ok() ==> Rec(new_s, rec).attest_state == NO_ATTEST_IN_PROGRESS)
  && (result.is_Ok() ==> MpidrEqual(Rec(new_s, rec).mpidr, RecParams(new_s, params_ptr).mpidr))
  && (result.is_Ok() ==> Rec(new_s, rec).state == REC_READY)
  && (result.is_Ok() && RecParams(old_s, params_ptr).flags.runnable == RMI_RUNNABLE ==> Rec(new_s, rec).flags.runnable == RUNNABLE)
  && (result.is_Ok() && RecParams(old_s, params_ptr).flags.runnable == RMI_NOT_RUNNABLE ==> Rec(new_s, rec).flags.runnable == NOT_RUNNABLE)
  && (result.is_Ok() ==> (Rec(new_s, rec).gprs[0] == RecParams(new_s, params_ptr).gprs[0] && Rec(new_s, rec).gprs[1] == RecParams(new_s, params_ptr).gprs[1] && Rec(new_s, rec).gprs[2] == RecParams(new_s, params_ptr).gprs[2] && Rec(new_s, rec).gprs[3] == RecParams(new_s, params_ptr).gprs[3] && Rec(new_s, rec).gprs[4] == RecParams(new_s, params_ptr).gprs[4] && Rec(new_s, rec).gprs[5] == RecParams(new_s, params_ptr).gprs[5] && Rec(new_s, rec).gprs[6] == RecParams(new_s, params_ptr).gprs[6] && Rec(new_s, rec).gprs[7] == RecParams(new_s, params_ptr).gprs[7] && Rec(new_s, rec).gprs[8] == 0 && Rec(new_s, rec).gprs[9] == 0 && Rec(new_s, rec).gprs[10] == 0 && Rec(new_s, rec).gprs[11] == 0 && Rec(new_s, rec).gprs[12] == 0 && Rec(new_s, rec).gprs[13] == 0 && Rec(new_s, rec).gprs[14] == 0 && Rec(new_s, rec).gprs[15] == 0 && Rec(new_s, rec).gprs[16] == 0 && Rec(new_s, rec).gprs[17] == 0 && Rec(new_s, rec).gprs[18] == 0 && Rec(new_s, rec).gprs[19] == 0 && Rec(new_s, rec).gprs[20] == 0 && Rec(new_s, rec).gprs[21] == 0 && Rec(new_s, rec).gprs[22] == 0 && Rec(new_s, rec).gprs[23] == 0 && Rec(new_s, rec).gprs[24] == 0 && Rec(new_s, rec).gprs[25] == 0 && Rec(new_s, rec).gprs[26] == 0 && Rec(new_s, rec).gprs[27] == 0 && Rec(new_s, rec).gprs[28] == 0 && Rec(new_s, rec).gprs[29] == 0 && Rec(new_s, rec).gprs[30] == 0 && Rec(new_s, rec).gprs[31] == 0))
  && (result.is_Ok() ==> Rec(new_s, rec).pc == RecParams(new_s, params_ptr).pc)
  && (result.is_Ok() && RecParams(old_s, params_ptr).flags.runnable == RMI_RUNNABLE ==> Realm(new_s, rd).measurements[0] == RimExtendRec(new_s, Realm(new_s, rd), RecParams(new_s, params_ptr)))
  && (result.is_Ok() ==> AuxEqual( Rec(new_s, rec).aux, RecParams(new_s, params_ptr).aux, RecAuxCount(new_s, rd)))
  && (result.is_Ok() ==> AuxStateEqual( Rec(new_s, rec).aux, RecAuxCount(new_s, rd), REC_AUX))
  && (result.is_Ok() ==> Rec(new_s, rec).ripas_addr == 0)
  && (result.is_Ok() ==> Rec(new_s, rec).ripas_top == 0)
  && (result.is_Ok() ==> Rec(new_s, rec).host_call_pending == NO_HOST_CALL_PENDING)
  && (result.is_Ok() ==> Realm(new_s, rd).num_recs == Realm(new_s, rd).num_recs + 1)
  && ((AddrIsGranuleAligned(old_s, params_ptr) &&
       PaIsDelegable(old_s, params_ptr) &&
       GranuleAccessPermitted(old_s, params_ptr, PAS_NS) &&
       AddrIsGranuleAligned(old_s, rec) &&
       PaIsDelegable(old_s, rec) &&
       !(Granule(old_s, rec).state != DELEGATED) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       !(Realm(old_s, rd).state != REALM_NEW) &&
       !(Realm(old_s, rd).num_recs == pow2(ImplFeatures(old_s).max_recs_order as nat) - 1) &&
       !(RecIndex(old_s, RecParams(old_s, params_ptr).mpidr) != Realm(old_s, rd).rec_index) &&
       !(RecParams(old_s, params_ptr).num_aux != RecAuxCount(old_s, rd)) &&
       AuxAligned(old_s, RecParams(old_s, params_ptr).aux, RecParams(old_s, params_ptr).num_aux as int) &&
       !(AuxAlias(old_s, rec, RecParams(old_s, params_ptr).aux, RecParams(old_s, params_ptr).num_aux as int)) &&
       AuxStateEqual(RecParams(old_s, params_ptr).aux, RecParams(old_s, params_ptr).num_aux as int, DELEGATED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Realm(new_s, rd).rec_index == Realm(old_s, rd).rec_index)
  && (result.is_Err()
    ==> Granule(new_s, rec).state == Granule(old_s, rec).state)
  && (result.is_Err()
    ==> Rec(new_s, rec).owner == Rec(old_s, rec).owner)
  && (result.is_Err()
    ==> Rec(new_s, rec).attest_state == Rec(old_s, rec).attest_state)
  && (result.is_Err()
    ==> Rec(new_s, rec).state == Rec(old_s, rec).state)
  && (result.is_Err()
    ==> Rec(new_s, rec).flags.runnable == Rec(old_s, rec).flags.runnable)
  && (result.is_Err()
    ==> Rec(new_s, rec).flags.runnable == Rec(old_s, rec).flags.runnable)
  && (result.is_Err()
    ==> Rec(new_s, rec).pc == Rec(old_s, rec).pc)
  && (result.is_Err()
    ==> Realm(new_s, rd).measurements[0] == Realm(old_s, rd).measurements[0])
  && (result.is_Err()
    ==> Rec(new_s, rec).ripas_addr == Rec(old_s, rec).ripas_addr)
  && (result.is_Err()
    ==> Rec(new_s, rec).ripas_top == Rec(old_s, rec).ripas_top)
  && (result.is_Err()
    ==> Rec(new_s, rec).host_call_pending == Rec(old_s, rec).host_call_pending)
  && (result.is_Err()
    ==> Realm(new_s, rd).num_recs == Realm(old_s, rd).num_recs)
  && (!(result.is_Ok() && (RecParams(old_s, params_ptr).flags.runnable == RMI_RUNNABLE)) ==> Rec(new_s, rec).flags.runnable == Rec(old_s, rec).flags.runnable)
}

pub open spec fn rmi_rec_destroy_spec(rec_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Rec(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (result.is_Ok() ==> Granule(new_s, rec_ptr).state == DELEGATED)
  && (result.is_Ok() ==> AuxStateEqual(Rec(new_s, rec_ptr).aux, RecAuxCount(new_s, Rec(new_s, rec_ptr).owner), DELEGATED))
  && (result.is_Ok() ==> Realm(new_s, Rec(new_s, rec_ptr).owner).num_recs == Realm(new_s, Rec(new_s, rec_ptr).owner).num_recs - 1)
  && ((AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(Granule(old_s, rec_ptr).state != REC) &&
       !(Rec(old_s, rec_ptr).state == REC_RUNNING))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Granule(new_s, rec_ptr).state == Granule(old_s, rec_ptr).state)
  && (result.is_Err()
    ==> Realm(new_s, Rec(new_s, rec_ptr).owner).num_recs == Realm(old_s, Rec(old_s, rec_ptr).owner).num_recs)
}

pub open spec fn rmi_rec_enter_spec(rec: Address, run_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, run_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, run_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, run_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rec).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Realm(old_s, Rec(old_s, rec).owner).state == REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0 as int)))
  && (Realm(old_s, Rec(old_s, rec).owner).state == REALM_SYSTEM_OFF ==> ResultEqual(result, RMI_ERROR_REALM(1 as int)))
  && (Rec(old_s, rec).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (Rec(old_s, rec).flags.runnable == NOT_RUNNABLE ==> ResultEqual(result, RMI_ERROR_REC))
  && ((RecRun(old_s, run_ptr).enter.flags.emul_mmio == RMI_EMULATED_MMIO && Rec(old_s, rec).emulatable_abort != EMULATABLE_ABORT) ==> ResultEqual(result, RMI_ERROR_REC))
  && (!Gicv3ConfigIsValid(old_s, RecRun(old_s, run_ptr).enter.gicv3_hcr, RecRun(old_s, run_ptr).enter.gicv3_lrs) ==> ResultEqual(result, RMI_ERROR_REC))
  && (Rec(old_s, rec).psci_pending == PSCI_REQUEST_PENDING ==> ResultEqual(result, RMI_ERROR_REC))
  && ((AddrIsGranuleAligned(old_s, run_ptr) &&
       PaIsDelegable(old_s, run_ptr) &&
       GranuleAccessPermitted(old_s, run_ptr, PAS_NS) &&
       AddrIsGranuleAligned(old_s, rec) &&
       PaIsDelegable(old_s, rec) &&
       !(Granule(old_s, rec).state != REC) &&
       !(Realm(old_s, Rec(old_s, rec).owner).state == REALM_NEW) &&
       !(Realm(old_s, Rec(old_s, rec).owner).state == REALM_SYSTEM_OFF) &&
       !(Rec(old_s, rec).state == REC_RUNNING) &&
       !(Rec(old_s, rec).flags.runnable == NOT_RUNNABLE) &&
       !((RecRun(old_s, run_ptr).enter.flags.emul_mmio == RMI_EMULATED_MMIO && Rec(old_s, rec).emulatable_abort != EMULATABLE_ABORT)) &&
       Gicv3ConfigIsValid(old_s, RecRun(old_s, run_ptr).enter.gicv3_hcr, RecRun(old_s, run_ptr).enter.gicv3_lrs) &&
       !(Rec(old_s, rec).psci_pending == PSCI_REQUEST_PENDING))
    ==> result.is_Ok())
}

pub open spec fn rmi_rtt_create_spec(rd: Address, rtt: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!RttLevelIsValid(old_s, rd, level as int) || RttLevelIsStarting(old_s, rd, level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rtt).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((rtt) >= 2^48)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk_(old_s,rd, ipa,level - 1 as int).level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa,level - 1 as int).level as int)))
  && (RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.state == TABLE ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa,level - 1 as int).level as int)))
  && (result.is_Ok() ==> Granule(new_s, rtt).state == RTT)
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.state == TABLE)
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.addr == rtt)
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> RttAllEntriesRipas(new_s, Rtt(new_s, rtt), RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.ripas))
  && (result.is_Ok() ==> RttAllEntriesState(new_s, Rtt(new_s, rtt), RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.state))
  && (result.is_Ok() && (RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.state != UNASSIGNED && RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.state != UNASSIGNED_NS) ==> RttAllEntriesContiguous(new_s, Rtt(new_s, rtt), RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.addr, level as int))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       !((!RttLevelIsValid(old_s, rd, level as int) || RttLevelIsStarting(old_s, rd, level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat)) &&
       AddrIsGranuleAligned(old_s, rtt) &&
       PaIsDelegable(old_s, rtt) &&
       !(Granule(old_s, rtt).state != DELEGATED) &&
       !(((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((rtt) >= 2^48))) &&
       !(RttWalk_(old_s,rd, ipa,level - 1 as int).level < level - 1) &&
       !(RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.state == TABLE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Granule(new_s, rtt).state == Granule(old_s, rtt).state)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.state == RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.addr == RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.addr)
  && (RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.ripas == RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.ripas)
}

pub open spec fn rmi_rtt_destroy_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!RttLevelIsValid(old_s, rd, level as int) || RttLevelIsStarting(old_s, rd, level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk_(old_s,rd, ipa,level - 1 as int).level < level - 1 ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa,level - 1 as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa,level - 1 as int).rtt_addr),RttWalk_(new_s,rd, ipa,level - 1 as int).level,ipa))))
  && (RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.state != TABLE ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa,level - 1 as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa,level - 1 as int).rtt_addr),RttWalk_(new_s,rd, ipa,level - 1 as int).level,ipa))))
  && (RttIsLive(old_s, Rtt(old_s, RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.addr)) ==> (ResultEqual(result, RMI_ERROR_RTT(level as int)) && (top == ipa)))
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.state == UNASSIGNED)
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.ripas == DESTROYED)
  && (result.is_Ok() ==> Granule(new_s, RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.addr).state == DELEGATED)
  && (result.is_Ok() ==> rtt == RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.addr)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa,level - 1 as int).rtt_addr),RttWalk_(new_s,rd, ipa,level - 1 as int).level,ipa))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       !((!RttLevelIsValid(old_s, rd, level as int) || RttLevelIsStarting(old_s, rd, level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat)) &&
       !(RttWalk_(old_s,rd, ipa,level - 1 as int).level < level - 1) &&
       !(RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.state != TABLE) &&
       !(RttIsLive(old_s, Rtt(old_s, RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.addr))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.state == RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.ripas == RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.ripas)
  && (result.is_Err()
    ==> Granule(new_s, RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.addr).state == Granule(old_s, RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.addr).state)
}

pub open spec fn rmi_rtt_fold_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!RttLevelIsValid(old_s, rd, level as int) || RttLevelIsStarting(old_s, rd, level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk_(old_s,rd, ipa,level - 1 as int).level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa,level - 1 as int).level as int)))
  && (RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.state != TABLE ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa,level - 1 as int).level as int)))
  && (!RttIsHomogeneous(old_s, Rtt(old_s, RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.addr)) ==> ResultEqual(result, RMI_ERROR_RTT(level as int)))
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.state == RttFold(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.addr)).state)
  && (result.is_Ok() && (RttFold(old_s, Rtt(old_s, RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.addr)).state != UNASSIGNED && RttFold(old_s, Rtt(old_s, RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.addr)).state != UNASSIGNED_NS) ==> RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.addr == RttFold(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.addr)).addr)
  && (result.is_Ok() && (RttFold(old_s, Rtt(old_s, RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.addr)).state == ASSIGNED || RttFold(old_s, Rtt(old_s, RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.addr)).state == ASSIGNED_NS) ==> (RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.MemAttr == RttFold(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.addr)).MemAttr && RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.S2AP == RttFold(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.addr)).S2AP))
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.ripas == RttFold(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.addr)).ripas)
  && (result.is_Ok() ==> Granule(new_s, RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.addr).state == DELEGATED)
  && (result.is_Ok() ==> rtt == RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.addr)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       !((!RttLevelIsValid(old_s, rd, level as int) || RttLevelIsStarting(old_s, rd, level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat)) &&
       !(RttWalk_(old_s,rd, ipa,level - 1 as int).level < level - 1) &&
       !(RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.state != TABLE) &&
       RttIsHomogeneous(old_s, Rtt(old_s, RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.addr)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.state == RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.addr == RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.addr)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.ripas == RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.ripas)
  && (result.is_Err()
    ==> Granule(new_s, RttWalk_(new_s,rd, ipa,level - 1 as int).rtte.addr).state == Granule(old_s, RttWalk_(old_s,rd, ipa,level - 1 as int).rtte.addr).state)
}

pub open spec fn rmi_rtt_init_ripas_spec(rd: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ToAddress((top) - RMM_GRANULE_SIZE),Realm(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Realm(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (!AddrIsRttLevelAligned(old_s, base, RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level as int) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((base) == (RttSkipEntriesWithRipas(old_s, Rtt(old_s, RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level,base, top, false)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (result.is_Ok() ==> RttEntriesInRangeRipas(new_s,  Rtt(new_s, RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level,base, RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level,base, top, false), RAM))
  && (result.is_Ok() ==> Realm(new_s, rd).measurements[0] == RimExtendRipas(new_s, Realm(new_s, rd), base, RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level,base, top, false), RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level))
  && (result.is_Ok() ==> out_top == RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level,base, top, false))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       !((top) <= (base)) &&
       AddrIsProtected(old_s, ToAddress((top) - RMM_GRANULE_SIZE),Realm(old_s, rd)) &&
       !(Realm(old_s, rd).state != REALM_NEW) &&
       AddrIsRttLevelAligned(old_s, base, RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level as int) &&
       !(RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).rtte.state != UNASSIGNED) &&
       AddrIsGranuleAligned(old_s, top) &&
       !((base) == (RttSkipEntriesWithRipas(old_s, Rtt(old_s, RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level,base, top, false))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Realm(new_s, rd).measurements[0] == Realm(old_s, rd).measurements[0])
}

pub open spec fn rmi_rtt_map_unprotected_spec(rd: Address, ipa: Address, level: Int64, desc: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!RttDescriptorIsValidForUnprotected(old_s, desc) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttLevelIsBlockOrPage(old_s, rd, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, RttEntryFromDescriptor(old_s, desc).addr, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((RttEntryFromDescriptor(old_s, desc).addr) >= 2^48)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, Realm(old_s, rd))) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk_(old_s,rd, ipa, level as int).level < level ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa, level as int).level as int)))
  && (RttWalk_(old_s,rd, ipa, level as int).rtte.state != UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa, level as int).level as int)))
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa, level as int).rtte.state == ASSIGNED_NS)
  && (result.is_Ok() ==> (RttWalk_(new_s,rd, ipa, level as int).rtte.MemAttr == RttEntryFromDescriptor(new_s, desc).MemAttr && RttWalk_(new_s,rd, ipa, level as int).rtte.S2AP == RttEntryFromDescriptor(new_s, desc).S2AP && RttWalk_(new_s,rd, ipa, level as int).rtte.addr == RttEntryFromDescriptor(new_s, desc).addr))
  && ((RttDescriptorIsValidForUnprotected(old_s, desc) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       RttLevelIsBlockOrPage(old_s, rd, level as int) &&
       AddrIsRttLevelAligned(old_s, RttEntryFromDescriptor(old_s, desc).addr, level as int) &&
       !(((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((RttEntryFromDescriptor(old_s, desc).addr) >= 2^48))) &&
       AddrIsRttLevelAligned(old_s, ipa, level as int) &&
       !(((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, Realm(old_s, rd)))) &&
       !(RttWalk_(old_s,rd, ipa, level as int).level < level) &&
       !(RttWalk_(old_s,rd, ipa, level as int).rtte.state != UNASSIGNED_NS))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa, level as int).rtte.state == RttWalk_(old_s,rd, ipa, level as int).rtte.state)
  && (RttWalk_(new_s,rd, ipa, level as int).rtte.ripas == RttWalk_(old_s,rd, ipa, level as int).rtte.ripas)
}

pub open spec fn rmi_rtt_read_entry_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, walk_level: UInt64, state: RmiRttEntryState, desc: Bits64, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttLevelIsValid(old_s, rd, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> state == RttEntryState(new_s, RttWalk_(new_s,rd, ipa, level as int).rtte.state))
  && (result.is_Ok() && (RttWalk_(old_s,rd, ipa, level as int).rtte.state == UNASSIGNED || RttWalk_(old_s,rd, ipa, level as int).rtte.state == UNASSIGNED_NS) ==> (RttEntryFromDescriptor(new_s, desc).MemAttr == 0 && RttEntryFromDescriptor(new_s, desc).S2AP == 0 && RttEntryFromDescriptor(new_s, desc).addr == 0))
  && (result.is_Ok() && (RttWalk_(old_s,rd, ipa, level as int).rtte.state == ASSIGNED || RttWalk_(old_s,rd, ipa, level as int).rtte.state == TABLE) ==> (RttEntryFromDescriptor(new_s, desc).MemAttr == 0 && RttEntryFromDescriptor(new_s, desc).S2AP == 0 && RttEntryFromDescriptor(new_s, desc).addr == RttWalk_(new_s,rd, ipa, level as int).rtte.addr))
  && (result.is_Ok() && RttWalk_(old_s,rd, ipa, level as int).rtte.state == ASSIGNED_NS ==> (RttEntryFromDescriptor(new_s, desc).MemAttr == RttWalk_(new_s,rd, ipa, level as int).rtte.MemAttr && RttEntryFromDescriptor(new_s, desc).S2AP == RttWalk_(new_s,rd, ipa, level as int).rtte.S2AP && RttEntryFromDescriptor(new_s, desc).addr == RttWalk_(new_s,rd, ipa, level as int).rtte.addr))
  && (result.is_Ok() && (RttWalk_(old_s,rd, ipa, level as int).rtte.state == UNASSIGNED || RttWalk_(old_s,rd, ipa, level as int).rtte.state == ASSIGNED) ==> ripas == RipasToRmi(new_s, RttWalk_(new_s,rd, ipa, level as int).rtte.ripas))
  && (result.is_Ok() && (RttWalk_(old_s,rd, ipa, level as int).rtte.state == UNASSIGNED_NS || RttWalk_(old_s,rd, ipa, level as int).rtte.state == ASSIGNED_NS) ==> ripas == RMI_EMPTY)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       RttLevelIsValid(old_s, rd, level as int) &&
       AddrIsRttLevelAligned(old_s, ipa, level as int) &&
       !((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat)))
    ==> result.is_Ok())
}

pub open spec fn rmi_rtt_set_ripas_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Rec(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (Rec(old_s, rec_ptr).owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (base != Rec(old_s, rec_ptr).ripas_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) > (Rec(old_s, rec_ptr).ripas_top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!AddrIsRttLevelAligned(old_s, base, RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int) && RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtte.ripas != Rec(old_s, rec_ptr).ripas_value) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((base) == (RttSkipEntriesWithRipas(old_s, Rtt(old_s, RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top,Rec(old_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED)) && RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtte.ripas != Rec(old_s, rec_ptr).ripas_value) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (result.is_Ok() ==> RttEntriesInRangeRipas(new_s,  Rtt(new_s, RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top,Rec(new_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED),Rec(new_s, rec_ptr).ripas_value))
  && (result.is_Ok() ==> Rec(new_s, rec_ptr).ripas_addr == MinAddress(new_s, top, RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top,Rec(new_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED)))
  && (result.is_Ok() ==> out_top == MinAddress(new_s, top, RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top,Rec(new_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED)))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(Granule(old_s, rec_ptr).state != REC) &&
       !(Rec(old_s, rec_ptr).state == REC_RUNNING) &&
       !(Rec(old_s, rec_ptr).owner != rd) &&
       !((top) <= (base)) &&
       !(base != Rec(old_s, rec_ptr).ripas_addr) &&
       !((top) > (Rec(old_s, rec_ptr).ripas_top)) &&
       !((!AddrIsRttLevelAligned(old_s, base, RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int) && RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtte.ripas != Rec(old_s, rec_ptr).ripas_value)) &&
       AddrIsGranuleAligned(old_s, top) &&
       !(((base) == (RttSkipEntriesWithRipas(old_s, Rtt(old_s, RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top,Rec(old_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED)) && RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtte.ripas != Rec(old_s, rec_ptr).ripas_value)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Rec(new_s, rec_ptr).ripas_addr == Rec(old_s, rec_ptr).ripas_addr)
}

pub open spec fn rmi_rtt_unmap_unprotected_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttLevelIsBlockOrPage(old_s, rd, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, Realm(old_s, rd))) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk_(old_s,rd, ipa, level as int).level < level ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa, level as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa, level as int).rtt_addr),RttWalk_(new_s,rd, ipa, level as int).level,ipa))))
  && (RttWalk_(old_s,rd, ipa, level as int).rtte.state != ASSIGNED_NS ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa, level as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa, level as int).rtt_addr),RttWalk_(new_s,rd, ipa, level as int).level,ipa))))
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa, level as int).rtte.state == UNASSIGNED_NS)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa, level as int).rtt_addr),RttWalk_(new_s,rd, ipa, level as int).level,ipa))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       RttLevelIsBlockOrPage(old_s, rd, level as int) &&
       AddrIsRttLevelAligned(old_s, ipa, level as int) &&
       !(((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, Realm(old_s, rd)))) &&
       !(RttWalk_(old_s,rd, ipa, level as int).level < level) &&
       !(RttWalk_(old_s,rd, ipa, level as int).rtte.state != ASSIGNED_NS))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa, level as int).rtte.state == RttWalk_(old_s,rd, ipa, level as int).rtte.state)
  && (RttWalk_(new_s,rd, ipa, level as int).rtte.ripas == RttWalk_(old_s,rd, ipa, level as int).rtte.ripas)
}

pub open spec fn rmi_version_spec(req: RmiInterfaceVersion, result: Result<(), RmiStatusCode>, lower: RmiInterfaceVersion, higher: RmiInterfaceVersion, old_s: S, new_s: S) -> bool {
  true
}

pub open spec fn rsi_attestation_token_continue_spec(addr: Address, offset: UInt64, size: UInt64, result: RsiCommandReturnCode, len: UInt64, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (offset >= RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
  && (offset + size < offset ==> result == RSI_ERROR_INPUT)
  && (offset + size > RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
  && (CurrentRec(old_s).attest_state != ATTEST_IN_PROGRESS ==> result == RSI_ERROR_STATE)
  && ((AddrIsGranuleAligned(old_s, addr) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)) &&
       !(offset >= RMM_GRANULE_SIZE) &&
       !(offset + size < offset) &&
       !(offset + size > RMM_GRANULE_SIZE) &&
       !(CurrentRec(old_s).attest_state != ATTEST_IN_PROGRESS))
    ==> result == RSI_SUCCESS)
}

pub open spec fn rsi_attestation_token_init_spec(challenge_0: Bits64, challenge_1: Bits64, challenge_2: Bits64, challenge_3: Bits64, challenge_4: Bits64, challenge_5: Bits64, challenge_6: Bits64, challenge_7: Bits64, result: RsiCommandReturnCode, size: UInt64, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> CurrentRec(new_s).attest_state == ATTEST_IN_PROGRESS)
  && (result == RSI_SUCCESS ==> CurrentRec(new_s).attest_challenge == [challenge_0,challenge_1,challenge_2,challenge_3,challenge_4,challenge_5,challenge_6,challenge_7 ])
  && (result != RSI_SUCCESS
    ==> CurrentRec(new_s).attest_state == CurrentRec(old_s).attest_state)
  && (result != RSI_SUCCESS
    ==> CurrentRec(new_s).attest_challenge == CurrentRec(old_s).attest_challenge)
}

pub open spec fn rsi_features_spec(index: UInt64, result: RsiCommandReturnCode, value: Bits64, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> value == 0)
}

pub open spec fn rsi_host_call_spec(addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (!AddrIsAligned(old_s, addr, 256) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && ((AddrIsAligned(old_s, addr, 256) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)))
    ==> result == RSI_SUCCESS)
}

pub open spec fn rsi_ipa_state_get_spec(base: Address, top: Address, result: RsiCommandReturnCode, out_top: Address, ripas: RsiRipas, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT)
  && ((top) <= (base) ==> result == RSI_ERROR_INPUT)
  && (!AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && ((AddrIsGranuleAligned(old_s, base) &&
       AddrIsGranuleAligned(old_s, top) &&
       !((top) <= (base)) &&
       AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)))
    ==> result == RSI_SUCCESS)
}

pub open spec fn rsi_ipa_state_set_spec(base: Address, top: Address, ripas: RsiRipas, flags: RsiRipasChangeFlags, result: RsiCommandReturnCode, new_base: Address, response: RsiResponse, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT)
  && ((top) <= (base) ==> result == RSI_ERROR_INPUT)
  && (!AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && ((ripas != RSI_EMPTY) && (ripas != RSI_RAM) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> new_base == CurrentRec(new_s).ripas_addr)
  && (result == RSI_SUCCESS ==> response == RecRipasChangeResponse(new_s, CurrentRec(new_s)))
  && ((AddrIsGranuleAligned(old_s, base) &&
       AddrIsGranuleAligned(old_s, top) &&
       !((top) <= (base)) &&
       AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) &&
       !((ripas != RSI_EMPTY) && (ripas != RSI_RAM)))
    ==> result == RSI_SUCCESS)
}

pub open spec fn rsi_measurement_extend_spec(index: UInt64, size: UInt64, value_0: Bits64, value_1: Bits64, value_2: Bits64, value_3: Bits64, value_4: Bits64, value_5: Bits64, value_6: Bits64, value_7: Bits64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (index < 1 || index > 4 ==> result == RSI_ERROR_INPUT)
  && (size > 64 ==> result == RSI_ERROR_INPUT)
  //&& (result == RSI_SUCCESS ==> CurrentRealm(new_s).measurements[index] == RemExtend(new_s, CurrentRealm(new_s).hash_algo, CurrentRealm(new_s).measurements[index], [value_0, value_1, value_2, value_3,value_4, value_5, value_6, value_7][ (RMM_REALM_MEASUREMENT_WIDTH-1):0],size))
  && ((!(index < 1 || index > 4) &&
       !(size > 64))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> CurrentRealm(new_s).measurements[index as int] == CurrentRealm(old_s).measurements[index as int])
}

pub open spec fn rsi_measurement_read_spec(index: UInt64, result: RsiCommandReturnCode, value_0: Bits64, value_1: Bits64, value_2: Bits64, value_3: Bits64, value_4: Bits64, value_5: Bits64, value_6: Bits64, value_7: Bits64, old_s: S, new_s: S) -> bool {
  (index > 4 ==> result == RSI_ERROR_INPUT)
  && ((!(index > 4))
    ==> result == RSI_SUCCESS)
}

pub open spec fn rsi_realm_config_spec(addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> RealmConfig(new_s, addr).ipa_width == CurrentRealm(new_s).ipa_width)
  //&& (result == RSI_SUCCESS ==> Equal(RealmConfig(new_s, addr).hash_algo, CurrentRealm(new_s).hash_algo))
  && ((AddrIsGranuleAligned(old_s, addr) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RealmConfig(new_s, addr).ipa_width == RealmConfig(old_s, addr).ipa_width)
}

pub open spec fn rsi_version_spec(req: RsiInterfaceVersion, result: RsiCommandReturnCode, lower: RsiInterfaceVersion, higher: RsiInterfaceVersion, old_s: S, new_s: S) -> bool {
  true
}

pub open spec fn psci_affinity_info_spec(target_affinity: Bits64, lowest_affinity_level: UInt32, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (lowest_affinity_level != 0 ==> result == PSCI_INVALID_PARAMETERS)
  && (!MpidrIsUsed(old_s, target_affinity) ==> result == PSCI_INVALID_PARAMETERS)
  && (result == PSCI_SUCCESS && RecFromMpidr(old_s, target_affinity).flags.runnable == RUNNABLE ==> result == PSCI_SUCCESS)
  && (result == PSCI_SUCCESS && RecFromMpidr(old_s, target_affinity).flags.runnable == NOT_RUNNABLE ==> result == PSCI_OFF)
  && ((!(lowest_affinity_level != 0) &&
       MpidrIsUsed(old_s, target_affinity))
    ==> result == PSCI_SUCCESS)
}

pub open spec fn psci_cpu_off_spec(old_s: S, new_s: S) -> bool {
  true
}

pub open spec fn psci_cpu_on_spec(target_cpu: Bits64, entry_point_address: Address, context_id: UInt32, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!AddrIsProtected(old_s, entry_point_address, CurrentRealm(old_s)) ==> result == PSCI_INVALID_ADDRESS)
  && (!MpidrIsUsed(old_s, target_cpu) ==> result == PSCI_INVALID_PARAMETERS)
  && (RecFromMpidr(old_s, target_cpu).flags.runnable == RUNNABLE ==> result == PSCI_ALREADY_ON)
  && (result == PSCI_SUCCESS ==> RecFromMpidr(new_s, target_cpu).pc == ToBits64((entry_point_address) as int))
  && (result == PSCI_SUCCESS ==> RecFromMpidr(new_s, target_cpu).flags.runnable == RUNNABLE)
  && ((AddrIsProtected(old_s, entry_point_address, CurrentRealm(old_s)) &&
       MpidrIsUsed(old_s, target_cpu) &&
       !(RecFromMpidr(old_s, target_cpu).flags.runnable == RUNNABLE))
    ==> result == PSCI_SUCCESS)
  && (result != PSCI_SUCCESS
    ==> RecFromMpidr(new_s, target_cpu).pc == RecFromMpidr(old_s, target_cpu).pc)
  && (result != PSCI_SUCCESS
    ==> RecFromMpidr(new_s, target_cpu).flags.runnable == RecFromMpidr(old_s, target_cpu).flags.runnable)
}

pub open spec fn psci_cpu_suspend_spec(power_state: UInt32, entry_point_address: Address, context_id: UInt64, old_s: S, new_s: S) -> bool {
  true
}

pub open spec fn psci_features_spec(psci_func_id: UInt32, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  true
}

pub open spec fn psci_system_off_spec(old_s: S, new_s: S) -> bool {
  (CurrentRealm(new_s).state == REALM_SYSTEM_OFF)
}

pub open spec fn psci_system_reset_spec(old_s: S, new_s: S) -> bool {
  (CurrentRealm(new_s).state == REALM_SYSTEM_OFF)
}

pub open spec fn psci_version_spec(result: PsciInterfaceVersion, old_s: S, new_s: S) -> bool {
  true
}

pub proof fn rmi_data_create_rule (rd: Address, data: Address, ipa: Address, src: Address, flags: RmiDataFlags, result: Result<(), RmiStatusCode>, old_s: S, new_s: S)
    requires rmi_data_create_spec(rd, data, ipa, src, flags, result, old_s, new_s),
{
  let old_walk = RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int);
  let new_walk = RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int);
  assert(result.is_Ok() ==> (true));
  assert(result.is_Ok() ==> (old_walk.rtte.state == UNASSIGNED));
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == RAM));
  assert(result.is_Ok() ==> (new_walk.rtte.state == ASSIGNED));
}

pub proof fn rmi_data_create_unknown_rule (rd: Address, data: Address, ipa: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S)
    requires rmi_data_create_unknown_spec(rd, data, ipa, result, old_s, new_s),
{
  let old_walk = RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int);
  let new_walk = RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int);
  assert(result.is_Ok() ==> (true));
  assert(result.is_Ok() ==> (old_walk.rtte.state == UNASSIGNED));
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert(result.is_Ok() ==> (new_walk.rtte.state == ASSIGNED));
}

pub proof fn rmi_data_destroy_rule (rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, data: Address, top: Address, old_s: S, new_s: S)
    requires rmi_data_destroy_spec(rd, ipa, result, data, top, old_s, new_s),
{
  let old_walk = RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int);
  let new_walk = RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int);
  assert(result.is_Ok() ==> (old_walk.rtte.ripas == EMPTY || old_walk.rtte.ripas == RAM)); // XXX: TP
  assert(result.is_Ok() ==> (old_walk.rtte.state == ASSIGNED));
  assert((result.is_Ok() && old_walk.rtte.ripas == EMPTY) ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert((result.is_Ok() && old_walk.rtte.ripas == RAM) ==> (new_walk.rtte.ripas == DESTROYED));
  assert(result.is_Ok() ==> (new_walk.rtte.state == UNASSIGNED));
}

pub proof fn rmi_rtt_create_rule (rd: Address, rtt: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S)
    requires rmi_rtt_create_spec(rd, rtt, ipa, level, result, old_s, new_s),
{
  let old_walk = RttWalk_(old_s, rd, ipa,level - 1 as int);
  let new_walk = RttWalk_(new_s, rd, ipa,level - 1 as int);
  assert(result.is_Ok() ==> (true));
  assert(result.is_Ok() ==> (true));
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert(result.is_Ok() ==> (new_walk.rtte.state == old_walk.rtte.state)); // XXX: FP
}

pub proof fn rmi_rtt_destroy_rule (rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, top: Address, old_s: S, new_s: S)
    requires rmi_rtt_destroy_spec(rd, ipa, level, result, rtt, top, old_s, new_s),
{
  let old_walk = RttWalk_(old_s, rd, ipa,level - 1 as int);
  let new_walk = RttWalk_(new_s, rd, ipa,level - 1 as int);
  assert(result.is_Ok() ==> (true));
  // Unsupported
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == DESTROYED));
  assert(result.is_Ok() ==> (new_walk.rtte.state == old_walk.rtte.state)); // XXX: TP
}

pub proof fn rmi_rtt_fold_rule (rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S)
    requires rmi_rtt_fold_spec(rd, ipa, level, result, rtt, old_s, new_s),
{
  let old_walk = RttWalk_(old_s, rd, ipa,level - 1 as int);
  let new_walk = RttWalk_(new_s, rd, ipa,level - 1 as int);
  // Unsupported
  // Unsupported
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == old_walk.rtte.ripas)); // XXX: FP
  assert(result.is_Ok() ==> (new_walk.rtte.state == old_walk.rtte.state)); // XXX: FP
}

pub proof fn rmi_rtt_init_ripas_rule (rd: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S)
    requires rmi_rtt_init_ripas_spec(rd, base, top, result, out_top, old_s, new_s),
{
  let old_walk = RttWalk_(old_s, rd,base,RMM_RTT_PAGE_LEVEL as int);
  let new_walk = RttWalk_(new_s, rd,base,RMM_RTT_PAGE_LEVEL as int);
  assert(result.is_Ok() ==> (true));
  assert(result.is_Ok() ==> (old_walk.rtte.state == UNASSIGNED));
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == RAM)); // XXX: FP
  assert(result.is_Ok() ==> (new_walk.rtte.state == old_walk.rtte.state)); // XXX: FP
}

pub proof fn rmi_rtt_set_ripas_rule (rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S)
    requires rmi_rtt_set_ripas_spec(rd, rec_ptr, base, top, result, out_top, old_s, new_s),
{
  let old_walk = RttWalk_(old_s, rd, base,RMM_RTT_PAGE_LEVEL as int);
  let new_walk = RttWalk_(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int);
  // Unsupported
  assert(result.is_Ok() ==> (true));
  // Unsupported
  assert(result.is_Ok() ==> (new_walk.rtte.state == old_walk.rtte.state)); // XXX: FP
}

fn main() {
}

}
