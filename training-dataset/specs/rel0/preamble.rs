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

