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

use crate::RmiBoolean::*;
use crate::RmiDataMeasureContent::*;
use crate::RmiDevCommProtocol::*;
use crate::RmiDevCommStatus::*;
use crate::RmiEmulatedMmio::*;
use crate::RmiFeature::*;
use crate::RmiHashAlgorithm::*;
use crate::RmiInjectSea::*;
use crate::RmiLfaPolicy::*;
use crate::RmiPdevEvent::*;
use crate::RmiPdevProtection::*;
use crate::RmiPdevState::*;
use crate::RmiPmuOverflowStatus::*;
use crate::RmiRecExitReason::*;
use crate::RmiRecRunnable::*;
use crate::RmiResponse::*;
use crate::RmiRipas::*;
use crate::RmiRttEntryState::*;
use crate::RmiRttPlaneFeature::*;
use crate::RmiRttS2APEncoding::*;
use crate::RmiSignatureAlgorithm::*;
use crate::RmiSmmuAction::*;
use crate::RmiSmmuIrq::*;
use crate::RmiStatusCode::*;
use crate::RmiTrap::*;
use crate::RmiVdevAction::*;
use crate::RmiVdevState::*;
use crate::RsiBoolean::*;
use crate::RsiCommandReturnCode::*;
use crate::RsiDeviceAttestationType::*;
use crate::RsiDeviceState::*;
use crate::RsiDevMemCoherent::*;
use crate::RsiDevMemOrdering::*;
use crate::RsiFeature::*;
use crate::RsiGicOwner::*;
use crate::RsiHashAlgorithm::*;
use crate::RsiPlaneExitReason::*;
use crate::RsiResponse::*;
use crate::RsiRipas::*;
use crate::RsiRipasChangeDestroyed::*;
use crate::RsiTrap::*;
use crate::PsciReturnCode::*;
use crate::RmmBoolean::*;
use crate::RmmDataMeasureContent::*;
use crate::RmmDevCommState::*;
use crate::RmmDevMemCoherent::*;
use crate::RmmDevMemOrdering::*;
use crate::RmmFeature::*;
use crate::RmmGptEntry::*;
use crate::RmmGranuleState::*;
use crate::RmmHashAlgorithm::*;
use crate::RmmHipas::*;
use crate::RmmLfaPolicy::*;
use crate::RmmMecPolicy::*;
use crate::RmmMecState::*;
use crate::RmmMemPermLocked::*;
use crate::RmmPdevProtection::*;
use crate::RmmPdevState::*;
use crate::RmmPhysicalAddressSpace::*;
use crate::RmmRdevOperation::*;
use crate::RmmRdevState::*;
use crate::RmmRealmState::*;
use crate::RmmRecAttestState::*;
use crate::RmmRecEmulatableAbort::*;
use crate::RmmRecPending::*;
use crate::RmmRecResponse::*;
use crate::RmmRecRunnable::*;
use crate::RmmRecState::*;
use crate::RmmRipas::*;
use crate::RmmRipasChangeDestroyed::*;
use crate::RmmRttEntryState::*;
use crate::RmmRttMemAttr::*;
use crate::RmmRttPlaneFeature::*;
use crate::RmmRttProtected::*;
use crate::RmmRttS2APEncoding::*;
use crate::RmmRttShareability::*;
use crate::RmmVdevState::*;
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

pub open spec fn VersionEqualRsi(ver1: RsiInterfaceVersion, ver2: RsiInterfaceVersion) -> bool;
pub open spec fn VersionEqualRmi(ver1: RmiInterfaceVersion, ver2: RmiInterfaceVersion) -> bool;
pub open spec fn RttWalk_(s: S, rd: Address, addr: Address, level: int) -> RmmRttWalkResult;

pub enum RmiBoolean {
  RMI_FALSE,
  RMI_TRUE,
}

pub enum RmiDataMeasureContent {
  RMI_NO_MEASURE_CONTENT,
  RMI_MEASURE_CONTENT,
}

pub enum RmiDevCommProtocol {
  RMI_PROTOCOL_SPDM,
  RMI_PROTOCOL_SECURE_SPDM,
}

