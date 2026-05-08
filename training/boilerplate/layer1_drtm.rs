use vstd::prelude::*;

verus! {

// ---------------------------------------------------------------------------
// Primitive type aliases
// ---------------------------------------------------------------------------

type UInt32  = u32;
type UInt64  = u64;
type Int64   = i64;
type Bits64  = u64;
type Address = u64;

// ---------------------------------------------------------------------------
// DRTM return codes  (Table 20 in DEN0113 v1.4, §3.18)
// ---------------------------------------------------------------------------

pub spec const DRTM_SUCCESS:              int = 0;
pub spec const DRTM_NOT_SUPPORTED:        int = -1;
pub spec const DRTM_INVALID_PARAMETERS:   int = -2;
pub spec const DRTM_DENIED:               int = -3;
pub spec const DRTM_NOT_FOUND:            int = -4;
pub spec const DRTM_INTERNAL_ERROR:       int = -5;
pub spec const DRTM_MEM_PROTECT_RANGE_ERROR: int = -6;

// ---------------------------------------------------------------------------
// DRTM function IDs  (DEN0113 v1.4, §3.2 ff.)
// ---------------------------------------------------------------------------

pub spec const DRTM_VERSION_FID:                  int = 0xC400_0110;
pub spec const DRTM_FEATURES_FID:                 int = 0xC400_0111;
pub spec const DRTM_DYNAMIC_LAUNCH_FID:           int = 0xC400_0114;
pub spec const DRTM_UNPROTECT_MEMORY_FID:         int = 0xC400_0115;
pub spec const DRTM_CLOSE_LOCALITY_FID:           int = 0xC400_0116;
pub spec const DRTM_GET_ERROR_FID:                int = 0xC400_0117;
pub spec const DRTM_SET_ERROR_FID:                int = 0x4400_0118;
pub spec const DRTM_SET_TCB_HASH_FID:             int = 0xC400_0119;
pub spec const DRTM_LOCK_TCB_HASHES_FID:          int = 0x4400_011A;
pub spec const DRTM_ENABLE_SECURE_INTERRUPTS_FID: int = 0x4400_011B;

// ---------------------------------------------------------------------------
// DRTM error code wrapper (used in Result<T, DRTMError>)
// ---------------------------------------------------------------------------

pub struct DRTMError(pub int);
impl DRTMError {
    pub open spec fn as_int(self) -> int { self.0 }
}

// ---------------------------------------------------------------------------
// Global state (minimal — DRTM is per-invocation with some persistent state)
// ---------------------------------------------------------------------------

pub struct S {
    /// Whether DRTM has been successfully launched (DRTM_DYNAMIC_LAUNCH succeeded).
    pub drtm_launched: bool,
    /// Whether locality 2 is currently active (DRTM_CLOSE_LOCALITY not yet called).
    pub locality2_active: bool,
    /// Whether TCB hashes are locked (DRTM_LOCK_TCB_HASHES called).
    pub tcb_hashes_locked: bool,
    /// Count of TCB hashes recorded so far.
    pub tcb_hash_count: u64,
    /// Maximum supported TCB hashes (from DRTM_FEATURES).
    pub tcb_hash_max: u64,
    /// Whether DLME data has been populated (needed for DRTM_GET_ERROR).
    pub dlme_data_available: bool,
}

// ---------------------------------------------------------------------------
// Uninterpreted helper predicates used by DRTM command specs
// ---------------------------------------------------------------------------

/// Returns true if DRTM is supported on this platform.
pub open spec fn DrtmIsSupported(s: S) -> bool;

/// Returns true if DRTM has been initialized (DRTM_DYNAMIC_LAUNCH completed).
pub open spec fn DrtmIsInitialized(s: S) -> bool;

/// Returns true if locality-2 DCE is currently active.
pub open spec fn DrtmLocality2IsActive(s: S) -> bool;

/// Returns true if TCB hashes are locked.
pub open spec fn TcbIsLocked(s: S) -> bool;

/// Returns true if the DLME data region is available and valid.
pub open spec fn DlmeDataIsAvailable(s: S) -> bool;

/// Returns true if the address range [base, base+size) is valid DRTM-protected memory.
pub open spec fn AddrRangeIsProtected(s: S, base: Address, size: Bits64) -> bool;

/// Returns true if the caller has privilege to invoke this DRTM function.
pub open spec fn CallerIsPrivileged(s: S) -> bool;

} // verus!
