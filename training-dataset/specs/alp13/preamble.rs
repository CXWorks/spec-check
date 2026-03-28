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
use crate::RmiDevCommObject::*;
use crate::RmiDevCommProtocol::*;
use crate::RmiDevCommStatus::*;
use crate::RmiEmulatedMmio::*;
use crate::RmiFeature::*;
use crate::RmiForceP0::*;
use crate::RmiHashAlgorithm::*;
use crate::RmiInjectSea::*;
use crate::RmiLfaPolicy::*;
use crate::RmiPdevCoherent::*;
use crate::RmiPdevEvent::*;
use crate::RmiPdevIde::*;
use crate::RmiPdevSpdm::*;
use crate::RmiPdevState::*;
use crate::RmiPmuOverflowStatus::*;
use crate::RmiRecExitReason::*;
use crate::RmiRecRunnable::*;
use crate::RmiResponse::*;
use crate::RmiRipas::*;
use crate::RmiRttEntryState::*;
use crate::RmiRttPlaneFeature::*;
use crate::RmiRttS2APBase::*;
use crate::RmiRttS2APEncoding::*;
use crate::RmiSignatureAlgorithm::*;
use crate::RmiSmmuAction::*;
use crate::RmiSmmuIrq::*;
use crate::RmiStatusCode::*;
use crate::RmiTrap::*;
use crate::RmiVdevMeasureAll::*;
use crate::RmiVdevMeasureRaw::*;
use crate::RmiVdevMeasureSigned::*;
use crate::RmiVdevState::*;
use crate::RsiBoolean::*;
use crate::RsiCommandReturnCode::*;
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
use crate::RsiVdevAttestType::*;
use crate::RsiVdevState::*;
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
use crate::RmmPdevCoherent::*;
use crate::RmmPdevIde::*;
use crate::RmmPdevSpdm::*;
use crate::RmmPdevState::*;
use crate::RmmPhysicalAddressSpace::*;
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
use crate::RmmRttS2APBase::*;
use crate::RmmRttS2APEncoding::*;
use crate::RmmRttShareability::*;
use crate::RmmVdevDmaState::*;
use crate::RmmVdevOperation::*;
use crate::RmmVdevState::*;
use crate::RmmVsmmuState::*;
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