pub enum RmiDevCommStatus {
  RMI_DEV_COMM_NONE,
  RMI_DEV_COMM_RESPONSE,
  RMI_DEV_COMM_ERROR,
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

pub enum RmiLfaPolicy {
  RMI_LFA_DISALLOW,
  RMI_LFA_ALLOW,
}

pub enum RmiPdevEvent {
  RMI_IDE_KEY_REFRESH,
}

pub enum RmiPdevProtection {
  RMI_PDEV_IOCOH_E2E_IDE,
  RMI_PDEV_IOCOH_E2E_SYS,
  RMI_PDEV_FCOH_E2E_IDE,
  RMI_PDEV_FCOH_E2E_SYS,
}

pub enum RmiPdevState {
  RMI_PDEV_NEW,
  RMI_PDEV_NEEDS_KEY,
  RMI_PDEV_HAS_KEY,
  RMI_PDEV_READY,
  RMI_PDEV_COMMUNICATING,
  RMI_PDEV_STOPPING,
  RMI_PDEV_STOPPED,
  RMI_PDEV_ERROR,
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
  RMI_EXIT_S2AP_CHANGE,
  RMI_EXIT_VDEV_REQUEST,
  RMI_EXIT_VDEV_COMM,
  RMI_EXIT_DEV_MEM_MAP,
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
  RMI_DEV,
}

pub enum RmiRttEntryState {
  RMI_UNASSIGNED,
  RMI_ASSIGNED,
  RMI_TABLE,
  RMI_ASSIGNED_DEV,
  RMI_AUX_DESTROYED,
  RMI_ASSIGNED_VSMMU,
}

pub enum RmiRttPlaneFeature {
  RMI_RTT_PLANE_AUX,
  RMI_RTT_PLANE_AUX_SINGLE,
  RMI_RTT_PLANE_SINGLE,
}

pub enum RmiRttS2APEncoding {
  RMI_S2AP_DIRECT,
  RMI_S2AP_INDIRECT,
}

pub enum RmiSignatureAlgorithm {
  RMI_SIG_RSASSA_3072,
  RMI_SIG_ECDSA_P256,
  RMI_SIG_ECDSA_P384,
}

pub enum RmiSmmuAction {
  RMI_SMMU_ACTION_NONE,
  RMI_SMMU_ACTION_VIRQ,
}

pub enum RmiSmmuIrq {
  RMI_SMMU_IRQ_GERROR,
  RMI_SMMU_IRQ_EVENTQ,
  RMI_SMMU_IRQ_PRIQ,
}

pub enum RmiStatusCode {
  RMI_ERROR_INPUT,
  RMI_ERROR_REALM(int),
  RMI_ERROR_REC,
  RMI_ERROR_RTT(int),
  RMI_ERROR_NOT_SUPPORTED,
  RMI_ERROR_DEVICE,
  RMI_ERROR_RTT_AUX(int),
}

pub enum RmiTrap {
  RMI_NO_TRAP,
  RMI_TRAP,
}

pub enum RmiVdevAction {
  RMI_VDEV_ACTION_GET_INTERFACE_REPORT,
  RMI_VDEV_ACTION_GET_MEASUREMENTS,
  RMI_VDEV_ACTION_LOCK,
  RMI_VDEV_ACTION_START,
  RMI_VDEV_ACTION_STOP,
}

pub enum RmiVdevState {
  RMI_VDEV_NEW,
  RMI_VDEV_READY,
  RMI_VDEV_COMMUNICATING,
  RMI_VDEV_STOPPING,
  RMI_VDEV_STOPPED,
  RMI_VDEV_ERROR,
}

pub enum RsiBoolean {
  RSI_FALSE,
  RSI_TRUE,
}

pub enum RsiCommandReturnCode {
  RSI_SUCCESS,
  RSI_ERROR_INPUT,
  RSI_ERROR_STATE,
  RSI_INCOMPLETE,
  RSI_ERROR_UNKNOWN,
  RSI_ERROR_DEVICE,
}

pub enum RsiDeviceAttestationType {
  RSI_INDEPENDENTLY_ATTESTED,
  RSI_PLATFORM_ATTESTED,
}

pub enum RsiDeviceState {
  RSI_RDEV_UNLOCKED,
  RSI_RDEV_UNLOCKED_BUSY,
  RSI_RDEV_LOCKED,
  RSI_RDEV_LOCKED_BUSY,
  RSI_RDEV_STARTED,
  RSI_RDEV_STARTED_BUSY,
  RSI_RDEV_STOPPING,
  RSI_RDEV_STOPPED,
  RSI_RDEV_ERROR,
}

pub enum RsiDevMemCoherent {
  RSI_DEV_MEM_NON_COHERENT,
  RSI_DEV_MEM_COHERENT,
}

pub enum RsiDevMemOrdering {
  RSI_DEV_MEM_NOT_LIMITED_ORDER,
  RSI_DEV_MEM_LIMITED_ORDER,
}

pub enum RsiFeature {
  RSI_FEATURE_FALSE,
  RSI_FEATURE_TRUE,
}

pub enum RsiGicOwner {
  RSI_GIC_OWNER_0,
  RSI_GIC_OWNER_N,
}

pub enum RsiHashAlgorithm {
  RSI_HASH_SHA_256,
  RSI_HASH_SHA_512,
}

pub enum RsiPlaneExitReason {
  RSI_EXIT_SYNC,
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

pub enum RsiTrap {
  RSI_NO_TRAP,
  RSI_TRAP,
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

pub enum RmmBoolean {
  RMM_FALSE,
  RMM_TRUE,
}

pub enum RmmDataMeasureContent {
  NO_MEASURE_CONTENT,
  MEASURE_CONTENT,
}

pub enum RmmDevCommState {
  DEV_COMM_ACTIVE,
  DEV_COMM_ERROR,
  DEV_COMM_IDLE,
  DEV_COMM_PENDING,
}

pub enum RmmDevMemCoherent {
  DEV_MEM_COHERENT,
  DEV_MEM_NON_COHERENT,
}

pub enum RmmDevMemOrdering {
  DEV_MEM_LIMITED_ORDER,
  DEV_MEM_NOT_LIMITED_ORDER,
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
  DEV_MAPPED,
  PDEV,
  PDEV_AUX,
  RD,
  REC,
  REC_AUX,
  RTT,
  UNDELEGATED,
  VDEV,
  VDEV_AUX,
  VSMMU,
}

pub enum RmmHashAlgorithm {
  HASH_SHA_256,
  HASH_SHA_512,
}

pub enum RmmHipas {
  HIPAS_ASSIGNED,
  HIPAS_ASSIGNED_DEV,
  HIPAS_ASSIGNED_NS,
  HIPAS_ASSIGNED_VSMMU,
  HIPAS_UNASSIGNED,
  HIPAS_UNASSIGNED_NS,
}

pub enum RmmLfaPolicy {
  LFA_ALLOW,
  LFA_DISALLOW,
}

pub enum RmmMecPolicy {
  MEC_POLICY_PRIVATE,
  MEC_POLICY_SHARED,
}

pub enum RmmMecState {
  MEC_STATE_PRIVATE_ASSIGNED,
  MEC_STATE_PRIVATE_UNASSIGNED,
  MEC_STATE_SHARED,
}

pub enum RmmMemPermLocked {
  MEM_PERM_LOCKED,
  MEM_PERM_UNLOCKED,
}

pub enum RmmPdevProtection {
  PDEV_FCOH_E2E_IDE,
  PDEV_FCOH_E2E_SYS,
  PDEV_IOCOH_E2E_IDE,
  PDEV_IOCOH_E2E_SYS,
}

pub enum RmmPdevState {
  PDEV_COMMUNICATING,
  PDEV_ERROR,
  PDEV_HAS_KEY,
  PDEV_IDE_RESETTING,
  PDEV_NEEDS_KEY,
  PDEV_NEW,
  PDEV_READY,
  PDEV_STOPPED,
  PDEV_STOPPING,
}

pub enum RmmPhysicalAddressSpace {
  PAS_NS,
  PAS_REALM,
  PAS_ROOT,
  PAS_SECURE,
}

pub enum RmmRdevOperation {
  RDEV_OP_GET_INTERFACE_REPORT,
  RDEV_OP_GET_MEASUREMENTS,
  RDEV_OP_LOCK,
  RDEV_OP_NONE,
  RDEV_OP_START,
}

pub enum RmmRdevState {
  RDEV_ERROR,
  RDEV_LOCKED,
  RDEV_LOCKED_BUSY,
  RDEV_STARTED,
  RDEV_STARTED_BUSY,
  RDEV_STOPPED,
  RDEV_STOPPING,
  RDEV_UNLOCKED,
  RDEV_UNLOCKED_BUSY,
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

pub enum RmmRecPending {
  REC_PENDING_HOST_CALL,
  REC_PENDING_NONE,
  REC_PENDING_PSCI,
  REC_PENDING_VDEV_REQUEST,
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
  ASSIGNED_DEV,
  ASSIGNED_NS,
  ASSIGNED_VSMMU,
  AUX_DESTROYED,
  TABLE,
  UNASSIGNED,
  UNASSIGNED_NS,
}

pub enum RmmRttMemAttr {
  MEMATTR_CACHEABLE,
  MEMATTR_NON_CACHEABLE,
  MEMATTR_PASSTHROUGH,
}

pub enum RmmRttPlaneFeature {
  RTT_PLANE_AUX,
  RTT_PLANE_AUX_SINGLE,
  RTT_PLANE_SINGLE,
}

pub enum RmmRttProtected {
  RTT_PROTECTED,
  RTT_UNPROTECTED,
}

pub enum RmmRttS2APEncoding {
  S2AP_DIRECT,
  S2AP_INDIRECT,
}

pub enum RmmRttShareability {
  SHAREABILITY_INNER,
  SHAREABILITY_OUTER,
}

pub enum RmmVdevState {
  VDEV_COMMUNICATING,
  VDEV_ERROR,
  VDEV_NEW,
  VDEV_READY,
  VDEV_STOPPED,
  VDEV_STOPPING,
}

struct RmiAddressRange {
  pub base: Address,
  pub top: Address,
}

struct RmiCommandReturnCode {
  pub status: RmiStatusCode,
  pub index: UInt8,
}

struct RmiDataFlags {
  pub measure: RmiDataMeasureContent,
}

struct RmiDevCommData {
  pub enter: RmiDevCommEnter,
  pub exit: RmiDevCommExit,
}

struct RmiDevCommEnter {
  pub status: RmiDevCommStatus,
  pub req_addr: Address,
  pub resp_addr: Address,
  pub resp_len: UInt64,
}

struct RmiDevCommExit {
  pub flags: RmiDevCommExitFlags,
  pub cache_offset: UInt64,
  pub cache_len: UInt64,
  pub protocol: RmiDevCommProtocol,
  pub req_len: UInt64,
}

struct RmiDevCommExitFlags {
  pub cache: RmiBoolean,
  pub send: RmiBoolean,
  pub wait: RmiBoolean,
  pub multi: RmiBoolean,
}

struct RmiFeatureRegister0 {
  pub S2SZ: UInt8,
  pub LPA2: RmiFeature,
  pub SVE: RmiFeature,
  pub SVE_VL: UInt4,
  pub NUM_BPS: UInt6,
  pub NUM_WPS: UInt6,
  pub PMU: RmiFeature,
  pub PMU_NUM_CTRS: UInt5,
  pub HASH_SHA_256: RmiFeature,
  pub HASH_SHA_512: RmiFeature,
  pub GICV3_NUM_LRS: UInt4,
  pub DA: RmiFeature,
  pub RTT_PLANE: RmiRttPlaneFeature,
}

struct RmiFeatureRegister1 {
  pub MAX_MECID: UInt64,
}

struct RmiInterfaceVersion {
  pub minor: UInt16,
  pub major: UInt15,
}

struct RmiPdevFlags {
  pub prot: RmiPdevProtection,
}

struct RmiPdevParams {
  pub aux: [Address; 32],
  pub coh_addr_range: [RmiAddressRange; 4],
  pub coh_num_addr_range: UInt64,
  pub ncoh_num_addr_range: UInt64,
  pub ncoh_addr_range: [RmiAddressRange; 16],
  pub root_id: UInt16,
  pub ide_sid: UInt64,
  pub pdev_id: UInt64,
  pub flags: RmiPdevFlags,
  pub hash_algo: RmiHashAlgorithm,
  pub rid_base: UInt16,
  pub rid_top: UInt16,
  pub cert_id: UInt64,
  pub ecam_addr: Address,
  pub segment_id: UInt8,
  pub num_aux: UInt64,
}

struct RmiPublicKeyParams {
  pub key_len: UInt64,
  pub metadata_len: UInt64,
  pub algo: RmiSignatureAlgorithm,
  pub key: [UInt8; 1024],
  pub metadata: [UInt8; 1024],
}

struct RmiRealmFlags0 {
  pub lpa2: RmiFeature,
  pub sve: RmiFeature,
  pub pmu: RmiFeature,
  pub da: RmiFeature,
  pub lfa_policy: RmiLfaPolicy,
}

struct RmiRealmFlags1 {
  pub rtt_tree_per_plane: RmiFeature,
  pub rtt_s2ap_encoding: RmiRttS2APEncoding,
}

struct RmiRealmParams {
  pub flags0: RmiRealmFlags0,
  pub s2sz: UInt8,
  pub sve_vl: UInt8,
  pub num_bps: UInt8,
  pub num_wps: UInt8,
  pub pmu_num_ctrs: UInt8,
  pub hash_algo: RmiHashAlgorithm,
  pub num_aux_planes: UInt64,
  pub rpv: [UInt64; 8],
  pub vmid: UInt16,
  pub rtt_base: Address,
  pub rtt_level_start: Int64,
  pub rtt_num_start: UInt32,
  pub flags1: RmiRealmFlags1,
  pub mecid: UInt64,
  pub aux_vmid: [UInt16; 3],
  pub aux_rtt_base: [Address; 3],
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
  pub s2ap_response: RmiResponse,
  pub dev_mem_response: RmiResponse,
}

struct RmiRecExit {
  pub exit_reason: RmiRecExitReason,
  pub esr: UInt64,
  pub far: UInt64,
  pub hpfar: UInt64,
  pub rtt_tree: UInt64,
  pub rtt_level: Int64,
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
  pub s2ap_base: UInt64,
  pub s2ap_top: UInt64,
  pub vdev_id: UInt64,
  pub imm: UInt16,
  pub plane: UInt64,
  pub vdev: Address,
  pub vdev_action: RmiVdevAction,
  pub dev_mem_base: UInt64,
  pub dev_mem_top: UInt64,
  pub dev_mem_pa: Address,
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

struct RmiVdevFlags {
  pub VSMMU: RmiFeature,
}

struct RmiVdevParams {
  pub flags: RmiVdevFlags,
  pub vdev_id: UInt64,
  pub tdi_id: UInt64,
  pub num_aux: UInt64,
  pub vsmmu_addr: Address,
  pub vsid: UInt64,
  pub aux: [Address; 32],
}

struct RmiVsmmuFlags {
}

struct RmiVsmmuParams {
  pub flags: RmiVsmmuFlags,
  pub reg_base: Address,
  pub reg_top: Address,
  pub aidr: UInt64,
  pub idr: [UInt64; 7],
}

struct RsiDeviceInfo {
  pub attest_type: RsiDeviceAttestationType,
  pub cert_id: UInt64,
  pub hash_algo: RsiHashAlgorithm,
  pub cert_digest: [UInt64; 8],
  pub meas_digest: [UInt64; 8],
  pub report_digest: [UInt64; 8],
}

struct RsiDevMemFlags {
  pub coh: RsiDevMemCoherent,
  pub order: RsiDevMemOrdering,
}

struct RsiFeatureRegister0 {
  pub DA: RsiFeature,
  pub MRO: RsiFeature,
}

struct RsiHostCall {
  pub imm: UInt16,
  pub gprs: [UInt64; 31],
}

struct RsiInterfaceVersion {
  pub minor: UInt16,
  pub major: UInt15,
}

struct RsiPlaneEnter {
  pub flags: RsiPlaneEnterFlags,
  pub pc: UInt64,
  pub gicv3_hcr: UInt64,
  pub gprs: [UInt64; 31],
  pub gicv3_lrs: [UInt64; 16],
}

struct RsiPlaneEnterFlags {
  pub trap_wfi: RsiTrap,
  pub trap_wfe: RsiTrap,
  pub trap_hc: RsiTrap,
  pub gic_owner: RsiGicOwner,
}

struct RsiPlaneExit {
  pub reason: RsiPlaneExitReason,
  pub elr_el2: UInt64,
  pub esr_el2: UInt64,
  pub far_el2: UInt64,
  pub hpfar_el2: UInt64,
  pub gicv3_hcr: UInt64,
  pub gicv3_misr: UInt64,
  pub gicv3_vmcr: UInt64,
  pub cntp_ctl: UInt64,
  pub cntp_cval: UInt64,
  pub cntv_ctl: UInt64,
  pub cntv_cval: UInt64,
  pub gprs: [UInt64; 31],
  pub gicv3_lrs: [UInt64; 16],
}

struct RsiPlaneRun {
  pub enter: RsiPlaneEnter,
  pub exit: RsiPlaneExit,
}

struct RsiRealmConfig {
  pub ipa_width: UInt64,
  pub hash_algo: RsiHashAlgorithm,
  pub num_aux_planes: UInt64,
  pub gicv3_vtr: UInt64,
  pub rpv: [UInt64; 8],
}

struct RsiRipasChangeFlags {
  pub destroyed: RsiRipasChangeDestroyed,
}

struct PsciInterfaceVersion {
  pub minor: UInt16,
  pub major: UInt15,
}

struct RmmAddressRange {
  pub base: Address,
  pub top: Address,
}

struct RmmDataFlags {
  pub measure: RmmDataMeasureContent,
}

struct RmmDevMemFlags {
  pub coh: RmmDevMemCoherent,
  pub order: RmmDevMemOrdering,
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
  pub feat_da: RmmFeature,
  pub rtt_plane: RmmRttPlaneFeature,
  pub rtt_s2ap_indirect: RmmFeature,
  pub max_mecid: UInt64,
  pub max_recs_order: UInt64,
  pub gicv3_num_lrs: UInt64,
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
  pub content: RmmRealmMeasurement,
}

struct RmmMeasurementDescriptorRipas {
  pub desc_type: UInt8,
  pub len: UInt64,
  pub rim: RmmRealmMeasurement,
  pub base: Address,
  pub top: Address,
}

struct RmmMemPerms {
  pub values: [UInt64; 16],
}

struct RmmPdev {
  pub pdev_id: UInt64,
  pub prot: RmmPdevProtection,
  pub segment_id: UInt8,
  pub ecam_addr: Address,
  pub root_id: UInt16,
  pub cert_id: UInt64,
  pub rid_base: UInt16,
  pub rid_top: UInt16,
  pub hash_algo: RmmHashAlgorithm,
  pub ide_sid: UInt64,
  pub ncoh_addr_range: [RmmAddressRange; 16],
  pub coh_addr_range: [RmmAddressRange; 4],
  pub aux: [Address; 32],
  pub num_aux: UInt64,
  pub state: RmmPdevState,
  pub comm_state: RmmDevCommState,
  pub num_vdevs: UInt64,
  pub coh_num_addr_range: UInt64,
  pub ncoh_num_addr_range: UInt64,
}

struct RmmRdev {
  pub state: RmmRdevState,
  pub operation: RmmRdevOperation,
  pub vdev_ptr: Address,
}

struct RmmRealm {
  pub feat_lpa2: RmmFeature,
  pub ipa_width: UInt8,
  pub measurements: [RmmRealmMeasurement; 5],
  pub hash_algo: RmmHashAlgorithm,
  pub rec_index: UInt64,
  pub rtt_base: [Address; 4],
  pub rtt_level_start: Int64,
  pub rtt_num_start: UInt64,
  pub state: RmmRealmState,
  pub vmid: [UInt16; 4],
  pub rpv: [UInt64; 8],
  pub feat_da: RmmFeature,
  pub rtt_tree_per_plane: RmmFeature,
  pub num_aux_planes: UInt64,
  pub rtt_s2ap_encoding: RmmRttS2APEncoding,
  pub overlay_perms: [RmmMemPerms; 4],
  pub overlay_locked: [RmmMemPermLocked; 16],
  pub lfa_policy: RmmLfaPolicy,
  pub mecid: UInt64,
  pub mec_policy: RmmMecPolicy,
  pub num_recs: UInt64,
  pub num_vdevs: UInt64,
  pub vdev_count: UInt64,
}

struct RmmRec {
  pub owner: Address,
  pub aux: [Address; 16],
  pub flags: RmmRecFlags,
  pub mpidr: UInt64,
  pub gic_owner: UInt64,
  pub state: RmmRecState,
  pub pending: RmmRecPending,
  pub emulatable_abort: RmmRecEmulatableAbort,
  pub gprs: [UInt64; 32],
  pub pc: UInt64,
  pub sysregs: RmmSystemRegisters,
  pub attest_state: RmmRecAttestState,
  pub attest_challenge: [UInt64; 8],
  pub ripas_addr: Address,
  pub ripas_top: Address,
  pub ripas_value: RmmRipas,
  pub ripas_destroyed: RmmRipasChangeDestroyed,
  pub ripas_response: RmmRecResponse,
  pub dev_mem_addr: Address,
  pub dev_mem_top: Address,
  pub dev_mem_pa: Address,
  pub dev_mem_flags: RmmDevMemFlags,
  pub dev_mem_response: RmmRecResponse,
  pub s2ap_addr: Address,
  pub s2ap_top: Address,
  pub s2ap_overlay_index: UInt4,
  pub s2ap_response: RmmRecResponse,
  pub vdev_id: UInt64,
  pub inst_id: UInt64,
  pub inst_id_valid: RmmBoolean,
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
  pub attr_prot: RmmRttMemAttr,
  pub attr_unprot: UInt3,
  pub sh: RmmRttShareability,
  pub s2ap_direct: RmmRttS2APDirect,
  pub s2ap_indirect: RmmRttS2APIndirect,
}

struct RmmRttS2APDirect {
  pub read: RmmBoolean,
  pub write: RmmBoolean,
}

struct RmmRttS2APIndirect {
  pub base_index: UInt4,
  pub overlay_index: UInt4,
}

struct RmmRttWalkNotAligned {
  pub valid: RmmBoolean,
  pub index: UInt64,
  pub addr: Address,
  pub walk: RmmRttWalkResult,
}

struct RmmRttWalkResult {
  pub level: int,
  pub rtt_addr: Address,
  pub rtte: RmmRttEntry,
}

struct RmmVdev {
  pub vdev_id: UInt64,
  pub tdi_id: UInt64,
  pub inst_id: UInt64,
  pub pdev: Address,
  pub realm: Address,
  pub state: RmmVdevState,
  pub comm_state: RmmDevCommState,
  pub aux: [Address; 32],
  pub num_aux: UInt64,
  pub vsmmu: RmmFeature,
  pub vsmmu_addr: Address,
  pub vsid: UInt64,
}

struct RmmVsmmu {
  pub realm: Address,
  pub reg_base: Address,
  pub reg_top: Address,
  pub aidr: UInt64,
  pub idr: [UInt64; 7],
}

pub open spec fn AddrInRange(s: S, addr: Address, base: Address, size: int) -> bool;

pub open spec fn AddrIsAligned(s: S, addr: Address, n: int) -> bool;

pub open spec fn AddrIsAuxLive(s: S, addr: Address, realm: RmmRealm) -> bool;

pub open spec fn AddrIsGranuleAligned(s: S, addr: Address) -> bool;

pub open spec fn AddrIsProtected(s: S, addr: Address, realm: RmmRealm) -> bool;

pub open spec fn AddrIsRttLevelAligned(s: S, addr: Address, level: int) -> bool;

pub open spec fn AddrIsWithin(s: S, addr: Address, base: Address, top: Address) -> bool;

pub open spec fn AddrRangeIsAuxLive(s: S, base: Address, top: Address, realm: RmmRealm) -> bool;

pub open spec fn AddrRangeIsProtected(s: S, base: Address, top: Address, realm: RmmRealm) -> bool;

pub open spec fn AddrRangeIsWithin(s: S, inner_base: Address, inner_top: Address, outer_base: Address, outer_top: Address) -> bool;

pub open spec fn AlignDownToRttLevel(s: S, addr: Address, level: int) -> Address;

pub open spec fn AlignUpToRttLevel(s: S, addr: Address, level: int) -> Address;

pub open spec fn AuxAlias16(s: S, obj: Address, aux: [Address; 16], count: int) -> bool;

pub open spec fn AuxAlias32(s: S, obj: Address, aux: [Address; 32], count: int) -> bool;

pub open spec fn AuxAligned16(s: S, aux: [Address; 16], count: int) -> bool;

pub open spec fn AuxAligned32(s: S, aux: [Address; 32], count: int) -> bool;

pub open spec fn AuxEqual16(s: S, aux1: [Address; 16], aux2: [Address; 16], count: int) -> bool;

pub open spec fn AuxEqual32(s: S, aux1: [Address; 32], aux2: [Address; 32], count: int) -> bool;

pub open spec fn AuxSort(s: S, addrs: [Address; 16], count: int) -> [Address; 16];

pub open spec fn AuxStateEqual16(s: S, aux: [Address; 16], count: int, state: RmmGranuleState) -> bool;

pub open spec fn AuxStateEqual32(s: S, aux: [Address; 32], count: int, state: RmmGranuleState) -> bool;

pub open spec fn AuxStates(s: S, aux: [Address; 16], count: int);

pub open spec fn CurrentRealm(s: S) -> RmmRealm;

pub open spec fn CurrentRec(s: S) -> RmmRec;

pub open spec fn DeviceCommunicate(s: S, pdev: RmmPdev, data: RmiDevCommData) -> RmmDevCommState;

pub open spec fn Equal(abstract_: RmmFeature, concrete: RmiFeature) -> bool;

pub open spec fn FeatureToRmi(s: S, value: RmmFeature) -> RmiFeature;

pub open spec fn FeatureToRsi(s: S, value: RmmFeature) -> RsiFeature;

pub open spec fn Gicv3ConfigIsValid(s: S, gicv3_hcr: u64, gicv3_lrs: [u64; 16]) -> bool;

pub open spec fn GranuleAccessPermitted(s: S, addr: Address, pas: RmmPhysicalAddressSpace) -> bool;

pub open spec fn GranuleAt(s: S, addr: Address) -> RmmGranule;

pub open spec fn ImplFeatures(s: S) -> RmmFeatures;

pub open spec fn MecMembers(s: S, mecid: u64) -> int;

pub open spec fn MecPolicy(s: S, mecid: u64) -> RmmMecPolicy;

pub open spec fn MecState(s: S, mecid: u64) -> RmmMecState;

pub open spec fn MemPermLabelSupported(s: S, label: u64) -> bool;

pub open spec fn MinAddress(s: S, addr1: Address, addr2: Address) -> Address;

pub open spec fn MpidrEqual(rmm_mpidr: u64, rmi_mpidr: RmiRecMpidr) -> bool;

pub open spec fn MpidrIsUsed(s: S, mpidr: u64) -> bool;

pub open spec fn MsiAddrIsValid(s: S, addr: Address) -> bool;

pub open spec fn PaIsDelegable(s: S, addr: Address) -> bool;

pub open spec fn PaIsDelegableCohDevMem(s: S, addr: Address) -> bool;

pub open spec fn PaIsDelegableDevMem(s: S, addr: Address) -> bool;

pub open spec fn PaIsDelegableDram(s: S, addr: Address) -> bool;

pub open spec fn PaIsDelegableNonCohDevMem(s: S, addr: Address) -> bool;

pub open spec fn PdevAt(s: S, addr: Address) -> RmmPdev;

pub open spec fn PdevAuxCount(s: S, flags: RmiPdevFlags) -> int;

pub open spec fn PdevFlags(s: S, pdev: RmmPdev) -> RmiPdevFlags;

pub open spec fn PdevVsmmuIsCompatible(s: S, pdev: RmmPdev, vsmmu: RmmVsmmu) -> bool;

pub open spec fn PlaneRegIsValid(s: S, realm: RmmRealm, encoding: u64) -> bool;

pub open spec fn PlaneRegValue(s: S, realm: RmmRealm, plane_idx: int, encoding: u64) -> u64;

pub open spec fn PsciReturnCodeEncode(s: S, value: PsciReturnCode) -> u64;

pub open spec fn PsciReturnCodePermitted(s: S, calling_rec: RmmRec, target_rec: RmmRec, value: PsciReturnCode) -> bool;

pub open spec fn PsmmuAddrIsValid(s: S, addr: Address) -> bool;

pub open spec fn PsmmuSupportsMsi(s: S, addr: Address) -> bool;

pub open spec fn RdevFromInstId(s: S, realm: RmmRealm, inst_id: int) -> RmmRdev;

pub open spec fn RdevFromVdevId(s: S, realm: RmmRealm, vdev_id: u64) -> RmmRdev;

pub open spec fn RdevIdIsValid(s: S, realm: RmmRealm, vdev_id: u64) -> bool;

pub open spec fn RdevIdsAreValid(s: S, realm: RmmRealm, vdev_id: u64, inst_id: int) -> bool;

pub open spec fn ReadMemory(s: S, addr: u64, offset: int, size: int) -> [u8; 1];

pub open spec fn RealmAt(s: S, addr: Address) -> RmmRealm;

pub open spec fn RealmIsLive(s: S, addr: Address) -> bool;

pub open spec fn RealmParamsSupported(s: S, params: RmiRealmParams) -> bool;

pub open spec fn RealmRttBaseEqual(realm: RmmRealm, rtt_base: Address, aux_rtt_base: [Address; 3]) -> bool;

pub open spec fn RealmVmidEqual(realm: RmmRealm, vmid: u16, aux_vmid: [u16; 3]) -> bool;

pub open spec fn RecAt(s: S, addr: Address) -> RmmRec;

pub open spec fn RecAuxCount(s: S, rd: Address) -> int;

pub open spec fn RecDevMemResponseToRsi(s: S, rec: RmmRec) -> RsiResponse;

pub open spec fn RecFromMpidr(s: S, mpidr: u64) -> RmmRec;

pub open spec fn RecIndex(s: S, mpidr: RmiRecMpidr) -> int;

pub open spec fn RecRipasResponseToRsi(s: S, rec: RmmRec) -> RsiResponse;

pub open spec fn RecS2APResponseToRsi(s: S, rec: RmmRec) -> RsiResponse;

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

pub open spec fn RmiAddressRangesEqual16(s: S, ranges1: [RmmAddressRange; 16], ranges2: [RmiAddressRange; 16], count: int) -> bool;

pub open spec fn RmiAddressRangesEqual4(s: S, ranges1: [RmmAddressRange; 4], ranges2: [RmiAddressRange; 4], count: int) -> bool;

pub open spec fn RmiDevCommDataAt(s: S, addr: Address) -> RmiDevCommData;

pub open spec fn RmiFeatureRegister0Decode(s: S, value: u64) -> RmiFeatureRegister0;

pub open spec fn RmiFeatureRegisterEncode(s: S, index: int) -> u64;

pub open spec fn RmiPdevEventIsValid(s: S, ev: RmiPdevEvent) -> bool;

pub open spec fn RmiPdevFlagsDecode(s: S, value: u64) -> RmiPdevFlags;

pub open spec fn RmiPdevParamsAt(s: S, addr: Address) -> RmiPdevParams;

pub open spec fn RmiPdevParamsIsValid(s: S, addr: Address) -> bool;

pub open spec fn RmiPublicKeyParamsAt(s: S, addr: Address) -> RmiPublicKeyParams;

pub open spec fn RmiRealmParamsAt(s: S, addr: Address) -> RmiRealmParams;

pub open spec fn RmiRealmParamsIsValid(s: S, addr: Address) -> bool;

pub open spec fn RmiRecParamsAt(s: S, addr: Address) -> RmiRecParams;

pub open spec fn RmiRecRunAt(s: S, addr: Address) -> RmiRecRun;

pub open spec fn RmiVdevFlagsDecode(s: S, value: u64) -> RmiVdevFlags;

pub open spec fn RmiVdevParamsAt(s: S, addr: Address) -> RmiVdevParams;

pub open spec fn RmiVdevParamsIsValid(s: S, addr: Address) -> bool;

pub open spec fn RmiVersionHigherIsSupported(s: S, version: RmiInterfaceVersion) -> bool;

pub open spec fn RmiVersionHighest(s: S) -> RmiInterfaceVersion;

pub open spec fn RmiVersionHighestBelow(s: S, version: RmiInterfaceVersion) -> RmiInterfaceVersion;

pub open spec fn RmiVersionIsSupported(s: S, version: RmiInterfaceVersion) -> bool;

pub open spec fn RmiVersionLowerIsSupported(s: S, version: RmiInterfaceVersion) -> bool;

pub open spec fn RmiVsmmuParamsAt(s: S, addr: Address) -> RmiVsmmuParams;

pub open spec fn RmiVsmmuParamsIsValid(s: S, addr: Address) -> bool;

pub open spec fn RsiDeviceInfoAt(s: S, addr: Address) -> RsiDeviceInfo;

pub open spec fn RsiFeatureRegisterEncode(s: S, realm: RmmRealm, index: int) -> u64;

pub open spec fn RsiHostCallAt(s: S, addr: Address) -> RsiHostCall;

pub open spec fn RsiPlaneRunAt(s: S, realm: RmmRealm, addr: Address) -> RsiPlaneRun;

pub open spec fn RsiRealmConfigAt(s: S, addr: Address) -> RsiRealmConfig;

pub open spec fn RsiVersionHigherIsSupported(s: S, version: RsiInterfaceVersion) -> bool;

pub open spec fn RsiVersionHighest(s: S) -> RsiInterfaceVersion;

pub open spec fn RsiVersionHighestBelow(s: S, version: RsiInterfaceVersion) -> RsiInterfaceVersion;

pub open spec fn RsiVersionIsSupported(s: S, version: RsiInterfaceVersion) -> bool;

pub open spec fn RsiVersionLowerIsSupported(s: S, version: RsiInterfaceVersion) -> bool;

pub open spec fn RttAllEntriesContiguous(s: S, rtt: RmmRtt, addr: Address, level: int) -> bool;

pub open spec fn RttAllEntriesRipas(s: S, rtt: RmmRtt, ripas: RmmRipas) -> bool;

pub open spec fn RttAllEntriesState(s: S, rtt: RmmRtt, state: RmmRttEntryState) -> bool;

pub open spec fn RttAt(s: S, addr: Address) -> RmmRtt;

pub open spec fn RttConfigIsValid(s: S, ipa_width: int, rtt_level_start: int, rtt_num_start: int) -> bool;

pub open spec fn RttDescriptorDecode(s: S, desc: u64, encoding: RmmRttS2APEncoding) -> RmmRttEntry;

pub open spec fn RttDescriptorIsValidForUnprotected(s: S, desc: u64) -> bool;

pub open spec fn RttEntriesInRangeCohDevMem(s: S, rtt: RmmRtt, level: int, base: Address, top: Address) -> bool;

pub open spec fn RttEntriesInRangeMemAttr(s: S, rtt: RmmRtt, level: int, base: Address, top: Address, attr: RmmRttMemAttr) -> bool;

pub open spec fn RttEntriesInRangeNonCohDevMem(s: S, rtt: RmmRtt, level: int, base: Address, top: Address) -> bool;

pub open spec fn RttEntriesInRangeOutputContiguous(s: S, rtt: RmmRtt, level: int, base: Address, top: Address, out: Address) -> bool;

pub open spec fn RttEntriesInRangeRipas(s: S, rtt: RmmRtt, level: int, base: Address, top: Address, ripas: RmmRipas) -> bool;

pub open spec fn RttEntryAt(s: S, rtt: RmmRtt, i: int) -> RmmRttEntry;

pub open spec fn RttEntryIndex(s: S, addr: Address, level: int) -> int;

pub open spec fn RttEntryStateToRmi(s: S, state: RmmRttEntryState) -> RmiRttEntryState;

pub open spec fn RttFold(s: S, rtt: RmmRtt) -> RmmRttEntry;

pub open spec fn RttIsHomogeneous(s: S, rtt: RmmRtt) -> bool;

pub open spec fn RttIsLive(s: S, rtt: RmmRtt) -> bool;

pub open spec fn RttLevelIsBlockOrPage(s: S, realm: RmmRealm, level: int) -> bool;

pub open spec fn RttLevelIsStarting(s: S, realm: RmmRealm, level: int) -> bool;

pub open spec fn RttLevelIsValid(s: S, realm: RmmRealm, level: int) -> bool;

pub open spec fn RttLevelSize(s: S, level: int) -> int;

pub open spec fn RttMemAttrEqual(rtte1: RmmRttEntry, rtte2: RmmRttEntry, prot: RmmRttProtected) -> bool;

pub open spec fn RttS2APEqual(rtte1: RmmRttEntry, rtte2: RmmRttEntry, encoding: RmmRttS2APEncoding) -> bool;

pub open spec fn RttsAllProtectedEntriesRipas(s: S, rtt_base: Address, rtt_num_start: int, ripas: RmmRipas) -> bool;

pub open spec fn RttsAllProtectedEntriesState(s: S, rtt_base: Address, rtt_num_start: int, state: RmmRttEntryState) -> bool;

pub open spec fn RttsAllUnprotectedEntriesState(s: S, rtt_base: Address, rtt_num_start: int, state: RmmRttEntryState) -> bool;

pub open spec fn RttsGranuleState(s: S, rtt_base: Address, rtt_num_start: int);

pub open spec fn RttSkipEntriesIfNotState(s: S, rtt: RmmRtt, level: int, base: Address, top: Address, state: RmmRttEntryState) -> Address;

pub open spec fn RttSkipEntriesUnlessRipas(s: S, rtt: RmmRtt, level: int, ipa: Address, ripas: RmmRipas) -> Address;

pub open spec fn RttSkipEntriesUnlessState(s: S, rtt: RmmRtt, level: int, ipa: Address, state: RmmRttEntryState) -> Address;

pub open spec fn RttSkipEntriesWithRipas(s: S, rtt: RmmRtt, level: int, base: Address, top: Address, stop_at_destroyed: bool) -> Address;

pub open spec fn RttSkipNonLiveEntries(s: S, rtt: RmmRtt, level: int, ipa: Address) -> Address;

pub open spec fn RttsStateEqual(rtt_base: Address, rtt_num_start: int, state: RmmGranuleState) -> bool;

pub open spec fn RttWalk(s: S, realm: RmmRealm, addr: Address, level: int, index: int) -> RmmRttWalkResult;

pub open spec fn RttWalkAnyNotAligned(s: S, realm: RmmRealm, base: Address, top: Address, level: int) -> RmmRttWalkNotAligned;

pub open spec fn TdiIdIsFree(s: S, tdi_id: u64, segment_id: u8) -> bool;

pub open spec fn ToAddress(value: int) -> Address;

pub open spec fn ToBits64(value: int) -> u64;

pub open spec fn VdevAt(s: S, addr: Address) -> RmmVdev;

pub open spec fn VdevAuxCount(s: S, pdev_flags: RmiPdevFlags, vdev_flags: RmiVdevFlags) -> int;

pub open spec fn VersionEqual(ver1: RmiInterfaceVersion, ver2: RmiInterfaceVersion) -> bool;

pub open spec fn VmidsAreFree(s: S, vmid: [u16; 4]) -> bool;

pub open spec fn VmidsAreValid(s: S, vmid: u16, aux_vmid: [u16; 3]) -> bool;

pub open spec fn VsidIsFree(s: S, vsmmu: RmmVsmmu, vsid: u64) -> bool;

pub open spec fn VsmmuAt(s: S, addr: Address) -> RmmVsmmu;

pub open spec fn VsmmuIsLive(s: S, addr: Address) -> bool;

pub open spec fn VmidsAreFree1(s: S, vmid: u16, aux_vmid: [u16;3]) -> bool;

pub open spec fn DeviceCommunicate1(s: S, vdev: RmmRdev) -> RmmDevCommState;

pub open spec fn DeviceCommunicate2(s: S, vdev: RmmVdev, data: RmiDevCommData) -> RmmDevCommState;

pub open spec fn rmi_data_create_spec(rd: Address, data: Address, ipa: Address, src: Address, flags: RmiDataFlags, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, src) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, src, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDram(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((data) >= 2^48)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() ==> GranuleAt(new_s, data).state == DATA)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RAM)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == data)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot == MEMATTR_CACHEABLE)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh == SHAREABILITY_INNER)
  && (result.is_Ok() ==> RealmAt(new_s, rd).measurements[0] == RimExtendData(new_s, RealmAt(new_s, rd), ipa, data, flags))
  && ((AddrIsGranuleAligned(old_s, src) &&
       GranuleAccessPermitted(old_s, src, PAS_NS) &&
       AddrIsGranuleAligned(old_s, data) &&
       PaIsDelegableDram(old_s, data) &&
       !(GranuleAt(old_s, data).state != DELEGATED) &&
       !(((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((data) >= 2^48))) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, data).state == GranuleAt(old_s, data).state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).measurements[0] == RealmAt(old_s, rd).measurements[0])
}

