use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;

pub type NotificationKind = u32;

pub const SYSTEM_POWER_STATE_NOTIFIER: NotificationKind = 0x1;

pub struct S {
    pub registered_agents: Set<UInt32>,
    pub notifications: Set<(UInt32, NotificationKind)>,
    pub platform_shutdown_timeout: bool,
}

pub uninterp spec fn bits_of(v: u32, hi: int, lo: int) -> int;

pub trait BitSliceIndex {
    spec fn spec_index(self, r: std::ops::Range<int>) -> int;
}

impl BitSliceIndex for u32 {
    open spec fn spec_index(self, r: std::ops::Range<int>) -> int {
        bits_of(self, r.start, r.end)
    }
}

pub uninterp spec fn IsRegisteredForSystemPowerStateNotify(s: S, agent_id: UInt32) -> bool;

pub uninterp spec fn NotificationSentToAgent(s: S, agent_id: UInt32, kind: NotificationKind) -> bool;

pub uninterp spec fn PlatformImposesShutdownTimeout(s: S) -> bool;

} // verus!
