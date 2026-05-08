use vstd::prelude::*;

verus! {

// ---------------------------------------------------------------------------
// Primitive type aliases
// ---------------------------------------------------------------------------

type UInt32  = u32;
type UInt64  = u64;
// Int64 defined below as int (mathematical integer for Verus specs)
type Bits64  = u64;
type Address = u64;

// ---------------------------------------------------------------------------
// SDEI return codes  (Table 6 in DEN0054C, §5.3)
// ---------------------------------------------------------------------------

pub spec const SDEI_SUCCESS:             int = 0;
pub spec const SDEI_NOT_SUPPORTED:       int = -1;
pub spec const SDEI_INVALID_PARAMETERS:  int = -2;
pub spec const SDEI_DENIED:              int = -3;
pub spec const SDEI_PENDING:             int = -4;
pub spec const SDEI_CANCELLED:           int = -5;
pub spec const SDEI_OUT_OF_RESOURCE:     int = -6;

// ---------------------------------------------------------------------------
// SDEI function IDs  (DEN0054C, §5.1.x)
// ---------------------------------------------------------------------------

pub spec const SDEI_VERSION_FID:                     int = 0xC400_0020;
pub spec const SDEI_EVENT_REGISTER_FID:              int = 0xC400_0021;
pub spec const SDEI_EVENT_ENABLE_FID:                int = 0xC400_0022;
pub spec const SDEI_EVENT_DISABLE_FID:               int = 0xC400_0023;
pub spec const SDEI_EVENT_CONTEXT_FID:               int = 0xC400_0024;
pub spec const SDEI_EVENT_COMPLETE_FID:              int = 0xC400_0025;
pub spec const SDEI_EVENT_COMPLETE_AND_RESUME_FID:   int = 0xC400_0026;
pub spec const SDEI_EVENT_UNREGISTER_FID:            int = 0xC400_0027;
pub spec const SDEI_EVENT_STATUS_FID:                int = 0xC400_0028;
pub spec const SDEI_EVENT_GET_INFO_FID:              int = 0xC400_0029;
pub spec const SDEI_EVENT_ROUTING_SET_FID:           int = 0xC400_002A;
pub spec const SDEI_PE_MASK_FID:                     int = 0xC400_002B;
pub spec const SDEI_PE_UNMASK_FID:                   int = 0xC400_002C;
pub spec const SDEI_INTERRUPT_BIND_FID:              int = 0xC400_002D;
pub spec const SDEI_INTERRUPT_RELEASE_FID:           int = 0xC400_002E;
pub spec const SDEI_EVENT_SIGNAL_FID:                int = 0xC400_002F;
pub spec const SDEI_FEATURES_FID:                    int = 0xC400_0030;
pub spec const SDEI_PRIVATE_RESET_FID:               int = 0xC400_0031;
pub spec const SDEI_SHARED_RESET_FID:                int = 0xC400_0032;

// ---------------------------------------------------------------------------
// SDEI event handler state values  (§4.3)
// ---------------------------------------------------------------------------

pub spec const SDEI_STATE_UNREGISTERED:  int = 0;
pub spec const SDEI_STATE_REGISTERED:    int = 1;
pub spec const SDEI_STATE_ENABLED:       int = 2;
pub spec const SDEI_STATE_RUNNING:       int = 3;

// ---------------------------------------------------------------------------
// SDEI routing mode values
// ---------------------------------------------------------------------------

pub spec const SDEI_ROUTING_PE_SPECIFIC: int = 0;
pub spec const SDEI_ROUTING_ANY_PE:      int = 1;

// ---------------------------------------------------------------------------
// SDEI error code wrapper (used in Result<T, SDEIErrorCode>)
// ---------------------------------------------------------------------------

pub struct SDEIErrorCode(pub int);
impl SDEIErrorCode {
    pub open spec fn as_int(self) -> int { self.0 }
}

// Type aliases used by some model outputs
type Int64 = int;
type int64 = int;
type Bytes = int;  // placeholder; SDEI context data is opaque

// SDEI event handler state (§4.3)
pub enum SDEIState {
    Unregistered,
    Registered,
    Enabled,
    Running,
}

// ---------------------------------------------------------------------------
// Global state (minimal per-PE/per-event SDEI state)
// ---------------------------------------------------------------------------

pub struct S {
    /// Whether SDEI is supported on this PE.
    pub sdei_supported: bool,
    /// Whether the calling PE is currently masked.
    pub pe_masked: bool,
    /// Placeholder for per-event state (modelled as uninterpreted predicates).
    pub _padding: u64,
}

// ---------------------------------------------------------------------------
// Uninterpreted helper predicates used by SDEI command specs
// ---------------------------------------------------------------------------

/// Returns true if the event `ev` is registered.
pub open spec fn EventIsRegistered(s: S, ev: Bits64) -> bool;

/// Returns true if the event `ev` is enabled.
pub open spec fn EventIsEnabled(s: S, ev: Bits64) -> bool;

/// Returns true if the event `ev` is currently running (handler is executing).
pub open spec fn EventIsRunning(s: S, ev: Bits64) -> bool;

/// Returns true if the event `ev` is a shared (not private) SDEI event.
pub open spec fn EventIsShared(s: S, ev: Bits64) -> bool;

/// Returns true if the event `ev` is signalable (software-triggered).
pub open spec fn EventIsSignalable(s: S, ev: Bits64) -> bool;

/// Returns true if the current PE is masked for SDEI events.
pub open spec fn PeIsMasked(s: S) -> bool;

/// Returns true if the interrupt `intr` is already bound to an SDEI event.
pub open spec fn InterruptIsBound(s: S, intr: Bits64) -> bool;

/// u32 variant used by model-generated SDEI_INTERRUPT_BIND specs.
pub open spec fn InterruptIsBound32(s: S, intr: u32) -> bool;

/// Returns true if the given event ID is a valid SDEI event number.
pub open spec fn EventIdIsValid(s: S, ev: Bits64) -> bool;

/// Returns true if the routing configuration is valid (mode, pe combination).
pub open spec fn RoutingIsValid(s: S, routing_mode: Bits64, affinity: Bits64) -> bool;

/// Returns true if SDEI is NOT supported on this PE.
pub open spec fn SdeiIsNotSupported(s: S) -> bool;

/// Returns true if SDEI IS supported on this PE.
pub open spec fn SdeiIsSupported(s: S) -> bool;

/// Returns true if the given interrupt number is a valid hardware interrupt.
pub open spec fn InterruptIsValid(s: S, intr: u32) -> bool;

} // verus!