pub open spec fn rmi_data_create_unknown_spec(rd: Address, data: Address, ipa: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDram(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((data) >= 2^48)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() ==> GranuleAt(new_s, data).state == DATA)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == data)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot == MEMATTR_CACHEABLE)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh == SHAREABILITY_INNER)
  && ((AddrIsGranuleAligned(old_s, data) &&
       PaIsDelegableDram(old_s, data) &&
       !(GranuleAt(old_s, data).state != DELEGATED) &&
       !(((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((data) >= 2^48))) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, data).state == GranuleAt(old_s, data).state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
}

pub open spec fn rmi_data_destroy_spec(rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, data: Address, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))))
  && (AddrIsAuxLive(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT_AUX(0 as int)))
  && (result.is_Ok() ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == DELEGATED)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED)
  && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RAM ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == DESTROYED)
  && (result.is_Ok() ==> data == RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED) &&
       !(AddrIsAuxLive(old_s, ipa, RealmAt(old_s, rd))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == GranuleAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
  && (!(result.is_Ok() && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RAM)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
}

pub open spec fn rmi_dev_mem_map_spec(rd: Address, ipa: Address, level: Int64, addr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDevMem(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, addr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttLevelIsBlockOrPage(old_s, RealmAt(old_s, rd), level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).level < level ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() ==> GranuleAt(new_s, addr).state == DEV_MAPPED)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_DEV)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == addr)
  && (result.is_Ok() && PaIsDelegableNonCohDevMem(old_s, addr) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot == MEMATTR_NON_CACHEABLE)
  && (result.is_Ok() && PaIsDelegableCohDevMem(old_s, addr) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot == MEMATTR_PASSTHROUGH)
  && (result.is_Ok() && PaIsDelegableNonCohDevMem(old_s, addr) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh == SHAREABILITY_OUTER)
  && (result.is_Ok() && PaIsDelegableCohDevMem(old_s, addr) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh == SHAREABILITY_INNER)
  && ((AddrIsGranuleAligned(old_s, addr) &&
       PaIsDelegableDevMem(old_s, addr) &&
       !(GranuleAt(old_s, addr).state != DELEGATED) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       RttLevelIsBlockOrPage(old_s, RealmAt(old_s, rd), level as int) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).level < level) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, addr).state == GranuleAt(old_s, addr).state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot == RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot == RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh == RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh == RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
}

pub open spec fn rmi_dev_mem_unmap_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, pa: Address, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttLevelIsBlockOrPage(old_s, RealmAt(old_s, rd), level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level < level ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_DEV ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))))
  && (result.is_Ok() ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == DELEGATED)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED)
  && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == DEV ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == DESTROYED)
  && (result.is_Ok() ==> pa == RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       RttLevelIsBlockOrPage(old_s, RealmAt(old_s, rd), level as int) &&
       AddrIsRttLevelAligned(old_s, ipa, level as int) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level < level) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_DEV))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == GranuleAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
  && (!(result.is_Ok() && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == DEV)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
}

pub open spec fn rmi_features_spec(index: UInt64, result: Result<(), RmiStatusCode>, value: Bits64, old_s: S, new_s: S) -> bool {
  (result.is_Ok() ==> value == RmiFeatureRegisterEncode(new_s, index as int))
}

pub open spec fn rmi_granule_delegate_spec(addr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, addr).state != UNDELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> GranuleAt(new_s, addr).state == DELEGATED)
  && (result.is_Ok() ==> GranuleAt(new_s, addr).gpt == GPT_REALM)
  && ((AddrIsGranuleAligned(old_s, addr) &&
       PaIsDelegable(old_s, addr) &&
       !(GranuleAt(old_s, addr).state != UNDELEGATED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, addr).state == GranuleAt(old_s, addr).state)
  && (result.is_Err()
    ==> GranuleAt(new_s, addr).gpt == GranuleAt(old_s, addr).gpt)
}

pub open spec fn rmi_granule_undelegate_spec(addr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, addr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> GranuleAt(new_s, addr).gpt != GPT_REALM)
  && (result.is_Ok() ==> GranuleAt(new_s, addr).state == UNDELEGATED)
  && ((AddrIsGranuleAligned(old_s, addr) &&
       PaIsDelegable(old_s, addr) &&
       !(GranuleAt(old_s, addr).state != DELEGATED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, addr).state == GranuleAt(old_s, addr).state)
}

pub open spec fn rmi_mec_set_private_spec(mecid: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((mecid) > (ImplFeatures(old_s).max_mecid) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (MecState(old_s, mecid) != MEC_STATE_SHARED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (MecMembers(old_s, mecid) != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> MecState(new_s, mecid) == MEC_STATE_PRIVATE_UNASSIGNED)
  && ((!((mecid) > (ImplFeatures(old_s).max_mecid)) &&
       !(MecState(old_s, mecid) != MEC_STATE_SHARED) &&
       !(MecMembers(old_s, mecid) != 0))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> MecState(new_s, mecid) == MecState(old_s, mecid))
}

pub open spec fn rmi_mec_set_shared_spec(mecid: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((mecid) > (ImplFeatures(old_s).max_mecid) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (MecState(old_s, mecid) != MEC_STATE_PRIVATE_UNASSIGNED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> MecState(new_s, mecid) == MEC_STATE_SHARED)
  && ((!((mecid) > (ImplFeatures(old_s).max_mecid)) &&
       !(MecState(old_s, mecid) != MEC_STATE_PRIVATE_UNASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> MecState(new_s, mecid) == MecState(old_s, mecid))
}

pub open spec fn rmi_pdev_abort_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PdevAt(old_s, pdev_ptr).state != PDEV_NEW && PdevAt(old_s, pdev_ptr).state != PDEV_HAS_KEY && PdevAt(old_s, pdev_ptr).state != PDEV_COMMUNICATING) ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() && PdevAt(old_s, pdev_ptr).state == PDEV_COMMUNICATING ==> (PdevAt(new_s, pdev_ptr).state == PDEV_READY && PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_IDLE))
  && (result.is_Ok() && PdevAt(old_s, pdev_ptr).state != PDEV_COMMUNICATING ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       !((PdevAt(old_s, pdev_ptr).state != PDEV_NEW && PdevAt(old_s, pdev_ptr).state != PDEV_HAS_KEY && PdevAt(old_s, pdev_ptr).state != PDEV_COMMUNICATING)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
}

pub open spec fn rmi_pdev_aux_count_spec(flags: Bits64, result: Result<(), RmiStatusCode>, aux_count: UInt64, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (result.is_Ok() ==> aux_count == PdevAuxCount(new_s, RmiPdevFlagsDecode(new_s, flags)))
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE))
    ==> result.is_Ok())
}

pub open spec fn rmi_pdev_communicate_spec(pdev_ptr: Address, data_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, data_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, data_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.req_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.req_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.resp_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.resp_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RmiDevCommDataAt(old_s, data_ptr).enter.resp_len > RMM_GRANULE_SIZE ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PdevAt(old_s, pdev_ptr).comm_state == DEV_COMM_IDLE || PdevAt(old_s, pdev_ptr).comm_state == DEV_COMM_ERROR) ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DeviceCommunicate(new_s, PdevAt(new_s, pdev_ptr), RmiDevCommDataAt(new_s, data_ptr)))
  && (result.is_Ok() && (DeviceCommunicate(old_s, PdevAt(old_s, pdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_ERROR && PdevAt(old_s, pdev_ptr).state != PDEV_STOPPING) ==> PdevAt(new_s, pdev_ptr).state == PDEV_ERROR)
  && (result.is_Ok() && (DeviceCommunicate(old_s, PdevAt(old_s, pdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_IDLE && PdevAt(old_s, pdev_ptr).state == PDEV_NEW) ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEEDS_KEY)
  && (result.is_Ok() && (DeviceCommunicate(old_s, PdevAt(old_s, pdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_IDLE && PdevAt(old_s, pdev_ptr).state == PDEV_HAS_KEY) ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY)
  && (result.is_Ok() && (DeviceCommunicate(old_s, PdevAt(old_s, pdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_IDLE && PdevAt(old_s, pdev_ptr).state == PDEV_READY) ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY)
  && (result.is_Ok() && (DeviceCommunicate(old_s, PdevAt(old_s, pdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) != DEV_COMM_ACTIVE && PdevAt(old_s, pdev_ptr).state == PDEV_STOPPING) ==> PdevAt(new_s, pdev_ptr).state == PDEV_STOPPED)
  && (result.is_Ok() && (DeviceCommunicate(old_s, PdevAt(old_s, pdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_IDLE && PdevAt(old_s, pdev_ptr).state == PDEV_COMMUNICATING) ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY)
  && (result.is_Ok() && (DeviceCommunicate(old_s, PdevAt(old_s, pdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_IDLE && PdevAt(old_s, pdev_ptr).state == PDEV_IDE_RESETTING) ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       AddrIsGranuleAligned(old_s, data_ptr) &&
       GranuleAccessPermitted(old_s, data_ptr, PAS_NS) &&
       AddrIsGranuleAligned(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.req_addr) &&
       GranuleAccessPermitted(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.req_addr, PAS_NS) &&
       AddrIsGranuleAligned(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.resp_addr) &&
       GranuleAccessPermitted(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.resp_addr, PAS_NS) &&
       !(RmiDevCommDataAt(old_s, data_ptr).enter.resp_len > RMM_GRANULE_SIZE) &&
       !((PdevAt(old_s, pdev_ptr).comm_state == DEV_COMM_IDLE || PdevAt(old_s, pdev_ptr).comm_state == DEV_COMM_ERROR)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
}

pub open spec fn rmi_pdev_create_spec(pdev_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDram(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RmiPdevParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RmiPdevParamsAt(old_s, params_ptr).num_aux != PdevAuxCount(old_s, RmiPdevParamsAt(old_s, params_ptr).flags) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AuxAligned32(old_s, RmiPdevParamsAt(old_s, params_ptr).aux, RmiPdevParamsAt(old_s, params_ptr).num_aux as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AuxAlias32(old_s, pdev_ptr, RmiPdevParamsAt(old_s, params_ptr).aux, RmiPdevParamsAt(old_s, params_ptr).num_aux as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AuxStateEqual32(old_s, RmiPdevParamsAt(old_s, params_ptr).aux, RmiPdevParamsAt(old_s, params_ptr).num_aux as int, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> GranuleAt(new_s, pdev_ptr).state == PDEV)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).pdev_id == RmiPdevParamsAt(new_s, params_ptr).pdev_id)
  && (result.is_Ok() ==> Equal(PdevAt(new_s, pdev_ptr).prot, RmiPdevParamsAt(new_s, params_ptr).flags.prot))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).segment_id == RmiPdevParamsAt(new_s, params_ptr).segment_id)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).ecam_addr == RmiPdevParamsAt(new_s, params_ptr).ecam_addr)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).root_id == RmiPdevParamsAt(new_s, params_ptr).root_id)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).cert_id == RmiPdevParamsAt(new_s, params_ptr).cert_id)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).rid_base == RmiPdevParamsAt(new_s, params_ptr).rid_base)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).rid_top == RmiPdevParamsAt(new_s, params_ptr).rid_top)
  && (result.is_Ok() ==> Equal(PdevAt(new_s, pdev_ptr).hash_algo, RmiPdevParamsAt(new_s, params_ptr).hash_algo))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).ide_sid == RmiPdevParamsAt(new_s, params_ptr).ide_sid)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).ncoh_num_addr_range == RmiPdevParamsAt(new_s, params_ptr).ncoh_num_addr_range)
  && (result.is_Ok() ==> RmiAddressRangesEqual16(new_s, PdevAt(new_s, pdev_ptr).ncoh_addr_range,RmiPdevParamsAt(new_s, params_ptr).ncoh_addr_range,RmiPdevParamsAt(new_s, params_ptr).ncoh_num_addr_range as int))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).coh_num_addr_range == RmiPdevParamsAt(new_s, params_ptr).coh_num_addr_range)
  && (result.is_Ok() ==> RmiAddressRangesEqual4(new_s, PdevAt(new_s, pdev_ptr).coh_addr_range,RmiPdevParamsAt(new_s, params_ptr).coh_addr_range,RmiPdevParamsAt(new_s, params_ptr).coh_num_addr_range as int))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEW)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).num_vdevs == 0)
  && (result.is_Ok() ==> AuxEqual32(new_s, PdevAt(new_s, pdev_ptr).aux, RmiPdevParamsAt(new_s, params_ptr).aux, PdevAuxCount(new_s, RmiPdevParamsAt(new_s, params_ptr).flags)))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).num_aux == PdevAuxCount(new_s, RmiPdevParamsAt(new_s, params_ptr).flags))
  && (result.is_Ok() ==> AuxStateEqual32(new_s, PdevAt(new_s, pdev_ptr).aux, PdevAuxCount(new_s, RmiPdevParamsAt(new_s, params_ptr).flags) as int, PDEV_AUX))
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegableDram(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != DELEGATED) &&
       AddrIsGranuleAligned(old_s, params_ptr) &&
       GranuleAccessPermitted(old_s, params_ptr, PAS_NS) &&
       RmiPdevParamsIsValid(old_s, params_ptr) &&
       !(RmiPdevParamsAt(old_s, params_ptr).num_aux != PdevAuxCount(old_s, RmiPdevParamsAt(old_s, params_ptr).flags)) &&
       AuxAligned32(old_s, RmiPdevParamsAt(old_s, params_ptr).aux, RmiPdevParamsAt(old_s, params_ptr).num_aux as int) &&
       !(AuxAlias32(old_s, pdev_ptr, RmiPdevParamsAt(old_s, params_ptr).aux, RmiPdevParamsAt(old_s, params_ptr).num_aux as int)) &&
       AuxStateEqual32(old_s, RmiPdevParamsAt(old_s, params_ptr).aux, RmiPdevParamsAt(old_s, params_ptr).num_aux as int, DELEGATED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, pdev_ptr).state == GranuleAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).pdev_id == PdevAt(old_s, pdev_ptr).pdev_id)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).segment_id == PdevAt(old_s, pdev_ptr).segment_id)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).ecam_addr == PdevAt(old_s, pdev_ptr).ecam_addr)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).root_id == PdevAt(old_s, pdev_ptr).root_id)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).cert_id == PdevAt(old_s, pdev_ptr).cert_id)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).rid_base == PdevAt(old_s, pdev_ptr).rid_base)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).rid_top == PdevAt(old_s, pdev_ptr).rid_top)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).ide_sid == PdevAt(old_s, pdev_ptr).ide_sid)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).ncoh_num_addr_range == PdevAt(old_s, pdev_ptr).ncoh_num_addr_range)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).coh_num_addr_range == PdevAt(old_s, pdev_ptr).coh_num_addr_range)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).num_vdevs == PdevAt(old_s, pdev_ptr).num_vdevs)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).num_aux == PdevAt(old_s, pdev_ptr).num_aux)
}

