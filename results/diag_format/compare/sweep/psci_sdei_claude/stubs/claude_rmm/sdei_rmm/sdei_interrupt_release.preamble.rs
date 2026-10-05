use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type Int64 = i64;
pub type PeId = u64;
pub type InterruptId = u32;
pub type InterruptCfg = u64;
pub type HandlerStateT = u8;

pub struct S {
    pub sdei_supported: bool,
    pub valid_events: Set<Int32>,
    pub bound_events: Set<Int32>,
    pub private_events: Set<Int32>,
    pub shared_events: Set<Int32>,
    pub registered_pes: Map<Int32, Set<PeId>>,
    pub handler_states: Map<(Int32, PeId), HandlerStateT>,
    pub bound_interrupts: Map<Int32, InterruptId>,
    pub interrupt_configs: Map<InterruptId, InterruptCfg>,
    pub pre_bind_interrupt_configs: Map<InterruptId, InterruptCfg>,
    pub enabled_interrupts: Set<InterruptId>,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;
pub const PENDING: Int64 = -5;
pub const OUT_OF_RESOURCE: Int64 = -10;

pub const HANDLER_UNREGISTERED: HandlerStateT = 0;
pub const HANDLER_REGISTERED: HandlerStateT = 1;
pub const HANDLER_ENABLED: HandlerStateT = 2;
pub const HANDLER_RUNNING: HandlerStateT = 3;

pub open spec fn IsSdeiSupported(s: S) -> bool;

pub open spec fn IsValidEvent(s: S, event: Int32) -> bool;

pub open spec fn IsEventBound(s: S, event: Int32) -> bool;

pub open spec fn IsPrivateEvent(s: S, event: Int32) -> bool;

pub open spec fn IsSharedEvent(s: S, event: Int32) -> bool;

pub open spec fn RegisteredPes(s: S, event: Int32) -> Set<PeId>;

pub open spec fn HandlerState(s: S, event: Int32, pe: PeId) -> HandlerStateT;

pub open spec fn ResultEqual(result: Int64, expected: Int64) -> bool;

pub open spec fn BoundInterrupt(s: S, event: Int32) -> InterruptId;

pub open spec fn InterruptConfig(s: S, intr: InterruptId) -> InterruptCfg;

pub open spec fn PreBindInterruptConfig(s: S, intr: InterruptId) -> InterruptCfg;

pub open spec fn IsInterruptEnabled(s: S, intr: InterruptId) -> bool;

} // verus!
