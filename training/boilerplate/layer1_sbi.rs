use vstd::prelude::*;

verus! {

// ---------------------------------------------------------------------------
// Primitive type aliases
// ---------------------------------------------------------------------------

type UInt32  = u32;
type UInt64  = u64;
type HartId  = u64;
type PhysAddr = u64;
type Bits64  = u64;

// ---------------------------------------------------------------------------
// SBI error codes  (Table 3, RISC-V SBI Specification v2.0)
// ---------------------------------------------------------------------------

pub spec const SBI_SUCCESS:               int =  0;
pub spec const SBI_ERR_FAILED:            int = -1;
pub spec const SBI_ERR_NOT_SUPPORTED:     int = -2;
pub spec const SBI_ERR_INVALID_PARAM:     int = -3;
pub spec const SBI_ERR_DENIED:            int = -4;
pub spec const SBI_ERR_INVALID_ADDRESS:   int = -5;
pub spec const SBI_ERR_ALREADY_AVAILABLE: int = -6;
pub spec const SBI_ERR_ALREADY_STARTED:   int = -7;
pub spec const SBI_ERR_ALREADY_STOPPED:   int = -8;
pub spec const SBI_ERR_NO_SHMEM:          int = -9;
pub spec const SBI_ERR_INVALID_STATE:     int = -10;
pub spec const SBI_ERR_BAD_RANGE:         int = -11;
pub spec const SBI_ERR_TIMEOUT:           int = -12;
pub spec const SBI_ERR_IO:                int = -13;

// ---------------------------------------------------------------------------
// HSM hart state values  (§8 Hart State Management)
// ---------------------------------------------------------------------------

pub spec const HART_STARTED:          int = 0;
pub spec const HART_STOPPED:          int = 1;
pub spec const HART_START_PENDING:    int = 2;
pub spec const HART_STOP_PENDING:     int = 3;
pub spec const HART_SUSPENDED:        int = 4;
pub spec const HART_SUSPEND_PENDING:  int = 5;
pub spec const HART_RESUME_PENDING:   int = 6;

// ---------------------------------------------------------------------------
// System Reset types and reasons  (§9 System Reset Extension)
// ---------------------------------------------------------------------------

pub spec const RESET_TYPE_SHUTDOWN:    int = 0x00000000;
pub spec const RESET_TYPE_COLD_REBOOT: int = 0x00000001;
pub spec const RESET_TYPE_WARM_REBOOT: int = 0x00000002;
pub spec const RESET_REASON_NONE:      int = 0x00000000;
pub spec const RESET_REASON_SYSFAIL:   int = 0x00000001;

// ---------------------------------------------------------------------------
// Global state (minimal — one hart-state byte per hart, 512 harts max)
// ---------------------------------------------------------------------------

pub struct S {
    pub harts: [u8; 64],   // hart power state (truncated for spec purposes)
}

// ---------------------------------------------------------------------------
// Uninterpreted helper predicates
// ---------------------------------------------------------------------------

/// Returns true if hart_id identifies a valid hart in the system.
pub open spec fn HartIsValid(s: S, hart_id: HartId) -> bool;

/// Returns the current power state of a hart (HART_STARTED, HART_STOPPED, etc.).
pub open spec fn HartState(s: S, hart_id: HartId) -> int;

/// Returns true if the physical address range [base, base+size) is valid and accessible.
pub open spec fn AddrRangeIsValid(base: PhysAddr, size: u64) -> bool;

/// Returns true if the given physical address is naturally aligned to `align` bytes.
pub open spec fn AddrIsAligned(addr: PhysAddr, align: u64) -> bool;

/// Returns true if the shared memory region [base, base+size) is usable for SBI shared memory.
pub open spec fn ShmemRegionIsValid(s: S, base: PhysAddr, size: u64) -> bool;

/// Returns true if the hart mask [hart_mask_base, ...] selects at least one valid hart.
pub open spec fn HartMaskIsValid(s: S, hart_mask: u64, hart_mask_base: HartId) -> bool;

/// Returns true if the given SBI extension ID is available on this platform.
pub open spec fn ExtensionIsAvailable(eid: u64) -> bool;

/// Returns true if PMU counter `counter_idx` exists and is of the given type.
pub open spec fn PmuCounterExists(s: S, counter_idx: u64) -> bool;

/// Returns true if PMU counter is currently running/started.
pub open spec fn PmuCounterIsStarted(s: S, counter_idx: u64) -> bool;

} // verus!