pub open spec fn rmi_pdev_destroy_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_ptr).state != PDEV_STOPPED ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (PdevAt(old_s, pdev_ptr).num_vdevs != 0 ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> GranuleAt(new_s, pdev_ptr).state == DELEGATED)
  && (result.is_Ok() ==> AuxStateEqual32(new_s, PdevAt(new_s, pdev_ptr).aux, PdevAt(new_s, pdev_ptr).num_aux as int, DELEGATED))
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       !(PdevAt(old_s, pdev_ptr).state != PDEV_STOPPED) &&
       !(PdevAt(old_s, pdev_ptr).num_vdevs != 0))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, pdev_ptr).state == GranuleAt(old_s, pdev_ptr).state)
}

pub open spec fn rmi_pdev_get_state_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, state: RmiPdevState, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> Equal(state, PdevAt(new_s, pdev_ptr).state))
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV))
    ==> result.is_Ok())
}

pub open spec fn rmi_pdev_ide_reset_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_ptr).state != PDEV_READY ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).state == PDEV_IDE_RESETTING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       !(PdevAt(old_s, pdev_ptr).state != PDEV_READY))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
}

pub open spec fn rmi_pdev_notify_spec(pdev_ptr: Address, ev: RmiPdevEvent, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_ptr).state != PDEV_READY ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (!RmiPdevEventIsValid(old_s, ev) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).state == PDEV_COMMUNICATING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       !(PdevAt(old_s, pdev_ptr).state != PDEV_READY) &&
       RmiPdevEventIsValid(old_s, ev))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
}

pub open spec fn rmi_pdev_set_pubkey_spec(pdev_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RmiPublicKeyParamsAt(old_s, params_ptr).key_len > 0x1000 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RmiPublicKeyParamsAt(old_s, params_ptr).metadata_len > 0x1000 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_ptr).state != PDEV_NEEDS_KEY ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).state == PDEV_HAS_KEY)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       AddrIsGranuleAligned(old_s, params_ptr) &&
       GranuleAccessPermitted(old_s, params_ptr, PAS_NS) &&
       !(RmiPublicKeyParamsAt(old_s, params_ptr).key_len > 0x1000) &&
       !(RmiPublicKeyParamsAt(old_s, params_ptr).metadata_len > 0x1000) &&
       !(PdevAt(old_s, pdev_ptr).state != PDEV_NEEDS_KEY))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
}

pub open spec fn rmi_pdev_stop_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PdevAt(old_s, pdev_ptr).state == PDEV_COMMUNICATING || PdevAt(old_s, pdev_ptr).state == PDEV_STOPPING || PdevAt(old_s, pdev_ptr).state == PDEV_STOPPED) ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (PdevAt(old_s, pdev_ptr).num_vdevs != 0 ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).state == PDEV_STOPPING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       !((PdevAt(old_s, pdev_ptr).state == PDEV_COMMUNICATING || PdevAt(old_s, pdev_ptr).state == PDEV_STOPPING || PdevAt(old_s, pdev_ptr).state == PDEV_STOPPED)) &&
       !(PdevAt(old_s, pdev_ptr).num_vdevs != 0))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
}

pub open spec fn rmi_psci_complete_spec(calling_rec_ptr: Address, target_rec_ptr: Address, status: PsciReturnCode, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (calling_rec_ptr == target_rec_ptr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, calling_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, calling_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, calling_rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, target_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, target_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, target_rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, calling_rec_ptr).pending != REC_PENDING_PSCI ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, target_rec_ptr).owner != RecAt(old_s, calling_rec_ptr).owner ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, target_rec_ptr).mpidr != RecAt(old_s, calling_rec_ptr).gprs[1] ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PsciReturnCodePermitted(old_s, RecAt(old_s, calling_rec_ptr), RecAt(old_s, target_rec_ptr), status) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> RecAt(new_s, calling_rec_ptr).pending == REC_PENDING_NONE)
  && (result.is_Ok() && (status == PSCI_SUCCESS && RecAt(old_s, calling_rec_ptr).gprs[0] == FID_PSCI_CPU_ON && RecAt(old_s, target_rec_ptr).flags.runnable == RUNNABLE) ==> (RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_ALREADY_ON)))
  && (result.is_Ok() && (status == PSCI_SUCCESS && RecAt(old_s, calling_rec_ptr).gprs[0] == FID_PSCI_CPU_ON && RecAt(old_s, target_rec_ptr).flags.runnable != RUNNABLE) ==> (RecAt(new_s, target_rec_ptr).gprs[0] == RecAt(new_s, calling_rec_ptr).gprs[3] && RecAt(new_s, target_rec_ptr).gprs[1] == 0 && RecAt(new_s, target_rec_ptr).gprs[2] == 0 && RecAt(new_s, target_rec_ptr).gprs[3] == 0 && RecAt(new_s, target_rec_ptr).gprs[4] == 0 && RecAt(new_s, target_rec_ptr).gprs[5] == 0 && RecAt(new_s, target_rec_ptr).gprs[6] == 0 && RecAt(new_s, target_rec_ptr).gprs[7] == 0 && RecAt(new_s, target_rec_ptr).gprs[8] == 0 && RecAt(new_s, target_rec_ptr).gprs[9] == 0 && RecAt(new_s, target_rec_ptr).gprs[10] == 0 && RecAt(new_s, target_rec_ptr).gprs[11] == 0 && RecAt(new_s, target_rec_ptr).gprs[12] == 0 && RecAt(new_s, target_rec_ptr).gprs[13] == 0 && RecAt(new_s, target_rec_ptr).gprs[14] == 0 && RecAt(new_s, target_rec_ptr).gprs[15] == 0 && RecAt(new_s, target_rec_ptr).gprs[16] == 0 && RecAt(new_s, target_rec_ptr).gprs[17] == 0 && RecAt(new_s, target_rec_ptr).gprs[18] == 0 && RecAt(new_s, target_rec_ptr).gprs[19] == 0 && RecAt(new_s, target_rec_ptr).gprs[20] == 0 && RecAt(new_s, target_rec_ptr).gprs[21] == 0 && RecAt(new_s, target_rec_ptr).gprs[22] == 0 && RecAt(new_s, target_rec_ptr).gprs[23] == 0 && RecAt(new_s, target_rec_ptr).gprs[24] == 0 && RecAt(new_s, target_rec_ptr).gprs[25] == 0 && RecAt(new_s, target_rec_ptr).gprs[26] == 0 && RecAt(new_s, target_rec_ptr).gprs[27] == 0 && RecAt(new_s, target_rec_ptr).gprs[28] == 0 && RecAt(new_s, target_rec_ptr).gprs[29] == 0 && RecAt(new_s, target_rec_ptr).gprs[30] == 0 && RecAt(new_s, target_rec_ptr).gprs[31] == 0 && RecAt(new_s, target_rec_ptr).pc == RecAt(new_s, calling_rec_ptr).gprs[2] && RecAt(new_s, target_rec_ptr).flags.runnable == RUNNABLE && RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_SUCCESS)))
  && (result.is_Ok() && (status == PSCI_SUCCESS && RecAt(old_s, calling_rec_ptr).gprs[0] == FID_PSCI_AFFINITY_INFO && RecAt(old_s, target_rec_ptr).flags.runnable == RUNNABLE) ==> (RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_SUCCESS)))
  && (result.is_Ok() && (status == PSCI_SUCCESS && RecAt(old_s, calling_rec_ptr).gprs[0] == FID_PSCI_AFFINITY_INFO && RecAt(old_s, target_rec_ptr).flags.runnable != RUNNABLE) ==> (RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_OFF)))
  && (result.is_Ok() && status != PSCI_SUCCESS ==> (RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(new_s, status)))
  && (result.is_Ok() ==> (RecAt(new_s, calling_rec_ptr).gprs[1] == 0 && RecAt(new_s, calling_rec_ptr).gprs[2] == 0 && RecAt(new_s, calling_rec_ptr).gprs[3] == 0))
  && ((!(calling_rec_ptr == target_rec_ptr) &&
       AddrIsGranuleAligned(old_s, calling_rec_ptr) &&
       PaIsDelegable(old_s, calling_rec_ptr) &&
       !(GranuleAt(old_s, calling_rec_ptr).state != REC) &&
       AddrIsGranuleAligned(old_s, target_rec_ptr) &&
       PaIsDelegable(old_s, target_rec_ptr) &&
       !(GranuleAt(old_s, target_rec_ptr).state != REC) &&
       !(RecAt(old_s, calling_rec_ptr).pending != REC_PENDING_PSCI) &&
       !(RecAt(old_s, target_rec_ptr).owner != RecAt(old_s, calling_rec_ptr).owner) &&
       !(RecAt(old_s, target_rec_ptr).mpidr != RecAt(old_s, calling_rec_ptr).gprs[1]) &&
       PsciReturnCodePermitted(old_s, RecAt(old_s, calling_rec_ptr), RecAt(old_s, target_rec_ptr), status))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, calling_rec_ptr).pending == RecAt(old_s, calling_rec_ptr).pending)
  && (result.is_Err()
    ==> RecAt(new_s, calling_rec_ptr).gprs[0] == RecAt(old_s, calling_rec_ptr).gprs[0])
  && (result.is_Err()
    ==> RecAt(new_s, calling_rec_ptr).gprs[0] == RecAt(old_s, calling_rec_ptr).gprs[0])
  && (result.is_Err()
    ==> RecAt(new_s, calling_rec_ptr).gprs[0] == RecAt(old_s, calling_rec_ptr).gprs[0])
  && (result.is_Err()
    ==> RecAt(new_s, calling_rec_ptr).gprs[0] == RecAt(old_s, calling_rec_ptr).gprs[0])
}

pub open spec fn rmi_psmmu_irq_notify_spec(psmmu: Address, irq: RmiSmmuIrq, result: Result<(), RmiStatusCode>, action: RmiSmmuAction, rd: Address, vsmmu: Address, msi_addr: Address, msi_data: Bits64, old_s: S, new_s: S) -> bool {
  (!PsmmuAddrIsValid(old_s, psmmu) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PsmmuAddrIsValid(old_s, psmmu))
    ==> result.is_Ok())
}

pub open spec fn rmi_psmmu_msi_config_spec(psmmu: Address, gerr_addr: Address, gerr_data: Bits64, eventq_addr: Address, eventq_data: Bits64, priq_addr: Address, priq_data: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!PsmmuAddrIsValid(old_s, psmmu) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PsmmuSupportsMsi(old_s, psmmu) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!MsiAddrIsValid(old_s, gerr_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!MsiAddrIsValid(old_s, eventq_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!MsiAddrIsValid(old_s, priq_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PsmmuAddrIsValid(old_s, psmmu) &&
       PsmmuSupportsMsi(old_s, psmmu) &&
       MsiAddrIsValid(old_s, gerr_addr) &&
       MsiAddrIsValid(old_s, eventq_addr) &&
       MsiAddrIsValid(old_s, priq_addr))
    ==> result.is_Ok())
}

pub open spec fn rmi_realm_activate_spec(rd: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (result.is_Ok() ==> RealmAt(new_s, rd).state == REALM_ACTIVE)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !(RealmAt(old_s, rd).state != REALM_NEW))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RealmAt(new_s, rd).state == RealmAt(old_s, rd).state)
}

pub open spec fn rmi_realm_create_spec(rd: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RmiRealmParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RealmParamsSupported(old_s, RmiRealmParamsAt(old_s, params_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AddrInRange(old_s, rd, RmiRealmParamsAt(old_s, params_ptr).rtt_base,(RmiRealmParamsAt(old_s, params_ptr).rtt_num_start - 1) * RMM_GRANULE_SIZE) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDram(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, RmiRealmParamsAt(old_s, params_ptr).rtt_base,RmiRealmParamsAt(old_s, params_ptr).rtt_num_start * RMM_GRANULE_SIZE) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttConfigIsValid(old_s, RmiRealmParamsAt(old_s, params_ptr).s2sz as int,RmiRealmParamsAt(old_s, params_ptr).rtt_level_start as int, RmiRealmParamsAt(old_s, params_ptr).rtt_num_start as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttsStateEqual(RmiRealmParamsAt(old_s, params_ptr).rtt_base, RmiRealmParamsAt(old_s, params_ptr).rtt_num_start as int, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!VmidsAreValid(old_s, RmiRealmParamsAt(old_s, params_ptr).vmid, RmiRealmParamsAt(old_s, params_ptr).aux_vmid) || !VmidsAreFree1(old_s, RmiRealmParamsAt(old_s, params_ptr).vmid, RmiRealmParamsAt(old_s, params_ptr).aux_vmid)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RmiRealmParamsAt(old_s, params_ptr).mecid) > (ImplFeatures(old_s).max_mecid) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (MecState(old_s, RmiRealmParamsAt(old_s, params_ptr).mecid) == MEC_STATE_PRIVATE_ASSIGNED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> GranuleAt(new_s, rd).state == RD)
  && (result.is_Ok() ==> RealmAt(new_s, rd).state == REALM_NEW)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rec_index == 0)
  && (result.is_Ok() ==> RealmRttBaseEqual(RealmAt(new_s, rd), RmiRealmParamsAt(new_s, params_ptr).rtt_base, RmiRealmParamsAt(new_s, params_ptr).aux_rtt_base))
  && (result.is_Ok() ==> RttsStateEqual(RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start as int, RTT))
  && (result.is_Ok() ==> RttsAllProtectedEntriesState(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start as int, UNASSIGNED))
  && (result.is_Ok() ==> RttsAllUnprotectedEntriesState(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start as int, UNASSIGNED_NS))
  && (result.is_Ok() ==> RttsAllProtectedEntriesRipas(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start as int, EMPTY))
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).feat_lpa2, RmiRealmParamsAt(new_s, params_ptr).flags0.lpa2))
  && (result.is_Ok() ==> RealmAt(new_s, rd).ipa_width == RmiRealmParamsAt(new_s, params_ptr).s2sz)
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).hash_algo, RmiRealmParamsAt(new_s, params_ptr).hash_algo))
  && (result.is_Ok() ==> RealmAt(new_s, rd).measurements[0] == RimInit(new_s, RealmAt(new_s, rd).hash_algo, RmiRealmParamsAt(new_s, params_ptr)))
  && (result.is_Ok() ==> (RealmAt(new_s, rd).measurements[1] == 0 && RealmAt(new_s, rd).measurements[2] == 0 && RealmAt(new_s, rd).measurements[3] == 0 && RealmAt(new_s, rd).measurements[4] == 0))
  && (result.is_Ok() ==> RealmAt(new_s, rd).rtt_level_start == RmiRealmParamsAt(new_s, params_ptr).rtt_level_start)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rtt_num_start == RmiRealmParamsAt(new_s, params_ptr).rtt_num_start)
  && (result.is_Ok() ==> RealmVmidEqual(RealmAt(new_s, rd), RmiRealmParamsAt(new_s, params_ptr).vmid, RmiRealmParamsAt(new_s, params_ptr).aux_vmid))
  && (result.is_Ok() ==> RealmAt(new_s, rd).rpv == RmiRealmParamsAt(new_s, params_ptr).rpv)
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).feat_da, RmiRealmParamsAt(new_s, params_ptr).flags0.da))
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).rtt_tree_per_plane, RmiRealmParamsAt(new_s, params_ptr).flags1.rtt_tree_per_plane))
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_aux_planes == RmiRealmParamsAt(new_s, params_ptr).num_aux_planes)
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).rtt_s2ap_encoding, RmiRealmParamsAt(new_s, params_ptr).flags1.rtt_s2ap_encoding))
  && (result.is_Ok() ==> Equal(RealmAt(new_s, rd).lfa_policy, RmiRealmParamsAt(new_s, params_ptr).flags0.lfa_policy))
  && (result.is_Ok() ==> RealmAt(new_s, rd).mecid == RmiRealmParamsAt(new_s, params_ptr).mecid)
  && (result.is_Ok() ==> RealmAt(new_s, rd).mec_policy == MecPolicy(new_s, RealmAt(new_s, rd).mecid))
  && (result.is_Ok() && MecState(old_s, RmiRealmParamsAt(old_s, params_ptr).mecid) == MEC_STATE_PRIVATE_UNASSIGNED ==> MecState(new_s, RmiRealmParamsAt(new_s, params_ptr).mecid) == MEC_STATE_PRIVATE_ASSIGNED)
  && (result.is_Ok() && MecState(old_s, RmiRealmParamsAt(old_s, params_ptr).mecid) == MEC_STATE_SHARED ==> MecMembers(new_s, RmiRealmParamsAt(new_s, params_ptr).mecid) == MecMembers(new_s, RmiRealmParamsAt(new_s, params_ptr).mecid) + 1)
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_recs == 0)
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vdevs == 0)
  && (result.is_Ok() ==> RealmAt(new_s, rd).vdev_count == 0)
  && ((AddrIsGranuleAligned(old_s, params_ptr) &&
       GranuleAccessPermitted(old_s, params_ptr, PAS_NS) &&
       RmiRealmParamsIsValid(old_s, params_ptr) &&
       RealmParamsSupported(old_s, RmiRealmParamsAt(old_s, params_ptr)) &&
       !(AddrInRange(old_s, rd, RmiRealmParamsAt(old_s, params_ptr).rtt_base,(RmiRealmParamsAt(old_s, params_ptr).rtt_num_start - 1) * RMM_GRANULE_SIZE)) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegableDram(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != DELEGATED) &&
       AddrIsAligned(old_s, RmiRealmParamsAt(old_s, params_ptr).rtt_base,RmiRealmParamsAt(old_s, params_ptr).rtt_num_start * RMM_GRANULE_SIZE) &&
       RttConfigIsValid(old_s, RmiRealmParamsAt(old_s, params_ptr).s2sz as int,RmiRealmParamsAt(old_s, params_ptr).rtt_level_start as int, RmiRealmParamsAt(old_s, params_ptr).rtt_num_start as int) &&
       RttsStateEqual(RmiRealmParamsAt(old_s, params_ptr).rtt_base, RmiRealmParamsAt(old_s, params_ptr).rtt_num_start as int, DELEGATED) &&
       !((!VmidsAreValid(old_s, RmiRealmParamsAt(old_s, params_ptr).vmid, RmiRealmParamsAt(old_s, params_ptr).aux_vmid) || !VmidsAreFree1(old_s, RmiRealmParamsAt(old_s, params_ptr).vmid, RmiRealmParamsAt(old_s, params_ptr).aux_vmid))) &&
       !((RmiRealmParamsAt(old_s, params_ptr).mecid) > (ImplFeatures(old_s).max_mecid)) &&
       !(MecState(old_s, RmiRealmParamsAt(old_s, params_ptr).mecid) == MEC_STATE_PRIVATE_ASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).state == RealmAt(old_s, rd).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rec_index == RealmAt(old_s, rd).rec_index)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).ipa_width == RealmAt(old_s, rd).ipa_width)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).measurements[0] == RealmAt(old_s, rd).measurements[0])
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rtt_level_start == RealmAt(old_s, rd).rtt_level_start)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rtt_num_start == RealmAt(old_s, rd).rtt_num_start)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rpv == RealmAt(old_s, rd).rpv)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_aux_planes == RealmAt(old_s, rd).num_aux_planes)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).mecid == RealmAt(old_s, rd).mecid)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).mec_policy == RealmAt(old_s, rd).mec_policy)
  && (result.is_Err()
    ==> MecState(new_s, RmiRealmParamsAt(new_s, params_ptr).mecid) == MecState(old_s, RmiRealmParamsAt(old_s, params_ptr).mecid))
  && (result.is_Err()
    ==> MecMembers(new_s, RmiRealmParamsAt(new_s, params_ptr).mecid) == MecMembers(old_s, RmiRealmParamsAt(old_s, params_ptr).mecid))
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_recs == RealmAt(old_s, rd).num_recs)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_vdevs == RealmAt(old_s, rd).num_vdevs)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).vdev_count == RealmAt(old_s, rd).vdev_count)
  && (!(result.is_Ok() && (MecState(old_s, RmiRealmParamsAt(old_s, params_ptr).mecid) == MEC_STATE_PRIVATE_UNASSIGNED)) ==> MecState(new_s, RmiRealmParamsAt(new_s, params_ptr).mecid) == MecState(old_s, RmiRealmParamsAt(old_s, params_ptr).mecid))
}

