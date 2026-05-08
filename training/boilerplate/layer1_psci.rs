use vstd::prelude::*;

verus! {

// ---------------------------------------------------------------------------
// Primitive type aliases (matches RMM layer1 for interoperability)
// ---------------------------------------------------------------------------

type UInt32  = u32;
type UInt64  = u64;
type Int32   = i32;
type Bits64  = u64;
type Address = u64;

// ---------------------------------------------------------------------------
// PSCI return codes  (Table 5 in DEN0022E.b)
// ---------------------------------------------------------------------------

pub spec const PSCI_SUCCESS:             int = 0;
pub spec const PSCI_NOT_SUPPORTED:       int = -1;
pub spec const PSCI_INVALID_PARAMETERS:  int = -2;
pub spec const PSCI_DENIED:              int = -3;
pub spec const PSCI_ALREADY_ON:          int = -4;
pub spec const PSCI_ON_PENDING:          int = -5;
pub spec const PSCI_INTERNAL_FAILURE:    int = -6;
pub spec const PSCI_NOT_PRESENT:         int = -7;
pub spec const PSCI_DISABLED:            int = -8;
pub spec const PSCI_INVALID_ADDRESS:     int = -9;

// ---------------------------------------------------------------------------
// PSCI function IDs  (Table 1 in DEN0022E.b, SMC32 and SMC64 variants)
// ---------------------------------------------------------------------------

// SMC32 (W0 = 32-bit FID)
pub spec const PSCI_VERSION_FID:            int = 0x84000000;
pub spec const PSCI_CPU_SUSPEND_32_FID:     int = 0x84000001;
pub spec const PSCI_CPU_OFF_FID:            int = 0x84000002;
pub spec const PSCI_CPU_ON_32_FID:          int = 0x84000003;
pub spec const PSCI_AFFINITY_INFO_32_FID:   int = 0x84000004;
pub spec const PSCI_MIGRATE_32_FID:         int = 0x84000005;
pub spec const PSCI_MIGRATE_INFO_TYPE_FID:  int = 0x84000006;
pub spec const PSCI_MIGRATE_INFO_CPU_32_FID:int = 0x84000007;
pub spec const PSCI_SYSTEM_OFF_FID:         int = 0x84000008;
pub spec const PSCI_SYSTEM_RESET_FID:       int = 0x84000009;
pub spec const PSCI_SYSTEM_RESET2_32_FID:   int = 0x84000012;
pub spec const PSCI_MEM_PROTECT_FID:        int = 0x84000013;
pub spec const PSCI_MEM_PROTECT_CHECK_32:   int = 0x84000014;
pub spec const PSCI_FEATURES_FID:           int = 0x8400000A;
pub spec const PSCI_CPU_FREEZE_FID:         int = 0x8400000B;
pub spec const PSCI_CPU_DEFAULT_SUSPEND_32: int = 0x8400000C;
pub spec const PSCI_NODE_HW_STATE_32_FID:   int = 0x8400000D;
pub spec const PSCI_SYSTEM_SUSPEND_32_FID:  int = 0x8400000E;
pub spec const PSCI_SET_SUSPEND_MODE_FID:   int = 0x8400000F;
pub spec const PSCI_STAT_RESIDENCY_32_FID:  int = 0x84000010;
pub spec const PSCI_STAT_COUNT_32_FID:      int = 0x84000011;

// SMC64 variants (where applicable)
pub spec const PSCI_CPU_SUSPEND_64_FID:     int = 0xC4000001;
pub spec const PSCI_CPU_ON_64_FID:          int = 0xC4000003;
pub spec const PSCI_AFFINITY_INFO_64_FID:   int = 0xC4000004;
pub spec const PSCI_MIGRATE_64_FID:         int = 0xC4000005;
pub spec const PSCI_MIGRATE_INFO_CPU_64_FID:int = 0xC4000007;
pub spec const PSCI_SYSTEM_RESET2_64_FID:   int = 0xC4000012;
pub spec const PSCI_MEM_PROTECT_CHECK_64:   int = 0xC4000014;
pub spec const PSCI_CPU_DEFAULT_SUSPEND_64: int = 0xC400000C;
pub spec const PSCI_NODE_HW_STATE_64_FID:   int = 0xC400000D;
pub spec const PSCI_SYSTEM_SUSPEND_64_FID:  int = 0xC400000E;
pub spec const PSCI_STAT_RESIDENCY_64_FID:  int = 0xC4000010;
pub spec const PSCI_STAT_COUNT_64_FID:      int = 0xC4000011;

// ---------------------------------------------------------------------------
// Affinity state values  (Section 5.5, DEN0022E.b)
// ---------------------------------------------------------------------------

pub spec const PSCI_AFFINITY_LEVEL_ON:       int = 0;
pub spec const PSCI_AFFINITY_LEVEL_OFF:      int = 1;
pub spec const PSCI_AFFINITY_LEVEL_ON_PENDING: int = 2;

// ---------------------------------------------------------------------------
// HW_STATE return values  (Section 5.17)
// ---------------------------------------------------------------------------

pub spec const PSCI_HW_STATE_ON:             int = 0;
pub spec const PSCI_HW_STATE_OFF:            int = 1;
pub spec const PSCI_HW_STATE_STANDBY:        int = 2;

// ---------------------------------------------------------------------------
// PSCI_FEATURES flags / capability bits
// ---------------------------------------------------------------------------

pub spec const PSCI_FEATURES_OS_INIT_MODE:   int = 1;  // bit 0 of features return value

// ---------------------------------------------------------------------------
// Global state (minimal — PSCI is mostly stateless per-call)
// ---------------------------------------------------------------------------

struct S {
    /// Per-CPU power state array (64 CPUs max).
    /// cpu_state[i] encodes the power state of CPU i.
    pub cpu_state: [u8; 64],
}

// ---------------------------------------------------------------------------
// Uninterpreted helper functions used by PSCI command specs
// ---------------------------------------------------------------------------

/// Returns true if `target_cpu` identifies a valid CPU in the system.
pub open spec fn CpuIsValid(s: S, target_cpu: Bits64) -> bool;

/// Returns true if `target_cpu` is currently powered on.
pub open spec fn CpuIsOn(s: S, target_cpu: Bits64) -> bool;

/// Returns true if `target_cpu` is currently in the PENDING state.
pub open spec fn CpuIsOnPending(s: S, target_cpu: Bits64) -> bool;

/// Returns true if the entry_point_address is in Non-Secure memory.
pub open spec fn AddrIsNonSecure(s: S, addr: Address) -> bool;

/// Returns true if the calling CPU is the last remaining powered-on CPU at
/// the affinity level used for MIGRATE.
pub open spec fn CallerIsLastCpu(s: S) -> bool;

/// Returns true if a Trusted OS migration is in progress.
pub open spec fn TrustedOsMigrationInProgress(s: S) -> bool;

} // verus!
