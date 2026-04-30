use vstd::prelude::*;

verus! {

// ---------------------------------------------------------------------------
// Primitive type aliases
// ---------------------------------------------------------------------------

type UInt32  = u32;
type UInt64  = u64;
type HPA     = u64;   // Host Physical Address
type GPA     = u64;   // Guest Physical Address
type Bits64  = u64;

// ---------------------------------------------------------------------------
// TDX Module error codes  (Intel TDX Module ABI v1.5, Chapter 6)
// Error codes are 64-bit values; we encode them as int for Verus spec purposes.
// ---------------------------------------------------------------------------

pub spec const TDX_SUCCESS:                        int = 0x0000_0000_0000_0000;

// Operand errors (0xC000_01xx)
pub spec const TDX_OPERAND_INVALID:                int = 0xC000_0100_0000_0000;
pub spec const TDX_OPERAND_ADDR_RANGE_ERROR:       int = 0xC000_0101_0000_0000;
pub spec const TDX_OPERAND_BUSY:                   int = 0x8000_0200_0000_0000;
pub spec const TDX_OPERAND_STATE_INCORRECT:        int = 0xC000_0200_0000_0000;
pub spec const TDX_OPERAND_NOT_FREE:               int = 0xC000_0201_0000_0000;

// TD/vCPU state errors
pub spec const TDX_TD_VCPU_STATE_INCORRECT:        int = 0xC000_0A02_0000_0000;
pub spec const TDX_TD_ASSOCIATED_VCPUS_EXIST:      int = 0xC000_0A03_0000_0000;
pub spec const TDX_VCPU_NOT_ASSOCIATED:            int = 0xC000_0A04_0000_0000;

// Key / crypto errors
pub spec const TDX_KEY_GENERATION_FAILED:          int = 0x8000_0B00_0000_0000;
pub spec const TDX_TD_KEYS_NOT_CONFIGURED:         int = 0x8000_0B01_0000_0000;
pub spec const TDX_KEY_STATE_INCORRECT:            int = 0xC000_0B02_0000_0000;
pub spec const TDX_KEY_CONFIGURED:                 int = 0xC000_0B03_0000_0000;

// Memory errors
pub spec const TDX_MEMORY_NOT_TDMR:                int = 0xC000_0C00_0000_0000;
pub spec const TDX_MEMORY_OUT_OF_BOUNDARIES:       int = 0xC000_0C01_0000_0000;
pub spec const TDX_GPA_RANGE_NOT_LOCKED:           int = 0xC000_0E00_0000_0000;
pub spec const TDX_GPA_RANGE_ALREADY_LOCKED:       int = 0xC000_0E01_0000_0000;

// System / API errors
pub spec const TDX_INCORRECT_API_VERSION:          int = 0xC000_0000_0000_0000;
pub spec const TDX_NOT_IMPLEMENTED:                int = 0xC000_0001_0000_0000;
pub spec const TDX_SYS_NOT_READY:                  int = 0xC000_0004_0000_0000;
pub spec const TDX_SYS_CONFIG_NOT_PENDING:         int = 0xC000_0005_0000_0000;
pub spec const TDX_SYS_LP_INIT_NOT_DONE:           int = 0xC000_0006_0000_0000;

// ---------------------------------------------------------------------------
// TD lifecycle states
// ---------------------------------------------------------------------------

pub spec const TD_UNINITIALIZED: int = 0;
pub spec const TD_INITIALIZED:   int = 1;
pub spec const TD_RUNNABLE:      int = 2;
pub spec const TD_TEARDOWN:      int = 3;

// ---------------------------------------------------------------------------
// vCPU lifecycle states
// ---------------------------------------------------------------------------

pub spec const VCPU_UNINITIALIZED: int = 0;
pub spec const VCPU_READY:         int = 1;
pub spec const VCPU_ACTIVE:        int = 2;
pub spec const VCPU_BLOCKED:       int = 3;
pub spec const VCPU_TEARDOWN:      int = 4;

// ---------------------------------------------------------------------------
// Page types (SEPT / TDMR page classifications)
// ---------------------------------------------------------------------------

pub spec const PT_NDA:      int = 0;  // Not a TDX-managed page
pub spec const PT_RSVD:     int = 1;
pub spec const PT_REG:      int = 3;  // Regular TD page
pub spec const PT_SS:       int = 4;  // Secure-stack page
pub spec const PT_KEY_SCHED:int = 5;
pub spec const PT_TDHOB:    int = 6;  // TD Hand-Off Block
pub spec const PT_MKTME:    int = 7;

// ---------------------------------------------------------------------------
// Global state (minimal opaque struct)
// ---------------------------------------------------------------------------

pub struct S {
    pub mem: [u8; 64],
}

// ---------------------------------------------------------------------------
// Uninterpreted helper predicates
// ---------------------------------------------------------------------------

/// Returns true if a TD control structure exists at hpa.
pub open spec fn TdExists(s: S, hpa: HPA) -> bool;

/// Returns the lifecycle state of the TD at hpa.
pub open spec fn TdState(s: S, hpa: HPA) -> int;

/// Returns true if a vCPU control structure for (td_hpa, vcpu_id) exists.
pub open spec fn VcpuExists(s: S, td_hpa: HPA, vcpu_id: u32) -> bool;

/// Returns the lifecycle state of a vCPU.
pub open spec fn VcpuState(s: S, td_hpa: HPA, vcpu_id: u32) -> int;

/// Returns true if the HPA page is free (not assigned to any TD).
pub open spec fn PageIsFree(s: S, hpa: HPA) -> bool;

/// Returns the page type classification of hpa.
pub open spec fn PageType(s: S, hpa: HPA) -> int;

/// Returns true if TD keys have been configured for the TD at td_hpa.
pub open spec fn KeyIsAssigned(s: S, td_hpa: HPA) -> bool;

/// Returns true if the GPA is mapped in the TD's Secure Extended Page Table.
pub open spec fn GpaIsMapped(s: S, td_hpa: HPA, gpa: GPA) -> bool;

/// Returns true if the address is naturally aligned to `align` bytes.
pub open spec fn AddrIsAligned(addr: u64, align: u64) -> bool;

/// Returns true if the given HPA lies within a valid TDMR (TD Memory Region).
pub open spec fn HpaIsInTdmr(s: S, hpa: HPA) -> bool;

/// Returns true if the TD measurement has been finalized.
pub open spec fn TdIsMeasurementFinalized(s: S, td_hpa: HPA) -> bool;

/// Returns true if the system has been initialized (TDH.SYS.INIT called).
pub open spec fn SysIsInitialized(s: S) -> bool;

} // verus!