pub open spec fn rmi_realm_destroy_spec(rd: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmIsLive(old_s, rd) ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (result.is_Ok() ==> RttsStateEqual(RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start as int, DELEGATED))
  && (result.is_Ok() ==> GranuleAt(new_s, rd).state == DELEGATED)
  && (result.is_Ok() ==> VmidsAreFree(new_s, RealmAt(new_s, rd).vmid))
  && (result.is_Ok() && RealmAt(old_s, rd).mec_policy == MEC_POLICY_PRIVATE ==> MecState(new_s, RealmAt(new_s, rd).mecid) == MEC_STATE_PRIVATE_UNASSIGNED)
  && (result.is_Ok() && RealmAt(old_s, rd).mec_policy == MEC_POLICY_SHARED ==> MecMembers(new_s, RealmAt(new_s, rd).mecid) == MecMembers(new_s, RealmAt(new_s, rd).mecid) - 1)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !(RealmIsLive(old_s, rd)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
  && (result.is_Err()
    ==> MecState(new_s, RealmAt(new_s, rd).mecid) == MecState(old_s, RealmAt(old_s, rd).mecid))
  && (result.is_Err()
    ==> MecMembers(new_s, RealmAt(new_s, rd).mecid) == MecMembers(old_s, RealmAt(old_s, rd).mecid))
  && (!(result.is_Ok() && (RealmAt(old_s, rd).mec_policy == MEC_POLICY_PRIVATE)) ==> MecState(new_s, RealmAt(new_s, rd).mecid) == MecState(old_s, RealmAt(old_s, rd).mecid))
}

pub open spec fn rmi_rec_aux_count_spec(rd: Address, result: Result<(), RmiStatusCode>, aux_count: UInt64, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> aux_count == RecAuxCount(new_s, rd))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD))
    ==> result.is_Ok())
}

pub open spec fn rmi_rec_create_spec(rd: Address, rec_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDram(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (RealmAt(old_s, rd).num_recs == pow2(ImplFeatures(old_s).max_recs_order as nat) - 1 ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (RecIndex(old_s, RmiRecParamsAt(old_s, params_ptr).mpidr) != RealmAt(old_s, rd).rec_index ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RmiRecParamsAt(old_s, params_ptr).num_aux != RecAuxCount(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AuxAligned16(old_s, RmiRecParamsAt(old_s, params_ptr).aux, RmiRecParamsAt(old_s, params_ptr).num_aux as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AuxAlias16(old_s, rec_ptr, RmiRecParamsAt(old_s, params_ptr).aux, RmiRecParamsAt(old_s, params_ptr).num_aux as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AuxStateEqual16(old_s, RmiRecParamsAt(old_s, params_ptr).aux, RmiRecParamsAt(old_s, params_ptr).num_aux as int, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> RealmAt(new_s, rd).rec_index == RealmAt(new_s, rd).rec_index + 1)
  && (result.is_Ok() ==> GranuleAt(new_s, rec_ptr).state == REC)
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).owner == rd)
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).attest_state == NO_ATTEST_IN_PROGRESS)
  && (result.is_Ok() ==> MpidrEqual(RecAt(new_s, rec_ptr).mpidr, RmiRecParamsAt(new_s, params_ptr).mpidr))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).state == REC_READY)
  && (result.is_Ok() && RmiRecParamsAt(old_s, params_ptr).flags.runnable == RMI_RUNNABLE ==> RecAt(new_s, rec_ptr).flags.runnable == RUNNABLE)
  && (result.is_Ok() && RmiRecParamsAt(old_s, params_ptr).flags.runnable == RMI_NOT_RUNNABLE ==> RecAt(new_s, rec_ptr).flags.runnable == NOT_RUNNABLE)
  && (result.is_Ok() ==> (RecAt(new_s, rec_ptr).gprs[0] == RmiRecParamsAt(new_s, params_ptr).gprs[0] && RecAt(new_s, rec_ptr).gprs[1] == RmiRecParamsAt(new_s, params_ptr).gprs[1] && RecAt(new_s, rec_ptr).gprs[2] == RmiRecParamsAt(new_s, params_ptr).gprs[2] && RecAt(new_s, rec_ptr).gprs[3] == RmiRecParamsAt(new_s, params_ptr).gprs[3] && RecAt(new_s, rec_ptr).gprs[4] == RmiRecParamsAt(new_s, params_ptr).gprs[4] && RecAt(new_s, rec_ptr).gprs[5] == RmiRecParamsAt(new_s, params_ptr).gprs[5] && RecAt(new_s, rec_ptr).gprs[6] == RmiRecParamsAt(new_s, params_ptr).gprs[6] && RecAt(new_s, rec_ptr).gprs[7] == RmiRecParamsAt(new_s, params_ptr).gprs[7] && RecAt(new_s, rec_ptr).gprs[8] == 0 && RecAt(new_s, rec_ptr).gprs[9] == 0 && RecAt(new_s, rec_ptr).gprs[10] == 0 && RecAt(new_s, rec_ptr).gprs[11] == 0 && RecAt(new_s, rec_ptr).gprs[12] == 0 && RecAt(new_s, rec_ptr).gprs[13] == 0 && RecAt(new_s, rec_ptr).gprs[14] == 0 && RecAt(new_s, rec_ptr).gprs[15] == 0 && RecAt(new_s, rec_ptr).gprs[16] == 0 && RecAt(new_s, rec_ptr).gprs[17] == 0 && RecAt(new_s, rec_ptr).gprs[18] == 0 && RecAt(new_s, rec_ptr).gprs[19] == 0 && RecAt(new_s, rec_ptr).gprs[20] == 0 && RecAt(new_s, rec_ptr).gprs[21] == 0 && RecAt(new_s, rec_ptr).gprs[22] == 0 && RecAt(new_s, rec_ptr).gprs[23] == 0 && RecAt(new_s, rec_ptr).gprs[24] == 0 && RecAt(new_s, rec_ptr).gprs[25] == 0 && RecAt(new_s, rec_ptr).gprs[26] == 0 && RecAt(new_s, rec_ptr).gprs[27] == 0 && RecAt(new_s, rec_ptr).gprs[28] == 0 && RecAt(new_s, rec_ptr).gprs[29] == 0 && RecAt(new_s, rec_ptr).gprs[30] == 0 && RecAt(new_s, rec_ptr).gprs[31] == 0))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).pc == RmiRecParamsAt(new_s, params_ptr).pc)
  && (result.is_Ok() && RmiRecParamsAt(old_s, params_ptr).flags.runnable == RMI_RUNNABLE ==> RealmAt(new_s, rd).measurements[0] == RimExtendRec(new_s, RealmAt(new_s, rd), RmiRecParamsAt(new_s, params_ptr)))
  && (result.is_Ok() ==> AuxEqual16(new_s, RecAt(new_s, rec_ptr).aux, RmiRecParamsAt(new_s, params_ptr).aux, RecAuxCount(new_s, rd)))
  && (result.is_Ok() ==> AuxStateEqual16(new_s, RecAt(new_s, rec_ptr).aux, RecAuxCount(new_s, rd) as int, REC_AUX))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).ripas_addr == 0)
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).ripas_top == 0)
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).pending == REC_PENDING_NONE)
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_recs == RealmAt(new_s, rd).num_recs + 1)
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).gic_owner == 0)
  && ((AddrIsGranuleAligned(old_s, params_ptr) &&
       GranuleAccessPermitted(old_s, params_ptr, PAS_NS) &&
       AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegableDram(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != DELEGATED) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       !(RealmAt(old_s, rd).num_recs == pow2(ImplFeatures(old_s).max_recs_order as nat) - 1) &&
       !(RecIndex(old_s, RmiRecParamsAt(old_s, params_ptr).mpidr) != RealmAt(old_s, rd).rec_index) &&
       !(RmiRecParamsAt(old_s, params_ptr).num_aux != RecAuxCount(old_s, rd)) &&
       AuxAligned16(old_s, RmiRecParamsAt(old_s, params_ptr).aux, RmiRecParamsAt(old_s, params_ptr).num_aux as int) &&
       !(AuxAlias16(old_s, rec_ptr, RmiRecParamsAt(old_s, params_ptr).aux, RmiRecParamsAt(old_s, params_ptr).num_aux as int)) &&
       AuxStateEqual16(old_s, RmiRecParamsAt(old_s, params_ptr).aux, RmiRecParamsAt(old_s, params_ptr).num_aux as int, DELEGATED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rec_index == RealmAt(old_s, rd).rec_index)
  && (result.is_Err()
    ==> GranuleAt(new_s, rec_ptr).state == GranuleAt(old_s, rec_ptr).state)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).owner == RecAt(old_s, rec_ptr).owner)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).attest_state == RecAt(old_s, rec_ptr).attest_state)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).state == RecAt(old_s, rec_ptr).state)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).flags.runnable == RecAt(old_s, rec_ptr).flags.runnable)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).flags.runnable == RecAt(old_s, rec_ptr).flags.runnable)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).pc == RecAt(old_s, rec_ptr).pc)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).measurements[0] == RealmAt(old_s, rd).measurements[0])
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).ripas_addr == RecAt(old_s, rec_ptr).ripas_addr)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).ripas_top == RecAt(old_s, rec_ptr).ripas_top)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).pending == RecAt(old_s, rec_ptr).pending)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_recs == RealmAt(old_s, rd).num_recs)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).gic_owner == RecAt(old_s, rec_ptr).gic_owner)
  && (!(result.is_Ok() && (RmiRecParamsAt(old_s, params_ptr).flags.runnable == RMI_RUNNABLE)) ==> RecAt(new_s, rec_ptr).flags.runnable == RecAt(old_s, rec_ptr).flags.runnable)
}

pub open spec fn rmi_rec_destroy_spec(rec_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (result.is_Ok() ==> GranuleAt(new_s, rec_ptr).state == DELEGATED)
  && (result.is_Ok() ==> AuxStateEqual16(new_s, RecAt(new_s, rec_ptr).aux, RecAuxCount(new_s, RecAt(new_s, rec_ptr).owner) as int, DELEGATED))
  && (result.is_Ok() ==> RealmAt(new_s, RecAt(new_s, rec_ptr).owner).num_recs == RealmAt(new_s, RecAt(new_s, rec_ptr).owner).num_recs - 1)
  && ((AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != REC) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rec_ptr).state == GranuleAt(old_s, rec_ptr).state)
  && (result.is_Err()
    ==> RealmAt(new_s, RecAt(new_s, rec_ptr).owner).num_recs == RealmAt(old_s, RecAt(old_s, rec_ptr).owner).num_recs)
}

pub open spec fn rmi_rec_enter_spec(rec_ptr: Address, run_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, run_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, run_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state == REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0 as int)))
  && (RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state == REALM_SYSTEM_OFF ==> ResultEqual(result, RMI_ERROR_REALM(1 as int)))
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).flags.runnable == NOT_RUNNABLE ==> ResultEqual(result, RMI_ERROR_REC))
  && ((RmiRecRunAt(old_s, run_ptr).enter.flags.emul_mmio == RMI_EMULATED_MMIO && RecAt(old_s, rec_ptr).emulatable_abort != EMULATABLE_ABORT) ==> ResultEqual(result, RMI_ERROR_REC))
  && (!Gicv3ConfigIsValid(old_s, RmiRecRunAt(old_s, run_ptr).enter.gicv3_hcr, RmiRecRunAt(old_s, run_ptr).enter.gicv3_lrs) ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).pending != REC_PENDING_NONE ==> ResultEqual(result, RMI_ERROR_REC))
  && ((AddrIsGranuleAligned(old_s, run_ptr) &&
       GranuleAccessPermitted(old_s, run_ptr, PAS_NS) &&
       AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != REC) &&
       !(RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state == REALM_NEW) &&
       !(RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state == REALM_SYSTEM_OFF) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) &&
       !(RecAt(old_s, rec_ptr).flags.runnable == NOT_RUNNABLE) &&
       !((RmiRecRunAt(old_s, run_ptr).enter.flags.emul_mmio == RMI_EMULATED_MMIO && RecAt(old_s, rec_ptr).emulatable_abort != EMULATABLE_ABORT)) &&
       Gicv3ConfigIsValid(old_s, RmiRecRunAt(old_s, run_ptr).enter.gicv3_hcr, RmiRecRunAt(old_s, run_ptr).enter.gicv3_lrs) &&
       !(RecAt(old_s, rec_ptr).pending != REC_PENDING_NONE))
    ==> result.is_Ok())
}

pub open spec fn rmi_rtt_aux_create_spec(rd: Address, rtt: Address, ipa: Address, level: Int64, index: UInt64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDram(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rtt).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((rtt) >= 2^48)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).level as int)))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.state == TABLE ==> ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).level as int)))
  && (result.is_Ok() ==> GranuleAt(new_s, rtt).state == RTT)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.state == TABLE)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr == rtt)
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> RttAllEntriesRipas(new_s, RttAt(new_s, rtt), RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.ripas))
  && (result.is_Ok() ==> RttAllEntriesState(new_s, RttAt(new_s, rtt), RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.state))
  && (result.is_Ok() && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.state != UNASSIGNED && RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.state != UNASSIGNED_NS) ==> RttAllEntriesContiguous(new_s, RttAt(new_s, rtt), RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr, level as int))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat)) &&
       !((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes)) &&
       AddrIsGranuleAligned(old_s, rtt) &&
       PaIsDelegableDram(old_s, rtt) &&
       !(GranuleAt(old_s, rtt).state != DELEGATED) &&
       !(((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((rtt) >= 2^48))) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).level < level - 1) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.state == TABLE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rtt).state == GranuleAt(old_s, rtt).state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.ripas)
}

pub open spec fn rmi_rtt_aux_destroy_spec(rd: Address, ipa: Address, level: Int64, index: UInt64, result: Result<(), RmiStatusCode>, rtt: Address, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).level < level - 1 ==> (ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).level,ipa))))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.state != TABLE ==> (ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).level,ipa))))
  && (RttIsLive(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr)) ==> (ResultEqual(result, RMI_ERROR_RTT_AUX(level as int)) && (top == ipa)))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.state == AUX_DESTROYED)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.ripas == DESTROYED)
  && (result.is_Ok() ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr).state == DELEGATED)
  && (result.is_Ok() ==> rtt == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).level,ipa))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat)) &&
       !((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).level < level - 1) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.state != TABLE) &&
       !(RttIsLive(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.ripas)
  && (result.is_Err()
    ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr).state == GranuleAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr).state)
}

pub open spec fn rmi_rtt_aux_fold_spec(rd: Address, ipa: Address, level: Int64, index: UInt64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).level as int)))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.state != TABLE ==> ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).level as int)))
  && (!RttIsHomogeneous(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr)) ==> ResultEqual(result, RMI_ERROR_RTT_AUX(level as int)))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.state == RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)).state)
  && (result.is_Ok() && (RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr)).state != UNASSIGNED && RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr)).state != UNASSIGNED_NS) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr == RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)).addr)
  && (result.is_Ok() && RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr)).state == ASSIGNED ==> (RttMemAttrEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)), RTT_PROTECTED) && RttS2APEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)), S2AP_INDIRECT)))
  && (result.is_Ok() && RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr)).state == ASSIGNED_NS ==> (RttMemAttrEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)), RTT_UNPROTECTED) && RttS2APEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)),RealmAt(new_s, rd).rtt_s2ap_encoding)))
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.ripas == RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)).ripas)
  && (result.is_Ok() ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr).state == DELEGATED)
  && (result.is_Ok() ==> rtt == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat)) &&
       !((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).level < level - 1) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.state != TABLE) &&
       RttIsHomogeneous(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.ripas)
  && (result.is_Err()
    ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr).state == GranuleAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr).state)
}

pub open spec fn rmi_rtt_aux_map_protected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, fail_index: UInt64, level_pri: Int64, state: RmiRttEntryState, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_DEV) ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && (fail_index == RMM_RTT_TREE_PRIMARY)  && (level_pri == RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)  && (state == RttEntryStateToRmi(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state))  && (ripas == RipasToRmi(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas))))
  && ((RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RAM) ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && (fail_index == RMM_RTT_TREE_PRIMARY)  && (level_pri == RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)  && (state == RttEntryStateToRmi(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state))  && (ripas == RipasToRmi(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas))))
  && ((RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_DEV && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != DEV) ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && (fail_index == RMM_RTT_TREE_PRIMARY)  && (level_pri == RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)  && (state == RttEntryStateToRmi(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state))  && (ripas == RipasToRmi(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas))))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state == AUX_DESTROYED ==> (ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level as int)) && (fail_index == index)  && (level_pri == RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)  && (state == RttEntryStateToRmi(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state))  && (ripas == RipasToRmi(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas))))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level < RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level ==> (ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level as int)) && (fail_index == index)  && (level_pri == RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)  && (state == RttEntryStateToRmi(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state))  && (ripas == RipasToRmi(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas))))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state == ASSIGNED)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.attr_prot == RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.sh == RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.addr ==RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr + (RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level) * RttLevelSize(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level as int)))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes)) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_DEV)) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RAM)) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_DEV && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != DEV)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state == AUX_DESTROYED) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level < RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.attr_prot == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.attr_prot)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.sh == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.sh)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.addr)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.ripas)
}

pub open spec fn rmi_rtt_aux_map_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, fail_index: UInt64, level_pri: Int64, state: RmiRttEntryState, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_NS ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && (fail_index == RMM_RTT_TREE_PRIMARY)  && (state == RttEntryStateToRmi(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state))  && (level_pri == RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level < RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level ==> (ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level as int)) && (fail_index == index)  && (level_pri == RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)  && (state == RttEntryStateToRmi(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state))))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state == ASSIGNED_NS)
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> (RttMemAttrEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte, RTT_PROTECTED) && RttS2APEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte,RealmAt(new_s, rd).rtt_s2ap_encoding)))
  && (result.is_Ok() && !AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> (RttMemAttrEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte, RTT_UNPROTECTED) && RttS2APEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte,RealmAt(new_s, rd).rtt_s2ap_encoding)))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.addr ==RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr + (RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level) * RttLevelSize(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level as int)))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       !(((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)))) &&
       !((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_NS) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level < RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.addr)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.ripas)
}