pub enum RmiDevCommObject {
  RMI_DEV_VCA,
  RMI_DEV_CERTIFICATE,
  RMI_DEV_MEASUREMENTS,
  RMI_DEV_INTERFACE_REPORT,
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

pub enum RmiForceP0 {
  RMI_NO_FORCE_P0,
  RMI_FORCE_P0,
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

pub enum RmiPdevCoherent {
  RMI_NCOH,
  RMI_COH,
}

pub enum RmiPdevEvent {
  RMI_IDE_KEY_REFRESH,
}

pub enum RmiPdevIde {
  RMI_IDE_FALSE,
  RMI_IDE_TRUE,
}

pub enum RmiPdevSpdm {
  RMI_SPDM_FALSE,
  RMI_SPDM_TRUE,
}

pub enum RmiPdevState {
  RMI_PDEV_NEW,
  RMI_PDEV_NEEDS_KEY,
  RMI_PDEV_HAS_KEY,
  RMI_PDEV_READY,
  RMI_PDEV_IDE_RESETTING,
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

pub enum RmiRttS2APBase {
  RMI_S2AP_NO_ACCESS,
  RMI_S2AP_RO,
  RMI_S2AP_WO,
  RMI_S2AP_RW,
  RMI_S2AP_RW_PUX,
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

pub enum RmiVdevMeasureAll {
  RMI_VDEV_MEASURE_NOT_ALL,
  RMI_VDEV_MEASURE_ALL,
}

pub enum RmiVdevMeasureRaw {
  RMI_VDEV_MEASURE_NOT_RAW,
  RMI_VDEV_MEASURE_RAW,
}

pub enum RmiVdevMeasureSigned {
  RMI_VDEV_MEASURE_NOT_SIGNED,
  RMI_VDEV_MEASURE_SIGNED,
}

pub enum RmiVdevState {
  RMI_VDEV_NEW,
  RMI_VDEV_UNLOCKED,
  RMI_VDEV_LOCKED,
  RMI_VDEV_STARTED,
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
  RSI_EXIT_IRQ,
  RSI_EXIT_HOST,
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

pub enum RsiVdevAttestType {
  RSI_INDEPENDENTLY_ATTESTED,
  RSI_PLATFORM_ATTESTED,
}

pub enum RsiVdevState {
  RSI_VDEV_NEW,
  RSI_VDEV_UNLOCKED,
  RSI_VDEV_LOCKED,
  RSI_VDEV_STARTED,
  RSI_VDEV_ERROR,
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
  P2P_STREAM,
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

pub enum RmmPdevCoherent {
  COH,
  NCOH,
}

pub enum RmmPdevIde {
  IDE_FALSE,
  IDE_TRUE,
}

pub enum RmmPdevSpdm {
  SPDM_FALSE,
  SPDM_TRUE,
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
  REC_PENDING_VDEV_COMPLETE,
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

pub enum RmmRttS2APBase {
  S2AP_NO_ACCESS,
  S2AP_RO,
  S2AP_RW,
  S2AP_RW_PUX,
  S2AP_WO,
}

pub enum RmmRttS2APEncoding {
  S2AP_DIRECT,
  S2AP_INDIRECT,
}

pub enum RmmRttShareability {
  SHAREABILITY_INNER,
  SHAREABILITY_OUTER,
}

pub enum RmmVdevDmaState {
  VDEV_DMA_DISABLED,
  VDEV_DMA_ENABLED,
}

pub enum RmmVdevOperation {
  VDEV_OP_GET_MEAS,
  VDEV_OP_GET_REPORT,
  VDEV_OP_LOCK,
  VDEV_OP_NONE,
  VDEV_OP_P2P_BIND,
  VDEV_OP_START,
  VDEV_OP_UNLOCK,
}

pub enum RmmVdevState {
  VDEV_ERROR,
  VDEV_LOCKED,
  VDEV_NEW,
  VDEV_STARTED,
  VDEV_UNLOCKED,
}

pub enum RmmVsmmuState {
  VSMMU_ACTIVATING,
  VSMMU_ACTIVE,
  VSMMU_INACTIVE,
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
  pub cache_req_offset: UInt64,
  pub cache_req_len: UInt64,
  pub cache_rsp_offset: UInt64,
  pub cache_rsp_len: UInt64,
  pub cache_obj_id: RmiDevCommObject,
  pub protocol: RmiDevCommProtocol,
  pub req_len: UInt64,
  pub timeout: UInt64,
}

struct RmiDevCommExitFlags {
  pub cache_req: RmiBoolean,
  pub cache_rsp: RmiBoolean,
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
  pub spdm: RmiPdevSpdm,
  pub ncoh_ide: RmiPdevIde,
  pub ncoh_addr: RmiFeature,
  pub coh_ide: RmiPdevIde,
  pub coh_addr: RmiFeature,
  pub p2p: RmiFeature,
}

struct RmiPdevParams {
  pub flags: RmiPdevFlags,
  pub pdev_id: UInt64,
  pub segment_id: UInt8,
  pub ecam_addr: Address,
  pub root_id: UInt16,
  pub cert_id: UInt64,
  pub rid_base: UInt16,
  pub rid_top: UInt16,
  pub hash_algo: RmiHashAlgorithm,
  pub num_aux: UInt64,
  pub ncoh_ide_sid: UInt64,
  pub aux: [Address; 32],
  pub coh_addr_range: [RmiAddressRange; 4],
  pub coh_num_addr_range: UInt64,
  pub ncoh_num_addr_range: UInt64,
  pub ncoh_addr_range: [RmiAddressRange; 16],
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
  pub ats: RmiFeature,
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
  pub ats_plane: UInt64,
  pub vmid: UInt16,
  pub rtt_level_start: Int64,
  pub rtt_num_start: UInt32,
  pub flags1: RmiRealmFlags1,
  pub mecid: UInt64,
  pub aux_vmid: [UInt16; 3],
  pub aux_rtt_base: [Address; 3],
  pub rtt_base: Address,
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
  pub force_p0: RmiForceP0,
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
  pub cntv_ctl: UInt64,
  pub cntv_cval: UInt64,
  pub ripas_base: UInt64,
  pub ripas_top: UInt64,
  pub ripas_value: RmiRipas,
  pub s2ap_base: UInt64,
  pub s2ap_top: UInt64,
  pub vdev_id: UInt64,
  pub dev_mem_base: UInt64,
  pub dev_mem_top: UInt64,
  pub dev_mem_pa: Address,
  pub imm: UInt16,
  pub plane: UInt64,
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
  pub aux: [Address; 32],
}

struct RmiRecRun {
  pub enter: RmiRecEnter,
  pub exit: RmiRecExit,
}

struct RmiVdevFlags {
  pub VSMMU: RmiFeature,
}

struct RmiVdevMeasureFlags {
}

struct RmiVdevMeasureParams {
  pub flags: RmiVdevMeasureFlags,
  pub indices: [UInt64; 4],
  pub nonce: [UInt64; 4],
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

struct RsiDevMemFlags {
  pub coh: RsiDevMemCoherent,
  pub order: RsiDevMemOrdering,
}

struct RsiFeatureRegister0 {
  pub DA: RsiFeature,
  pub MRO: RsiFeature,
  pub ATS: RsiFeature,
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
  pub spsr_el2: UInt64,
  pub gprs: [UInt64; 31],
  pub gicv3_lrs: [UInt64; 16],
}

struct RsiPlaneEnterFlags {
  pub trap_wfi: RsiTrap,
  pub trap_wfe: RsiTrap,
  pub trap_hc: RsiTrap,
  pub gic_owner: RsiGicOwner,
  pub trap_simd: RsiTrap,
}

struct RsiPlaneExit {
  pub reason: RsiPlaneExitReason,
  pub elr_el2: UInt64,
  pub esr_el2: UInt64,
  pub far_el2: UInt64,
  pub hpfar_el2: UInt64,
  pub spsr_el2: UInt64,
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
  pub ats_plane: UInt64,
  pub rpv: [UInt64; 8],
}

struct RsiRipasChangeFlags {
  pub destroyed: RsiRipasChangeDestroyed,
}

struct RsiSysregAddress {
  pub Op2: UInt3,
  pub CRm: UInt4,
  pub CRn: UInt4,
  pub Op1: UInt3,
  pub Op0: UInt2,
  pub d128: RsiBoolean,
}

struct RsiVdevDmaFlags {
  pub ats: RsiFeature,
}

struct RsiVdevFlags {
  pub p2p: RsiFeature,
}

struct RsiVdevInfo {
  pub flags: RsiVdevFlags,
  pub attest_type: RsiVdevAttestType,
  pub cert_id: UInt64,
  pub hash_algo: RsiHashAlgorithm,
  pub lock_nonce: UInt64,
  pub meas_nonce: UInt64,
  pub report_nonce: UInt64,
  pub tdisp_version: UInt64,
  pub state: RsiVdevState,
  pub vca_digest: [UInt64; 8],
  pub cert_digest: [UInt64; 8],
  pub meas_digest: [UInt64; 8],
  pub report_digest: [UInt64; 8],
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

struct RmmMemPerms {
  pub values: [UInt64; 16],
}

struct RmmP2PStream {
  pub num_pdevs: UInt64,
}

struct RmmPdev {
  pub pdev_id: UInt64,
  pub spdm: RmmPdevSpdm,
  pub ncoh_ide: RmmPdevIde,
  pub ncoh_addr: RmmFeature,
  pub coh_ide: RmmPdevIde,
  pub coh_addr: RmmFeature,
  pub segment_id: UInt8,
  pub ecam_addr: Address,
  pub root_id: UInt16,
  pub cert_id: UInt64,
  pub rid_base: UInt16,
  pub rid_top: UInt16,
  pub hash_algo: RmmHashAlgorithm,
  pub ncoh_ide_sid: UInt64,
  pub ncoh_addr_range: [RmmAddressRange; 16],
  pub coh_addr_range: [RmmAddressRange; 4],
  pub aux: [Address; 32],
  pub num_aux: UInt64,
  pub state: RmmPdevState,
  pub comm_state: RmmDevCommState,
  pub num_vdevs: UInt64,
  pub p2p_enabled: RmmFeature,
  pub p2p_added: RmmBoolean,
  pub p2p_addr: Address,
  pub vca_digest: [UInt64; 8],
  pub coh_num_addr_range: UInt64,
  pub ncoh_num_addr_range: UInt64,
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
  pub feat_ats: RmmFeature,
  pub ats_plane: UInt64,
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
  pub num_vsmmus: UInt64,
}

struct RmmRec {
  pub owner: Address,
  pub aux: [Address; 32],
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
  pub vdev_pa: Address,
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
  pub base_index: RmmRttS2APBase,
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
  pub pdev: Address,
  pub realm: Address,
  pub vdev_state: RmmVdevState,
  pub dma_state: RmmVdevDmaState,
  pub op: RmmVdevOperation,
  pub comm_state: RmmDevCommState,
  pub aux: [Address; 32],
  pub num_aux: UInt64,
  pub vsmmu: RmmFeature,
  pub vsmmu_addr: Address,
  pub vsid: UInt64,
  pub num_map: UInt64,
  pub lock_nonce: UInt64,
  pub meas_nonce: UInt64,
  pub report_nonce: UInt64,
  pub meas_digest: [UInt64; 8],
  pub report_digest: [UInt64; 8],
}

struct RmmVsmmu {
  pub state: RmmVsmmuState,
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

pub open spec fn AuxAlias32(s: S, obj: Address, aux: [Address; 32], count: int) -> bool;

pub open spec fn AuxAligned32(s: S, aux: [Address; 32], count: int) -> bool;

pub open spec fn AuxEqual32(s: S, aux1: [Address; 32], aux2: [Address; 32], count: int) -> bool;

pub open spec fn AuxSort(s: S, addrs: [Address; 16], count: int) -> [Address; 16];

pub open spec fn AuxStateEqual32(s: S, aux: [Address; 32], count: int, state: RmmGranuleState) -> bool;

pub open spec fn AuxStates(s: S, aux: [Address; 32], count: int);

pub open spec fn CurrentRealm(s: S) -> RmmRealm;

pub open spec fn CurrentRec(s: S) -> RmmRec;

pub open spec fn DeviceCommunicate(s: S, pdev: RmmPdev, data: RmiDevCommData) -> RmmDevCommState;

pub open spec fn Equal(abstract_: RmmFeature, concrete: RmiFeature) -> bool;

pub open spec fn FeatureToRmi(s: S, value: RmmFeature) -> RmiFeature;

pub open spec fn FeatureToRsi(s: S, value: RmmFeature) -> RsiFeature;

pub open spec fn Gicv3ConfigIsValid(s: S, gicv3_hcr: u64, gicv3_lrs: [u64; 16]) -> bool;

pub open spec fn GranuleAccessPermitted(s: S, addr: Address, pas: RmmPhysicalAddressSpace) -> bool;

pub open spec fn GranuleAt(s: S, addr: Address) -> RmmGranule;

pub open spec fn GranulesAllState(s: S, base: Address, top: Address, state: RmmGranuleState) -> bool;

pub open spec fn GranulesAllVdevUnvalidated(s: S, base: Address, top: Address) -> bool;

pub open spec fn GranulesAllVdevValidated(s: S, base: Address, top: Address, vdev: RmmVdev) -> bool;

pub open spec fn ImplFeatures(s: S) -> RmmFeatures;

pub open spec fn MecMembers(s: S, mecid: u64) -> int;

pub open spec fn MecPolicy(s: S, mecid: u64) -> RmmMecPolicy;

pub open spec fn MecState(s: S, mecid: u64) -> RmmMecState;

pub open spec fn MemPermLabelSupported(s: S, label: u64) -> bool;

pub open spec fn MinAddress(s: S, addr1: Address, addr2: Address) -> Address;

pub open spec fn MpidrEqual(rmm_mpidr: u64, rmi_mpidr: RmiRecMpidr) -> bool;

pub open spec fn MpidrIsUsed(s: S, mpidr: u64) -> bool;

pub open spec fn MsiAddrIsValid(s: S, addr: Address) -> bool;

pub open spec fn P2PStreamAt(s: S, addr: Address) -> RmmP2PStream;

pub open spec fn PaIsDelegable(s: S, addr: Address) -> bool;

pub open spec fn PaIsDelegableCohDevMem(s: S, addr: Address) -> bool;

pub open spec fn PaIsDelegableDevMem(s: S, addr: Address) -> bool;

pub open spec fn PaIsDelegableDram(s: S, addr: Address) -> bool;

pub open spec fn PaIsDelegableNonCohDevMem(s: S, addr: Address) -> bool;

pub open spec fn PdevAt(s: S, addr: Address) -> RmmPdev;

pub open spec fn PdevAuxCount(s: S, flags: RmiPdevFlags) -> int;

pub open spec fn PdevFlags(s: S, pdev: RmmPdev) -> RmiPdevFlags;

pub open spec fn PdevVsmmuIsCompatible(s: S, pdev: RmmPdev, vsmmu: RmmVsmmu) -> bool;

pub open spec fn PsciReturnCodeEncode(s: S, value: PsciReturnCode) -> u64;

pub open spec fn PsciReturnCodePermitted(s: S, calling_rec: RmmRec, target_rec: RmmRec, value: PsciReturnCode) -> bool;

pub open spec fn PsmmuAddrIsValid(s: S, addr: Address) -> bool;

pub open spec fn PsmmuSupportsMsi(s: S, addr: Address) -> bool;

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

pub open spec fn RecSysregValid(s: S, rec: RmmRec, addr: RsiSysregAddress, is_write: bool) -> bool;

pub open spec fn RecSysregValue(s: S, rec: RmmRec, plane_idx: int, addr: RsiSysregAddress) -> (u64, u64);

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

pub open spec fn RmiPdevFlagsSupported(s: S, flags: RmiPdevFlags) -> bool;

pub open spec fn RmiPdevParamsAt(s: S, addr: Address) -> RmiPdevParams;

pub open spec fn RmiPdevParamsIsValid(s: S, addr: Address) -> bool;

pub open spec fn RmiPublicKeyParamsAt(s: S, addr: Address) -> RmiPublicKeyParams;

pub open spec fn RmiRealmParamsAt(s: S, addr: Address) -> RmiRealmParams;

pub open spec fn RmiRealmParamsIsValid(s: S, addr: Address) -> bool;

pub open spec fn RmiRecParamsAt(s: S, addr: Address) -> RmiRecParams;

pub open spec fn RmiRecRunAt(s: S, addr: Address) -> RmiRecRun;

pub open spec fn RmiVdevFlagsDecode(s: S, value: u64) -> RmiVdevFlags;

pub open spec fn RmiVdevMeasureParamsAt(s: S, addr: Address) -> RmiVdevMeasureParams;

pub open spec fn RmiVdevParamsAt(s: S, addr: Address) -> RmiVdevParams;

pub open spec fn RmiVdevParamsIsValid(s: S, addr: Address) -> bool;

pub open spec fn RmiVersionHigherIsSupported(s: S, version: RmiInterfaceVersion) -> bool;

pub open spec fn RmiVersionHighest(s: S) -> RmiInterfaceVersion;

pub open spec fn RmiVersionHighestBelow(s: S, version: RmiInterfaceVersion) -> RmiInterfaceVersion;

pub open spec fn RmiVersionIsSupported(s: S, version: RmiInterfaceVersion) -> bool;

pub open spec fn RmiVersionLowerIsSupported(s: S, version: RmiInterfaceVersion) -> bool;

pub open spec fn RmiVsmmuParamsAt(s: S, addr: Address) -> RmiVsmmuParams;

pub open spec fn RmiVsmmuParamsIsValid(s: S, addr: Address) -> bool;

pub open spec fn RsiFeatureRegisterEncode(s: S, realm: RmmRealm, index: int) -> u64;

pub open spec fn RsiHostCallAt(s: S, addr: Address) -> RsiHostCall;

pub open spec fn RsiPlaneRunAt(s: S, realm: RmmRealm, addr: Address) -> RsiPlaneRun;

pub open spec fn RsiRealmConfigAt(s: S, addr: Address) -> RsiRealmConfig;

pub open spec fn RsiVdevInfoAt(s: S, addr: Address) -> RsiVdevInfo;

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

pub open spec fn VdevFromVdevId(s: S, realm: RmmRealm, vdev_id: u64) -> RmmVdev;

pub open spec fn VdevGenerateNonce(s: S, vdev: RmmVdev) -> int;

pub open spec fn VdevIdIsFree(s: S, realm: RmmRealm, vdev_id: u64) -> bool;

pub open spec fn VersionEqual(ver1: RmiInterfaceVersion, ver2: RmiInterfaceVersion) -> bool;

pub open spec fn VmidsAreFree(s: S, vmid: [u16; 4]) -> bool;

pub open spec fn VmidsAreValid(s: S, vmid: u16, aux_vmid: [u16; 3]) -> bool;

pub open spec fn VsidIsFree(s: S, vsmmu: RmmVsmmu, vsid: u64) -> bool;

pub open spec fn VsmmuAt(s: S, addr: Address) -> RmmVsmmu;

pub open spec fn VsmmuIsLive(s: S, addr: Address) -> bool;

pub open spec fn VmidsAreFree1(s: S, vmid: u16, aux_vmid: [u16;3]) -> bool;

pub open spec fn DeviceCommunicate1(s: S, vdev: RmmVdev) -> RmmDevCommState;

pub open spec fn DeviceCommunicate2(s: S, vdev: RmmVdev, data: RmiDevCommData) -> RmmDevCommState;