pub open spec fn rmi_rtt_aux_unmap_protected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, top: Address, level: Int64, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state != ASSIGNED ==> (ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level,ipa))))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state == UNASSIGNED)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level,ipa))
  && (result.is_Ok() ==> level == RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state != ASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.ripas)
}

pub open spec fn rmi_rtt_aux_unmap_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, top: Address, level: Int64, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state != ASSIGNED_NS ==> (ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level,ipa))))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state == UNASSIGNED_NS)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level,ipa))
  && (result.is_Ok() ==> level == RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       !(((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)))) &&
       !((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state != ASSIGNED_NS))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.ripas)
}

pub open spec fn rmi_rtt_create_spec(rd: Address, rtt: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDram(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rtt).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((rtt) >= 2^48)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == TABLE ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() ==> GranuleAt(new_s, rtt).state == RTT)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == TABLE)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == rtt)
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> RttAllEntriesRipas(new_s, RttAt(new_s, rtt), RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas))
  && (result.is_Ok() ==> RttAllEntriesState(new_s, RttAt(new_s, rtt), RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state))
  && (result.is_Ok() && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED && RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED_NS) ==> RttAllEntriesContiguous(new_s, RttAt(new_s, rtt), RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr, level as int))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat)) &&
       AddrIsGranuleAligned(old_s, rtt) &&
       PaIsDelegableDram(old_s, rtt) &&
       !(GranuleAt(old_s, rtt).state != DELEGATED) &&
       !(((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((rtt) >= 2^48))) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level < level - 1) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == TABLE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rtt).state == GranuleAt(old_s, rtt).state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
}

pub open spec fn rmi_rtt_destroy_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level < level - 1 ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != TABLE ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))))
  && (RttIsLive(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)) ==> (ResultEqual(result, RMI_ERROR_RTT(level as int)) && (top == ipa)))
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED)
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == DESTROYED)
  && (result.is_Ok() && !AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED_NS)
  && (result.is_Ok() ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == DELEGATED)
  && (result.is_Ok() ==> rtt == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level < level - 1) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != TABLE) &&
       !(RttIsLive(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == GranuleAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state)
}

pub open spec fn rmi_rtt_dev_mem_validate_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (base != RecAt(old_s, rec_ptr).dev_mem_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) > (RecAt(old_s, rec_ptr).dev_mem_top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((base) == (RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && ((RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_NON_COHERENT && !RttEntriesInRangeMemAttr(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false),MEMATTR_NON_CACHEABLE)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && ((RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_NON_COHERENT && !RttEntriesInRangeNonCohDevMem(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false))) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && ((RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_COHERENT && !RttEntriesInRangeMemAttr(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false),MEMATTR_PASSTHROUGH)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && ((RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_COHERENT && !RttEntriesInRangeCohDevMem(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false))) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (!RttEntriesInRangeOutputContiguous(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false),RecAt(old_s, rec_ptr).dev_mem_pa) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (AddrRangeIsAuxLive(old_s, base, top, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() ==> RttEntriesInRangeRipas(new_s,  RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false), DEV))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).dev_mem_addr == MinAddress(new_s, top, RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false)))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).dev_mem_pa == ToAddress( (RecAt(new_s, rec_ptr).dev_mem_pa) + ((RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false)) - (base))))
  && (result.is_Ok() ==> out_top == MinAddress(new_s, top, RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false)))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != REC) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) &&
       !(RecAt(old_s, rec_ptr).owner != rd) &&
       !((top) <= (base)) &&
       !(base != RecAt(old_s, rec_ptr).dev_mem_addr) &&
       !((top) > (RecAt(old_s, rec_ptr).dev_mem_top)) &&
       AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) &&
       AddrIsGranuleAligned(old_s, top) &&
       !((base) == (RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false))) &&
       !((RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_NON_COHERENT && !RttEntriesInRangeMemAttr(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false),MEMATTR_NON_CACHEABLE))) &&
       !((RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_NON_COHERENT && !RttEntriesInRangeNonCohDevMem(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false)))) &&
       !((RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_COHERENT && !RttEntriesInRangeMemAttr(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false),MEMATTR_PASSTHROUGH))) &&
       !((RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_COHERENT && !RttEntriesInRangeCohDevMem(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false)))) &&
       RttEntriesInRangeOutputContiguous(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false),RecAt(old_s, rec_ptr).dev_mem_pa) &&
       !(AddrRangeIsAuxLive(old_s, base, top, RealmAt(old_s, rd))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).dev_mem_addr == RecAt(old_s, rec_ptr).dev_mem_addr)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).dev_mem_pa == RecAt(old_s, rec_ptr).dev_mem_pa)
}

pub open spec fn rmi_rtt_fold_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != TABLE ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (!RttIsHomogeneous(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)) ==> ResultEqual(result, RMI_ERROR_RTT(level as int)))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)).state)
  && (result.is_Ok() && (RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)).state != UNASSIGNED && RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)).state != UNASSIGNED_NS) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)).addr)
  && (result.is_Ok() && RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)).state == ASSIGNED ==> (RttMemAttrEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)), RTT_PROTECTED) && RttS2APEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)), S2AP_INDIRECT)))
  && (result.is_Ok() && RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)).state == ASSIGNED_NS ==> (RttMemAttrEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)), RTT_UNPROTECTED) && RttS2APEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)),RealmAt(new_s, rd).rtt_s2ap_encoding)))
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)).ripas)
  && (result.is_Ok() ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == DELEGATED)
  && (result.is_Ok() ==> rtt == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level < level - 1) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != TABLE) &&
       RttIsHomogeneous(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
  && (result.is_Err()
    ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == GranuleAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state)
}

pub open spec fn rmi_rtt_init_ripas_spec(rd: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ToAddress((top) - RMM_GRANULE_SIZE),RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (!AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((base) == (RttSkipEntriesIfNotState(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top, UNASSIGNED)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() ==> RttEntriesInRangeRipas(new_s,  RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, RttSkipEntriesIfNotState(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top, UNASSIGNED), RAM))
  && (result.is_Ok() ==> RealmAt(new_s, rd).measurements[0] == RimExtendRipas(new_s, RealmAt(new_s, rd), base, RttSkipEntriesIfNotState(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top, UNASSIGNED), RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level))
  && (result.is_Ok() ==> out_top == RttSkipEntriesIfNotState(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top, UNASSIGNED))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !((top) <= (base)) &&
       AddrIsProtected(old_s, ToAddress((top) - RMM_GRANULE_SIZE),RealmAt(old_s, rd)) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED) &&
       AddrIsGranuleAligned(old_s, top) &&
       !((base) == (RttSkipEntriesIfNotState(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top, UNASSIGNED))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RealmAt(new_s, rd).measurements[0] == RealmAt(old_s, rd).measurements[0])
}

pub open spec fn rmi_rtt_map_unprotected_spec(rd: Address, ipa: Address, level: Int64, desc: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!RttDescriptorIsValidForUnprotected(old_s, desc) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttLevelIsBlockOrPage(old_s, RealmAt(old_s, rd), level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding).addr, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding).addr) >= 2^48)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level < level ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_NS)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_unprot == RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).attr_unprot)
  && (result.is_Ok() && RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_DIRECT ==> (RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_direct.read == RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_direct.read && RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_direct.write == RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_direct.write))
  && (result.is_Ok() && RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_INDIRECT ==> (RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_indirect.base_index == RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index && RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_indirect.overlay_index == 15))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).addr)
  && ((RttDescriptorIsValidForUnprotected(old_s, desc) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       RttLevelIsBlockOrPage(old_s, RealmAt(old_s, rd), level as int) &&
       AddrIsRttLevelAligned(old_s, RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding).addr, level as int) &&
       !(((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding).addr) >= 2^48))) &&
       AddrIsRttLevelAligned(old_s, ipa, level as int) &&
       !(((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)))) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level < level) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED_NS))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_unprot == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_unprot)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
}

pub open spec fn rmi_rtt_read_entry_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, walk_level: UInt64, state: RmiRttEntryState, desc: Bits64, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> walk_level == RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level)
  && (result.is_Ok() ==> state == RttEntryStateToRmi(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state))
  && (result.is_Ok() && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED_NS) ==> (RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).attr_unprot == 0 && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == 0 && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_indirect.overlay_index == 0 && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_direct.read == RMM_FALSE && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_direct.write == RMM_FALSE && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).addr == 0))
  && (result.is_Ok() && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_DEV || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_VSMMU || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == TABLE) ==> (RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).attr_unprot == 0 && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == 0 && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_indirect.overlay_index == 0 && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_direct.read == RMM_FALSE && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_direct.write == RMM_FALSE && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).addr == RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr))
  && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_NS ==> (RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).attr_unprot == RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_unprot && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_indirect.base_index && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_indirect.overlay_index == 0 && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_direct.read == RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_direct.read && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_direct.write == RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_direct.write && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).addr == RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr))
  && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_DEV ==> (RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).attr_unprot == 0 && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == 0 && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_indirect.overlay_index == 0 && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_direct.read == RMM_FALSE && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_direct.write == RMM_FALSE && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).addr == RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr))
  && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_VSMMU ==> (RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).attr_unprot == 0 && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == 0 && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_indirect.overlay_index == 0 && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_direct.read == RMM_FALSE && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_direct.write == RMM_FALSE && RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding).addr == RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr))
  && (result.is_Ok() && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED) ==> ripas == RipasToRmi(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas))
  && (result.is_Ok() && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED_NS || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_NS) ==> ripas == RMI_EMPTY)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) &&
       AddrIsRttLevelAligned(old_s, ipa, level as int) &&
       !((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat)))
    ==> result.is_Ok())
}

pub open spec fn rmi_rtt_set_ripas_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (base != RecAt(old_s, rec_ptr).ripas_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) > (RecAt(old_s, rec_ptr).ripas_top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RecAt(old_s, rec_ptr).ripas_value) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((base) == (RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,RecAt(old_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED)) && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RecAt(old_s, rec_ptr).ripas_value) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (AddrRangeIsAuxLive(old_s, base, top, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() ==> RttEntriesInRangeRipas(new_s,  RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,RecAt(new_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED),RecAt(new_s, rec_ptr).ripas_value))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).ripas_addr == MinAddress(new_s, top, RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,RecAt(new_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED)))
  && (result.is_Ok() ==> out_top == MinAddress(new_s, top, RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,RecAt(new_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED)))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != REC) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) &&
       !(RecAt(old_s, rec_ptr).owner != rd) &&
       !((top) <= (base)) &&
       !(base != RecAt(old_s, rec_ptr).ripas_addr) &&
       !((top) > (RecAt(old_s, rec_ptr).ripas_top)) &&
       !((!AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RecAt(old_s, rec_ptr).ripas_value)) &&
       AddrIsGranuleAligned(old_s, top) &&
       !(((base) == (RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,RecAt(old_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED)) && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RecAt(old_s, rec_ptr).ripas_value)) &&
       !(AddrRangeIsAuxLive(old_s, base, top, RealmAt(old_s, rd))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).ripas_addr == RecAt(old_s, rec_ptr).ripas_addr)
}

pub open spec fn rmi_rtt_set_s2ap_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, rtt_tree: UInt64, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (base != RecAt(old_s, rec_ptr).s2ap_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) > (RecAt(old_s, rec_ptr).s2ap_top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).valid == RMM_TRUE && !AddrRangeIsWithin(old_s, base, top,AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).walk.level),AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).walk.level)) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).index == RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).walk.rtte.s2ap_indirect.overlay_index!= RecAt(old_s, rec_ptr).s2ap_overlay_index) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalkAnyNotAligned(new_s, RealmAt(new_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).walk.level as int)))
  && ((RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).valid == RMM_TRUE && !AddrRangeIsWithin(old_s, base, top,AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).walk.level),AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).walk.level)) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).index != RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).walk.rtte.s2ap_indirect.overlay_index!= RecAt(old_s, rec_ptr).s2ap_overlay_index) ==> ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalkAnyNotAligned(new_s, RealmAt(new_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).walk.level as int)))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).s2ap_addr == out_top)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != REC) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) &&
       !(RecAt(old_s, rec_ptr).owner != rd) &&
       !((top) <= (base)) &&
       !(base != RecAt(old_s, rec_ptr).s2ap_addr) &&
       !((top) > (RecAt(old_s, rec_ptr).s2ap_top)) &&
       AddrIsGranuleAligned(old_s, top) &&
       !((RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).valid == RMM_TRUE && !AddrRangeIsWithin(old_s, base, top,AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).walk.level),AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).walk.level)) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).index == RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).walk.rtte.s2ap_indirect.overlay_index!= RecAt(old_s, rec_ptr).s2ap_overlay_index)) &&
       !((RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).valid == RMM_TRUE && !AddrRangeIsWithin(old_s, base, top,AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).walk.level),AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).walk.level)) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).index != RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top as int as usize,RMM_RTT_PAGE_LEVEL).walk.rtte.s2ap_indirect.overlay_index!= RecAt(old_s, rec_ptr).s2ap_overlay_index)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).s2ap_addr == RecAt(old_s, rec_ptr).s2ap_addr)
}

pub open spec fn rmi_rtt_unmap_unprotected_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttLevelIsBlockOrPage(old_s, RealmAt(old_s, rd), level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level < level ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_NS ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED_NS)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       RttLevelIsBlockOrPage(old_s, RealmAt(old_s, rd), level as int) &&
       AddrIsRttLevelAligned(old_s, ipa, level as int) &&
       !(((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)))) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level < level) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_NS))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
}

pub open spec fn rmi_vdev_abort_spec(vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).state != VDEV_COMMUNICATING ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).state == VDEV_READY)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_IDLE)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, vdev_ptr) &&
       PaIsDelegable(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != VDEV) &&
       !(VdevAt(old_s, vdev_ptr).state != VDEV_COMMUNICATING))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).state == VdevAt(old_s, vdev_ptr).state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
}

pub open spec fn rmi_vdev_aux_count_spec(pdev_flags: Bits64, vdev_flags: Bits64, result: Result<(), RmiStatusCode>, aux_count: UInt64, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (result.is_Ok() ==> aux_count == VdevAuxCount(new_s,  RmiPdevFlagsDecode(new_s, pdev_flags), RmiVdevFlagsDecode(new_s, vdev_flags)))
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE))
    ==> result.is_Ok())
}

pub open spec fn rmi_vdev_communicate_spec(pdev_ptr: Address, vdev_ptr: Address, data_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, data_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, data_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.req_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.req_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.resp_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.resp_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RmiDevCommDataAt(old_s, data_ptr).enter.resp_len > RMM_GRANULE_SIZE ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).pdev != pdev_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && ((VdevAt(old_s, vdev_ptr).state != VDEV_COMMUNICATING && VdevAt(old_s, vdev_ptr).state != VDEV_STOPPING) ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DeviceCommunicate2(new_s, VdevAt(new_s, vdev_ptr), RmiDevCommDataAt(new_s, data_ptr)))
  && (result.is_Ok() && (DeviceCommunicate2(old_s, VdevAt(old_s, vdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_ERROR && VdevAt(old_s, vdev_ptr).state == VDEV_COMMUNICATING) ==> VdevAt(new_s, vdev_ptr).state == VDEV_ERROR)
  && (result.is_Ok() && (DeviceCommunicate2(old_s, VdevAt(old_s, vdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_IDLE && VdevAt(old_s, vdev_ptr).state == VDEV_COMMUNICATING) ==> VdevAt(new_s, vdev_ptr).state == VDEV_READY)
  && (result.is_Ok() && (DeviceCommunicate2(old_s, VdevAt(old_s, vdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) != DEV_COMM_ACTIVE && VdevAt(old_s, vdev_ptr).state == VDEV_STOPPING) ==> VdevAt(new_s, vdev_ptr).state == VDEV_STOPPED)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       AddrIsGranuleAligned(old_s, vdev_ptr) &&
       PaIsDelegable(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != VDEV) &&
       AddrIsGranuleAligned(old_s, data_ptr) &&
       GranuleAccessPermitted(old_s, data_ptr, PAS_NS) &&
       AddrIsGranuleAligned(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.req_addr) &&
       GranuleAccessPermitted(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.req_addr, PAS_NS) &&
       AddrIsGranuleAligned(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.resp_addr) &&
       GranuleAccessPermitted(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.resp_addr, PAS_NS) &&
       !(RmiDevCommDataAt(old_s, data_ptr).enter.resp_len > RMM_GRANULE_SIZE) &&
       !(VdevAt(old_s, vdev_ptr).pdev != pdev_ptr) &&
       !((VdevAt(old_s, vdev_ptr).state != VDEV_COMMUNICATING && VdevAt(old_s, vdev_ptr).state != VDEV_STOPPING)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).state == VdevAt(old_s, vdev_ptr).state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).state == VdevAt(old_s, vdev_ptr).state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).state == VdevAt(old_s, vdev_ptr).state)
}

pub open spec fn rmi_vdev_complete_spec(rec_ptr: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).pending != REC_PENDING_VDEV_REQUEST ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).owner != VdevAt(old_s, vdev_ptr).realm ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).vdev_id != VdevAt(old_s, vdev_ptr).vdev_id ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RecAt(old_s, rec_ptr).inst_id_valid == RMM_TRUE && RecAt(old_s, rec_ptr).inst_id != VdevAt(old_s, vdev_ptr).inst_id) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).pending == REC_PENDING_NONE)
  && ((AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != REC) &&
       AddrIsGranuleAligned(old_s, vdev_ptr) &&
       PaIsDelegable(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != VDEV) &&
       !(RecAt(old_s, rec_ptr).pending != REC_PENDING_VDEV_REQUEST) &&
       !(RecAt(old_s, rec_ptr).owner != VdevAt(old_s, vdev_ptr).realm) &&
       !(RecAt(old_s, rec_ptr).vdev_id != VdevAt(old_s, vdev_ptr).vdev_id) &&
       !((RecAt(old_s, rec_ptr).inst_id_valid == RMM_TRUE && RecAt(old_s, rec_ptr).inst_id != VdevAt(old_s, vdev_ptr).inst_id)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).pending == RecAt(old_s, rec_ptr).pending)
}

pub open spec fn rmi_vdev_create_spec(rd: Address, pdev_ptr: Address, vdev_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_ptr).state != PDEV_READY ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDram(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RmiVdevParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (RmiVdevParamsAt(old_s, params_ptr).num_aux != VdevAuxCount(old_s, PdevFlags(old_s, PdevAt(old_s, pdev_ptr)),RmiVdevParamsAt(old_s, params_ptr).flags) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AuxAligned32(old_s, RmiVdevParamsAt(old_s, params_ptr).aux, RmiVdevParamsAt(old_s, params_ptr).num_aux as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AuxAlias32(old_s, vdev_ptr, RmiVdevParamsAt(old_s, params_ptr).aux, RmiVdevParamsAt(old_s, params_ptr).num_aux as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AuxStateEqual32(old_s, RmiVdevParamsAt(old_s, params_ptr).aux, RmiVdevParamsAt(old_s, params_ptr).num_aux as int, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!TdiIdIsFree(old_s, RmiVdevParamsAt(old_s, params_ptr).tdi_id, PdevAt(old_s, pdev_ptr).segment_id) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((RmiVdevParamsAt(old_s, params_ptr).tdi_id) < (PdevAt(old_s, pdev_ptr).rid_base) || (RmiVdevParamsAt(old_s, params_ptr).tdi_id) >= (PdevAt(old_s, pdev_ptr).rid_top)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !AddrIsGranuleAligned(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !PaIsDelegable(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && GranuleAt(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr).state != VSMMU) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !VsidIsFree(old_s, VsmmuAt(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr),RmiVdevParamsAt(old_s, params_ptr).vsid)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !PdevVsmmuIsCompatible(old_s, PdevAt(old_s, pdev_ptr),VsmmuAt(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr))) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).num_vdevs == PdevAt(new_s, pdev_ptr).num_vdevs + 1)
  && (result.is_Ok() ==> GranuleAt(new_s, vdev_ptr).state == VDEV)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).vdev_id == RmiVdevParamsAt(new_s, params_ptr).vdev_id)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).tdi_id == RmiVdevParamsAt(new_s, params_ptr).tdi_id)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).pdev == pdev_ptr)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).realm == rd)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).state == VDEV_READY)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_IDLE)
  && (result.is_Ok() ==> RdevFromInstId(new_s, RealmAt(new_s, rd),RealmAt(new_s, rd).vdev_count as int).state == RDEV_UNLOCKED)
  && (result.is_Ok() ==> RdevFromInstId(new_s, RealmAt(new_s, rd),RealmAt(new_s, rd).vdev_count as int).operation == RDEV_OP_NONE)
  && (result.is_Ok() ==> RdevFromInstId(new_s, RealmAt(new_s, rd),RealmAt(new_s, rd).vdev_count as int).vdev_ptr == vdev_ptr)
  && (result.is_Ok() ==> AuxEqual32(new_s, VdevAt(new_s, vdev_ptr).aux, RmiVdevParamsAt(new_s, params_ptr).aux, VdevAuxCount(new_s, PdevFlags(new_s, PdevAt(new_s, pdev_ptr)),RmiVdevParamsAt(new_s, params_ptr).flags)))
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).num_aux == VdevAuxCount(new_s, PdevFlags(new_s, PdevAt(new_s, pdev_ptr)),RmiVdevParamsAt(new_s, params_ptr).flags))
  && (result.is_Ok() ==> AuxStateEqual32(new_s, VdevAt(new_s, vdev_ptr).aux, VdevAuxCount(new_s, PdevFlags(new_s, PdevAt(new_s, pdev_ptr)),RmiVdevParamsAt(new_s, params_ptr).flags) as int, VDEV_AUX))
  && (result.is_Ok() ==> !TdiIdIsFree(new_s, RmiVdevParamsAt(new_s, params_ptr).tdi_id, PdevAt(new_s, pdev_ptr).segment_id))
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).inst_id == RealmAt(new_s, rd).vdev_count)
  && (result.is_Ok() ==> Equal(VdevAt(new_s, vdev_ptr).vsmmu, RmiVdevParamsAt(new_s, params_ptr).flags.VSMMU))
  && (result.is_Ok() && RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE ==> VdevAt(new_s, vdev_ptr).vsmmu_addr == RmiVdevParamsAt(new_s, params_ptr).vsmmu_addr)
  && (result.is_Ok() && RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE ==> VdevAt(new_s, vdev_ptr).vsid == RmiVdevParamsAt(new_s, params_ptr).vsid)
  && (result.is_Ok() && RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE ==> !VsidIsFree(new_s,  VsmmuAt(new_s, RmiVdevParamsAt(new_s, params_ptr).vsmmu_addr),RmiVdevParamsAt(new_s, params_ptr).vsid))
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vdevs == RealmAt(new_s, rd).num_vdevs + 1)
  && (result.is_Ok() ==> RealmAt(new_s, rd).vdev_count == RealmAt(new_s, rd).vdev_count + 1)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       !(PdevAt(old_s, pdev_ptr).state != PDEV_READY) &&
       AddrIsGranuleAligned(old_s, vdev_ptr) &&
       PaIsDelegableDram(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != DELEGATED) &&
       AddrIsGranuleAligned(old_s, params_ptr) &&
       GranuleAccessPermitted(old_s, params_ptr, PAS_NS) &&
       RmiVdevParamsIsValid(old_s, params_ptr) &&
       !(RealmAt(old_s, rd).feat_da != FEATURE_TRUE) &&
       !(RmiVdevParamsAt(old_s, params_ptr).num_aux != VdevAuxCount(old_s, PdevFlags(old_s, PdevAt(old_s, pdev_ptr)),RmiVdevParamsAt(old_s, params_ptr).flags)) &&
       AuxAligned32(old_s, RmiVdevParamsAt(old_s, params_ptr).aux, RmiVdevParamsAt(old_s, params_ptr).num_aux as int) &&
       !(AuxAlias32(old_s, vdev_ptr, RmiVdevParamsAt(old_s, params_ptr).aux, RmiVdevParamsAt(old_s, params_ptr).num_aux as int)) &&
       AuxStateEqual32(old_s, RmiVdevParamsAt(old_s, params_ptr).aux, RmiVdevParamsAt(old_s, params_ptr).num_aux as int, DELEGATED) &&
       TdiIdIsFree(old_s, RmiVdevParamsAt(old_s, params_ptr).tdi_id, PdevAt(old_s, pdev_ptr).segment_id) &&
       !(((RmiVdevParamsAt(old_s, params_ptr).tdi_id) < (PdevAt(old_s, pdev_ptr).rid_base) || (RmiVdevParamsAt(old_s, params_ptr).tdi_id) >= (PdevAt(old_s, pdev_ptr).rid_top))) &&
       !((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !AddrIsGranuleAligned(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr))) &&
       !((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !PaIsDelegable(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr))) &&
       !((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && GranuleAt(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr).state != VSMMU)) &&
       !((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !VsidIsFree(old_s, VsmmuAt(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr),RmiVdevParamsAt(old_s, params_ptr).vsid))) &&
       !((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !PdevVsmmuIsCompatible(old_s, PdevAt(old_s, pdev_ptr),VsmmuAt(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr)))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).num_vdevs == PdevAt(old_s, pdev_ptr).num_vdevs)
  && (result.is_Err()
    ==> GranuleAt(new_s, vdev_ptr).state == GranuleAt(old_s, vdev_ptr).state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_id == VdevAt(old_s, vdev_ptr).vdev_id)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).tdi_id == VdevAt(old_s, vdev_ptr).tdi_id)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).pdev == VdevAt(old_s, vdev_ptr).pdev)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).realm == VdevAt(old_s, vdev_ptr).realm)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).state == VdevAt(old_s, vdev_ptr).state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Err()
    ==> RdevFromInstId(new_s, RealmAt(new_s, rd),RealmAt(new_s, rd).vdev_count as int).state == RdevFromInstId(old_s, RealmAt(old_s, rd),RealmAt(old_s, rd).vdev_count as int).state)
  && (result.is_Err()
    ==> RdevFromInstId(new_s, RealmAt(new_s, rd),RealmAt(new_s, rd).vdev_count as int).operation == RdevFromInstId(old_s, RealmAt(old_s, rd),RealmAt(old_s, rd).vdev_count as int).operation)
  && (result.is_Err()
    ==> RdevFromInstId(new_s, RealmAt(new_s, rd),RealmAt(new_s, rd).vdev_count as int).vdev_ptr == RdevFromInstId(old_s, RealmAt(old_s, rd),RealmAt(old_s, rd).vdev_count as int).vdev_ptr)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).num_aux == VdevAt(old_s, vdev_ptr).num_aux)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).inst_id == VdevAt(old_s, vdev_ptr).inst_id)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vsmmu_addr == VdevAt(old_s, vdev_ptr).vsmmu_addr)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vsid == VdevAt(old_s, vdev_ptr).vsid)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_vdevs == RealmAt(old_s, rd).num_vdevs)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).vdev_count == RealmAt(old_s, rd).vdev_count)
  && (!(result.is_Ok() && (RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE)) ==> VdevAt(new_s, vdev_ptr).vsmmu_addr == VdevAt(old_s, vdev_ptr).vsmmu_addr)
}

pub open spec fn rmi_vdev_destroy_spec(rd: Address, pdev_ptr: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).realm != rd ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_ptr).pdev != pdev_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_ptr).state != VDEV_STOPPED ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> GranuleAt(new_s, vdev_ptr).state == DELEGATED)
  && (result.is_Ok() ==> AuxStateEqual32(new_s, VdevAt(new_s, vdev_ptr).aux, VdevAt(new_s, vdev_ptr).num_aux as int, DELEGATED))
  && (result.is_Ok() ==> TdiIdIsFree(new_s, VdevAt(new_s, vdev_ptr).tdi_id, PdevAt(new_s, pdev_ptr).segment_id))
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vdevs == RealmAt(new_s, rd).num_vdevs - 1)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).num_vdevs == PdevAt(new_s, pdev_ptr).num_vdevs - 1)
  && (result.is_Ok() && VdevAt(old_s, vdev_ptr).vsmmu == FEATURE_TRUE ==> VsidIsFree(new_s,  VsmmuAt(new_s, VdevAt(new_s, vdev_ptr).vsmmu_addr), VdevAt(new_s, vdev_ptr).vsid))
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       AddrIsGranuleAligned(old_s, vdev_ptr) &&
       PaIsDelegable(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != VDEV) &&
       !(VdevAt(old_s, vdev_ptr).realm != rd) &&
       !(VdevAt(old_s, vdev_ptr).pdev != pdev_ptr) &&
       !(VdevAt(old_s, vdev_ptr).state != VDEV_STOPPED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, vdev_ptr).state == GranuleAt(old_s, vdev_ptr).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_vdevs == RealmAt(old_s, rd).num_vdevs)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).num_vdevs == PdevAt(old_s, pdev_ptr).num_vdevs)
}

pub open spec fn rmi_vdev_get_state_spec(vdev_ptr: Address, result: Result<(), RmiStatusCode>, state: RmiVdevState, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> Equal(state, VdevAt(new_s, vdev_ptr).state))
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, vdev_ptr) &&
       PaIsDelegable(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != VDEV))
    ==> result.is_Ok())
}

pub open spec fn rmi_vdev_stop_spec(vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((VdevAt(old_s, vdev_ptr).state != VDEV_READY && VdevAt(old_s, vdev_ptr).state != VDEV_ERROR) ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).state == VDEV_STOPPING)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, vdev_ptr) &&
       PaIsDelegable(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != VDEV) &&
       !((VdevAt(old_s, vdev_ptr).state != VDEV_READY && VdevAt(old_s, vdev_ptr).state != VDEV_ERROR)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).state == VdevAt(old_s, vdev_ptr).state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
}

pub open spec fn rmi_version_spec(req: RmiInterfaceVersion, result: Result<(), RmiStatusCode>, lower: RmiInterfaceVersion, higher: RmiInterfaceVersion, old_s: S, new_s: S) -> bool {
  ((!RmiVersionIsSupported(old_s, req) && RmiVersionLowerIsSupported(old_s, req)) ==> (ResultEqual(result, RMI_ERROR_INPUT)  && VersionEqualRmi(lower, RmiVersionHighestBelow(new_s, req))  && VersionEqualRmi(higher, RmiVersionHighest(new_s))))
  && ((!RmiVersionIsSupported(old_s, req) && !RmiVersionLowerIsSupported(old_s, req) && RmiVersionHigherIsSupported(old_s, req)) ==> (ResultEqual(result, RMI_ERROR_INPUT)  && VersionEqualRmi(lower, higher)  && VersionEqualRmi(higher, RmiVersionHighest(new_s))))
  && (result.is_Ok() ==> VersionEqualRmi(lower, req))
  && (result.is_Ok() ==> VersionEqualRmi(higher, RmiVersionHighest(new_s)))
  && ((!((!RmiVersionIsSupported(old_s, req) && RmiVersionLowerIsSupported(old_s, req))) &&
       !((!RmiVersionIsSupported(old_s, req) && !RmiVersionLowerIsSupported(old_s, req) && RmiVersionHigherIsSupported(old_s, req))))
    ==> result.is_Ok())
}

pub open spec fn rmi_vsmmu_create_spec(rd: Address, vsmmu_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDram(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vsmmu_ptr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RmiVsmmuParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!AddrIsGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base) || !AddrIsGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base, RealmAt(old_s, rd)) || !AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top, RealmAt(old_s, rd)) || (RmiVsmmuParamsAt(old_s, params_ptr).reg_top) <= (RmiVsmmuParamsAt(old_s, params_ptr).reg_base)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> GranuleAt(new_s, vsmmu_ptr).state == VSMMU)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).realm == rd)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).reg_base == RmiVsmmuParamsAt(new_s, params_ptr).reg_base)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).reg_top == RmiVsmmuParamsAt(new_s, params_ptr).reg_top)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).aidr == RmiVsmmuParamsAt(new_s, params_ptr).aidr)
  && (result.is_Ok() ==> (VsmmuAt(new_s, vsmmu_ptr).idr[0] == RmiVsmmuParamsAt(new_s, params_ptr).idr[0] && VsmmuAt(new_s, vsmmu_ptr).idr[1] == RmiVsmmuParamsAt(new_s, params_ptr).idr[1] && VsmmuAt(new_s, vsmmu_ptr).idr[2] == RmiVsmmuParamsAt(new_s, params_ptr).idr[2] && VsmmuAt(new_s, vsmmu_ptr).idr[3] == RmiVsmmuParamsAt(new_s, params_ptr).idr[3] && VsmmuAt(new_s, vsmmu_ptr).idr[4] == RmiVsmmuParamsAt(new_s, params_ptr).idr[4] && VsmmuAt(new_s, vsmmu_ptr).idr[5] == RmiVsmmuParamsAt(new_s, params_ptr).idr[5] && VsmmuAt(new_s, vsmmu_ptr).idr[6] == RmiVsmmuParamsAt(new_s, params_ptr).idr[6]))
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       AddrIsGranuleAligned(old_s, vsmmu_ptr) &&
       PaIsDelegableDram(old_s, vsmmu_ptr) &&
       !(GranuleAt(old_s, vsmmu_ptr).state != DELEGATED) &&
       AddrIsGranuleAligned(old_s, params_ptr) &&
       GranuleAccessPermitted(old_s, params_ptr, PAS_NS) &&
       RmiVsmmuParamsIsValid(old_s, params_ptr) &&
       !((!AddrIsGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base) || !AddrIsGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top))) &&
       !((!AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base, RealmAt(old_s, rd)) || !AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top, RealmAt(old_s, rd)) || (RmiVsmmuParamsAt(old_s, params_ptr).reg_top) <= (RmiVsmmuParamsAt(old_s, params_ptr).reg_base))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, vsmmu_ptr).state == GranuleAt(old_s, vsmmu_ptr).state)
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).realm == VsmmuAt(old_s, vsmmu_ptr).realm)
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).reg_base == VsmmuAt(old_s, vsmmu_ptr).reg_base)
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).reg_top == VsmmuAt(old_s, vsmmu_ptr).reg_top)
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).aidr == VsmmuAt(old_s, vsmmu_ptr).aidr)
}

pub open spec fn rmi_vsmmu_destroy_spec(rd: Address, vsmmu_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vsmmu_ptr).state != VSMMU ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VsmmuIsLive(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> GranuleAt(new_s, vsmmu_ptr).state == DELEGATED)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, vsmmu_ptr) &&
       PaIsDelegable(old_s, vsmmu_ptr) &&
       !(GranuleAt(old_s, vsmmu_ptr).state != VSMMU) &&
       !(VsmmuIsLive(old_s, vsmmu_ptr)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, vsmmu_ptr).state == GranuleAt(old_s, vsmmu_ptr).state)
}

pub open spec fn rmi_vsmmu_map_spec(rd: Address, vsmmu_ptr: Address, ipa: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vsmmu_ptr).state != VSMMU ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) < (VsmmuAt(old_s, vsmmu_ptr).reg_base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != EMPTY ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (((ipa) + (RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) - 1)>= (VsmmuAt(old_s, vsmmu_ptr).reg_top)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_VSMMU)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == vsmmu_ptr)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       AddrIsGranuleAligned(old_s, vsmmu_ptr) &&
       PaIsDelegable(old_s, vsmmu_ptr) &&
       !(GranuleAt(old_s, vsmmu_ptr).state != VSMMU) &&
       !((ipa) < (VsmmuAt(old_s, vsmmu_ptr).reg_base)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != EMPTY) &&
       !(((ipa) + (RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) - 1)>= (VsmmuAt(old_s, vsmmu_ptr).reg_top))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
}

pub open spec fn rmi_vsmmu_unmap_spec(rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, vsmmu: Address, top: Address, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))))
  && (AddrIsAuxLive(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT_AUX(0 as int)))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED)
  && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == DEV ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == DESTROYED)
  && (result.is_Ok() ==> vsmmu == RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,ipa))
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU) &&
       !(AddrIsAuxLive(old_s, ipa, RealmAt(old_s, rd))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
  && (!(result.is_Ok() && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == DEV)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
}

pub open spec fn rsi_attestation_token_continue_spec(addr: Address, offset: UInt64, size: UInt64, result: RsiCommandReturnCode, len: UInt64, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
  && (offset >= RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
  && (offset + size < offset ==> result == RSI_ERROR_INPUT)
  && (offset + size > RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
  && (CurrentRec(old_s).attest_state != ATTEST_IN_PROGRESS ==> result == RSI_ERROR_STATE)
  && ((AddrIsGranuleAligned(old_s, addr) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY) &&
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
  (result == RSI_SUCCESS ==> value == RsiFeatureRegisterEncode(new_s, CurrentRealm(new_s), index as int))
}

pub open spec fn rsi_host_call_spec(addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (!AddrIsAligned(old_s, addr, 256) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
  && ((AddrIsAligned(old_s, addr, 256) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY))
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
  && (result == RSI_SUCCESS ==> response == RecRipasResponseToRsi(new_s, CurrentRec(new_s)))
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
  && (result == RSI_SUCCESS ==> CurrentRealm(new_s).measurements[index] == RemExtend(new_s, CurrentRealm(new_s).hash_algo, CurrentRealm(new_s).measurements[index], [value_0, value_1, value_2, value_3,value_4, value_5, value_6, value_7][ (RMM_REALM_MEASUREMENT_WIDTH-1):0],size * 8))
  && ((!(index < 1 || index > 4) &&
       !(size > 64))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> CurrentRealm(new_s).measurements[index] == CurrentRealm(old_s).measurements[index])
}

pub open spec fn rsi_measurement_read_spec(index: UInt64, result: RsiCommandReturnCode, value_0: Bits64, value_1: Bits64, value_2: Bits64, value_3: Bits64, value_4: Bits64, value_5: Bits64, value_6: Bits64, value_7: Bits64, old_s: S, new_s: S) -> bool {
  (index > 4 ==> result == RSI_ERROR_INPUT)
  && ((!(index > 4))
    ==> result == RSI_SUCCESS)
}

pub open spec fn rsi_mem_get_perm_value_spec(plane_index: UInt64, perm_index: UInt64, result: RsiCommandReturnCode, value: Bits64, old_s: S, new_s: S) -> bool {
  (plane_index > CurrentRealm(old_s).num_aux_planes ==> result == RSI_ERROR_INPUT)
  && (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> value == CurrentRealm(new_s).overlay_perms[plane_index as int].values[perm_index as int])
  && ((!(plane_index > CurrentRealm(old_s).num_aux_planes) &&
       !(perm_index >= RMM_NUM_PERM_OVERLAY_INDICES))
    ==> result == RSI_SUCCESS)
}

pub open spec fn rsi_mem_set_perm_index_spec(base: Address, top: Address, perm_index: UInt64, cookie: Bits64, result: RsiCommandReturnCode, new_base: Address, response: RsiResponse, new_cookie: Bits64, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT)
  && ((top) <= (base) ==> result == RSI_ERROR_INPUT)
  && (!AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT)
  && (Cookie is invalid ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> CurrentRealm(new_s).overlay_locked[perm_index as int] == MEM_PERM_LOCKED)
  && (result == RSI_SUCCESS ==> new_base == CurrentRec(new_s).s2ap_addr)
  && (result == RSI_SUCCESS ==> response == RecS2APResponseToRsi(new_s, CurrentRec(new_s)))
  && ((AddrIsGranuleAligned(old_s, base) &&
       AddrIsGranuleAligned(old_s, top) &&
       !((top) <= (base)) &&
       AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) &&
       !(perm_index >= RMM_NUM_PERM_OVERLAY_INDICES) &&
       !(Cookie is invalid))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> CurrentRealm(new_s).overlay_locked[perm_index as int] == CurrentRealm(old_s).overlay_locked[perm_index as int])
}

pub open spec fn rsi_mem_set_perm_value_spec(plane_index: UInt64, perm_index: UInt64, value: Bits64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  ((plane_index == 0 || plane_index > CurrentRealm(old_s).num_aux_planes) ==> result == RSI_ERROR_INPUT)
  && (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT)
  && (CurrentRealm(old_s).overlay_locked[perm_index as int] == MEM_PERM_LOCKED ==> result == RSI_ERROR_INPUT)
  && (!MemPermLabelSupported(old_s, value) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> CurrentRealm(new_s).overlay_perms[plane_index as int].values[perm_index as int] == value)
  && ((!((plane_index == 0 || plane_index > CurrentRealm(old_s).num_aux_planes)) &&
       !(perm_index >= RMM_NUM_PERM_OVERLAY_INDICES) &&
       !(CurrentRealm(old_s).overlay_locked[perm_index as int] == MEM_PERM_LOCKED) &&
       MemPermLabelSupported(old_s, value))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> CurrentRealm(new_s).overlay_perms[plane_index as int].values[perm_index as int] == CurrentRealm(old_s).overlay_perms[plane_index as int].values[perm_index as int])
}

pub open spec fn rsi_plane_enter_spec(plane_idx: UInt64, run_ptr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  ((plane_idx == 0 || plane_idx > CurrentRealm(old_s).num_aux_planes) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, run_ptr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, run_ptr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), run_ptr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
  && ((!((plane_idx == 0 || plane_idx > CurrentRealm(old_s).num_aux_planes)) &&
       AddrIsGranuleAligned(old_s, run_ptr) &&
       AddrIsProtected(old_s, run_ptr, CurrentRealm(old_s)) &&
       !(RttWalk(old_s, CurrentRealm(old_s), run_ptr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY))
    ==> result == RSI_SUCCESS)
}

pub open spec fn rsi_plane_reg_read_spec(plane_idx: UInt64, encoding: Bits64, result: RsiCommandReturnCode, value: Bits64, old_s: S, new_s: S) -> bool {
  (plane_idx > CurrentRealm(old_s).num_aux_planes ==> result == RSI_ERROR_INPUT)
  && (!PlaneRegIsValid(old_s, CurrentRealm(old_s), encoding) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> value == PlaneRegValue(new_s, CurrentRealm(new_s), plane_idx as int, encoding))
  && ((!(plane_idx > CurrentRealm(old_s).num_aux_planes) &&
       PlaneRegIsValid(old_s, CurrentRealm(old_s), encoding))
    ==> result == RSI_SUCCESS)
}

pub open spec fn rsi_plane_reg_write_spec(plane_idx: UInt64, encoding: Bits64, value: Bits64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (plane_idx > CurrentRealm(old_s).num_aux_planes ==> result == RSI_ERROR_INPUT)
  && (!PlaneRegIsValid(old_s, CurrentRealm(old_s), encoding) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> PlaneRegValue(new_s, CurrentRealm(new_s), plane_idx as int, encoding) == value)
  && ((!(plane_idx > CurrentRealm(old_s).num_aux_planes) &&
       PlaneRegIsValid(old_s, CurrentRealm(old_s), encoding))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> PlaneRegValue(new_s, CurrentRealm(new_s), plane_idx as int, encoding) == PlaneRegValue(old_s, CurrentRealm(old_s), plane_idx as int, encoding))
}

pub open spec fn rsi_rdev_continue_spec(vdev_id: Bits64, inst_id: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) ==> result == RSI_ERROR_INPUT)
  && ((RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_UNLOCKED_BUSY && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_LOCKED_BUSY && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STARTED_BUSY && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STOPPING) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS && (DeviceCommunicate1(old_s, RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int)) == DEV_COMM_ERROR && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STOPPING) ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_ERROR)
  && (result == RSI_SUCCESS && (DeviceCommunicate1(old_s, RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int)) == DEV_COMM_IDLE && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state == RDEV_UNLOCKED_BUSY && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).operation != RDEV_OP_LOCK) ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_UNLOCKED)
  && (result == RSI_SUCCESS && (DeviceCommunicate1(old_s, RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int)) == DEV_COMM_IDLE && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state == RDEV_UNLOCKED_BUSY && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).operation == RDEV_OP_LOCK) ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_LOCKED)
  && (result == RSI_SUCCESS && (DeviceCommunicate1(old_s, RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int)) == DEV_COMM_IDLE && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state == RDEV_LOCKED_BUSY && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).operation != RDEV_OP_LOCK) ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_LOCKED)
  && (result == RSI_SUCCESS && (DeviceCommunicate1(old_s, RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int)) == DEV_COMM_IDLE && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state == RDEV_LOCKED_BUSY && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).operation == RDEV_OP_START) ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_STARTED)
  && (result == RSI_SUCCESS && (DeviceCommunicate1(old_s, RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int)) == DEV_COMM_IDLE && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state == RDEV_STARTED_BUSY) ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_STARTED)
  && (result == RSI_SUCCESS && (DeviceCommunicate1(old_s, RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int)) != DEV_COMM_ACTIVE && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state == RDEV_STOPPING) ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_STOPPED)
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) &&
       !((RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_UNLOCKED_BUSY && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_LOCKED_BUSY && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STARTED_BUSY && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STOPPING)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
}

pub open spec fn rsi_rdev_get_info_spec(vdev_id: Bits64, inst_id: UInt64, addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> Equal(RsiDeviceInfoAt(new_s, addr).hash_algo, PdevAt(new_s, VdevAt(new_s, RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).vdev_ptr).pdev).hash_algo))
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) &&
       AddrIsGranuleAligned(old_s, addr) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY))
    ==> result == RSI_SUCCESS)
}

pub open spec fn rsi_rdev_get_instance_id_spec(vdev_id: Bits64, result: RsiCommandReturnCode, inst_id: UInt64, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!RdevIdIsValid(old_s, CurrentRealm(old_s), vdev_id) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> inst_id == VdevAt(new_s, RdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).vdev_ptr).inst_id)
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       RdevIdIsValid(old_s, CurrentRealm(old_s), vdev_id))
    ==> result == RSI_SUCCESS)
}

pub open spec fn rsi_rdev_get_interface_report_spec(vdev_id: Bits64, inst_id: UInt64, version_max: UInt64, result: RsiCommandReturnCode, version: UInt64, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) ==> result == RSI_ERROR_INPUT)
  && ((RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_LOCKED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STARTED) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state == RDEV_LOCKED ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_LOCKED_BUSY)
  && (result == RSI_SUCCESS && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state == RDEV_STARTED ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_STARTED_BUSY)
  && (result == RSI_SUCCESS ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).operation == RDEV_OP_GET_INTERFACE_REPORT)
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) &&
       !((RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_LOCKED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STARTED)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).operation == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).operation)
  && (!(result == RSI_SUCCESS && (RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state == RDEV_LOCKED)) ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
}

pub open spec fn rsi_rdev_get_measurements_spec(vdev_id: Bits64, inst_id: UInt64, op: Bits64, flags: Bits64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) ==> result == RSI_ERROR_INPUT)
  && ((RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_UNLOCKED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_LOCKED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STARTED) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state == RDEV_UNLOCKED ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_UNLOCKED_BUSY)
  && (result == RSI_SUCCESS && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state == RDEV_LOCKED ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_LOCKED_BUSY)
  && (result == RSI_SUCCESS && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state == RDEV_STARTED ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_STARTED_BUSY)
  && (result == RSI_SUCCESS ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).operation == RDEV_OP_GET_MEASUREMENTS)
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) &&
       !((RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_UNLOCKED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_LOCKED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STARTED)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).operation == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).operation)
  && (!(result == RSI_SUCCESS && (RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state == RDEV_UNLOCKED)) ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
}

pub open spec fn rsi_rdev_get_state_spec(vdev_id: Bits64, inst_id: UInt64, result: RsiCommandReturnCode, state: RsiDeviceState, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> Equal(state, RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state))
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int))
    ==> result == RSI_SUCCESS)
}

pub open spec fn rsi_rdev_lock_spec(vdev_id: Bits64, inst_id: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) ==> result == RSI_ERROR_INPUT)
  && (RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_UNLOCKED ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_UNLOCKED_BUSY)
  && (result == RSI_SUCCESS ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).operation == RDEV_OP_LOCK)
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) &&
       !(RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_UNLOCKED))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).operation == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).operation)
}

pub open spec fn rsi_rdev_start_spec(vdev_id: Bits64, inst_id: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) ==> result == RSI_ERROR_INPUT)
  && (RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_LOCKED ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_LOCKED_BUSY)
  && (result == RSI_SUCCESS ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).operation == RDEV_OP_START)
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) &&
       !(RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_LOCKED))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).operation == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).operation)
}

pub open spec fn rsi_rdev_stop_spec(vdev_id: Bits64, inst_id: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) ==> result == RSI_ERROR_INPUT)
  && ((RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_UNLOCKED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_LOCKED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STARTED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_ERROR) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_STOPPING)
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) &&
       !((RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_UNLOCKED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_LOCKED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STARTED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_ERROR)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
}

pub open spec fn rsi_rdev_validate_mapping_spec(vdev_id: Bits64, inst_id: UInt64, ipa_base: Address, ipa_top: Address, pa_base: Address, flags: RsiDevMemFlags, result: RsiCommandReturnCode, new_ipa_base: Address, response: RsiResponse, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) ==> result == RSI_ERROR_INPUT)
  && ((RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_LOCKED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STARTED) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, ipa_base) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, ipa_top) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, pa_base) ==> result == RSI_ERROR_INPUT)
  && ((ipa_top) <= (ipa_base) ==> result == RSI_ERROR_INPUT)
  && (!AddrRangeIsProtected(old_s, ipa_base, ipa_top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> new_ipa_base == CurrentRec(new_s).dev_mem_addr)
  && (result == RSI_SUCCESS ==> response == RecDevMemResponseToRsi(new_s, CurrentRec(new_s)))
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) &&
       !((RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_LOCKED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STARTED)) &&
       AddrIsGranuleAligned(old_s, ipa_base) &&
       AddrIsGranuleAligned(old_s, ipa_top) &&
       AddrIsGranuleAligned(old_s, pa_base) &&
       !((ipa_top) <= (ipa_base)) &&
       AddrRangeIsProtected(old_s, ipa_base, ipa_top, CurrentRealm(old_s)))
    ==> result == RSI_SUCCESS)
}

pub open spec fn rsi_realm_config_spec(addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> RsiRealmConfigAt(new_s, addr).ipa_width == CurrentRealm(new_s).ipa_width)
  && (result == RSI_SUCCESS ==> Equal(RsiRealmConfigAt(new_s, addr).hash_algo, CurrentRealm(new_s).hash_algo))
  && (result == RSI_SUCCESS ==> RsiRealmConfigAt(new_s, addr).num_aux_planes == CurrentRealm(new_s).num_aux_planes)
  && ((AddrIsGranuleAligned(old_s, addr) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RsiRealmConfigAt(new_s, addr).ipa_width == RsiRealmConfigAt(old_s, addr).ipa_width)
  && (result != RSI_SUCCESS
    ==> RsiRealmConfigAt(new_s, addr).num_aux_planes == RsiRealmConfigAt(old_s, addr).num_aux_planes)
}

pub open spec fn rsi_version_spec(req: RsiInterfaceVersion, result: RsiCommandReturnCode, lower: RsiInterfaceVersion, higher: RsiInterfaceVersion, old_s: S, new_s: S) -> bool {
  ((!RsiVersionIsSupported(old_s, req) && RsiVersionLowerIsSupported(old_s, req)) ==> (result == RSI_ERROR_INPUT && VersionEqualRsi(lower, RsiVersionHighestBelow(new_s, req)) && VersionEqualRsi(higher, RsiVersionHighest(new_s))))
  && ((!RsiVersionIsSupported(old_s, req) && !RsiVersionLowerIsSupported(old_s, req) && RsiVersionHigherIsSupported(old_s, req)) ==> (result == RSI_ERROR_INPUT && VersionEqualRsi(lower, higher) && VersionEqualRsi(higher, RsiVersionHighest(new_s))))
  && (result == RSI_SUCCESS ==> VersionEqualRsi(lower, req))
  && (result == RSI_SUCCESS ==> VersionEqualRsi(higher, RsiVersionHighest(new_s)))
  && ((!((!RsiVersionIsSupported(old_s, req) && RsiVersionLowerIsSupported(old_s, req))) &&
       !((!RsiVersionIsSupported(old_s, req) && !RsiVersionLowerIsSupported(old_s, req) && RsiVersionHigherIsSupported(old_s, req))))
    ==> result == RSI_SUCCESS)
}

pub open spec fn rsi_vsmmu_activate_spec(base: Address, top: Address, result: RsiCommandReturnCode, new_base: Address, old_s: S, new_s: S) -> bool {
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

pub open spec fn rsi_vsmmu_get_info_spec(addr: Address, result: RsiCommandReturnCode, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)
  && (addr != VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_base ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> top == VsmmuAt(new_s, RttWalk(new_s, CurrentRealm(new_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_top)
  && ((AddrIsGranuleAligned(old_s, addr) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU) &&
       !(addr != VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_base))
    ==> result == RSI_SUCCESS)
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
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (true));
  assert(result.is_Ok() ==> (old_walk.rtte.state == UNASSIGNED));
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == RAM));
  assert(result.is_Ok() ==> (new_walk.rtte.state == ASSIGNED));
}

pub proof fn rmi_data_create_unknown_rule (rd: Address, data: Address, ipa: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S)
    requires rmi_data_create_unknown_spec(rd, data, ipa, result, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (true));
  assert(result.is_Ok() ==> (old_walk.rtte.state == UNASSIGNED));
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert(result.is_Ok() ==> (new_walk.rtte.state == ASSIGNED));
}

pub proof fn rmi_data_destroy_rule (rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, data: Address, top: Address, old_s: S, new_s: S)
    requires rmi_data_destroy_spec(rd, ipa, result, data, top, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (old_walk.rtte.ripas != RAM || old_walk.rtte.ripas == RAM));
  assert(result.is_Ok() ==> (old_walk.rtte.state == ASSIGNED));
  assert((result.is_Ok() && old_walk.rtte.ripas != RAM) ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert((result.is_Ok() && old_walk.rtte.ripas == RAM) ==> (new_walk.rtte.ripas == DESTROYED));
  assert(result.is_Ok() ==> (new_walk.rtte.state == UNASSIGNED));
}

pub proof fn rmi_rtt_create_rule (rd: Address, rtt: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S)
    requires rmi_rtt_create_spec(rd, rtt, ipa, level, result, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (true));
  assert(result.is_Ok() ==> (true));
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert(result.is_Ok() ==> (new_walk.rtte.state == old_walk.rtte.state));
}

pub proof fn rmi_rtt_destroy_rule (rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, top: Address, old_s: S, new_s: S)
    requires rmi_rtt_destroy_spec(rd, ipa, level, result, rtt, top, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (true));
  // Unsupported
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == DESTROYED));
  assert(result.is_Ok() ==> (new_walk.rtte.state == UNASSIGNED));
}

pub proof fn rmi_rtt_dev_mem_validate_rule (rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S)
    requires rmi_rtt_dev_mem_validate_spec(rd, rec_ptr, base, top, result, out_top, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (true));
  // Unsupported
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == DEV));
  assert(result.is_Ok() ==> (new_walk.rtte.state == old_walk.rtte.state));
}

pub proof fn rmi_rtt_fold_rule (rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S)
    requires rmi_rtt_fold_spec(rd, ipa, level, result, rtt, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int);
  // Unsupported
  // Unsupported
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert(result.is_Ok() ==> (new_walk.rtte.state == old_walk.rtte.state));
}

pub proof fn rmi_rtt_init_ripas_rule (rd: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S)
    requires rmi_rtt_init_ripas_spec(rd, base, top, result, out_top, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (true));
  assert(result.is_Ok() ==> (old_walk.rtte.state == UNASSIGNED));
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == RAM));
  assert(result.is_Ok() ==> (new_walk.rtte.state == old_walk.rtte.state));
}

pub proof fn rmi_rtt_set_ripas_rule (rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S)
    requires rmi_rtt_set_ripas_spec(rd, rec_ptr, base, top, result, out_top, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  // Unsupported
  assert(result.is_Ok() ==> (true));
  // Unsupported
  assert(result.is_Ok() ==> (new_walk.rtte.state == old_walk.rtte.state));
}

pub proof fn rmi_dev_mem_map_rule (rd: Address, ipa: Address, level: Int64, addr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S)
    requires rmi_dev_mem_map_spec(rd, ipa, level, addr, result, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (true));
  assert(result.is_Ok() ==> (old_walk.rtte.state == UNASSIGNED));
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert(result.is_Ok() ==> (new_walk.rtte.state == ASSIGNED_DEV));
}

pub proof fn rmi_dev_mem_unmap_rule (rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, pa: Address, top: Address, old_s: S, new_s: S)
    requires rmi_dev_mem_unmap_spec(rd, ipa, level, result, pa, top, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (old_walk.rtte.ripas != DEV || old_walk.rtte.ripas == DEV));
  assert(result.is_Ok() ==> (old_walk.rtte.state == ASSIGNED_DEV));
  assert((result.is_Ok() && old_walk.rtte.ripas != DEV) ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert((result.is_Ok() && old_walk.rtte.ripas == DEV) ==> (new_walk.rtte.ripas == DESTROYED));
  assert(result.is_Ok() ==> (new_walk.rtte.state == UNASSIGNED));
}

pub proof fn rmi_vsmmu_map_rule (rd: Address, vsmmu_ptr: Address, ipa: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S)
    requires rmi_vsmmu_map_spec(rd, vsmmu_ptr, ipa, result, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (old_walk.rtte.ripas == EMPTY));
  assert(result.is_Ok() ==> (old_walk.rtte.state == UNASSIGNED));
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert(result.is_Ok() ==> (new_walk.rtte.state == ASSIGNED_VSMMU));
}

pub proof fn rmi_vsmmu_unmap_rule (rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, vsmmu: Address, top: Address, old_s: S, new_s: S)
    requires rmi_vsmmu_unmap_spec(rd, ipa, result, vsmmu, top, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (old_walk.rtte.ripas != DEV || old_walk.rtte.ripas == DEV));
  assert(result.is_Ok() ==> (old_walk.rtte.state == ASSIGNED_VSMMU));
  assert((result.is_Ok() && old_walk.rtte.ripas != DEV) ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert((result.is_Ok() && old_walk.rtte.ripas == DEV) ==> (new_walk.rtte.ripas == DESTROYED));
  assert(result.is_Ok() ==> (new_walk.rtte.state == UNASSIGNED));
}

fn main() {
}

}
