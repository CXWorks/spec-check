use vstd::prelude::*;
verus! {

pub type unsigned_long = u64;

pub type HartId = u64;

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub hart_enabled: Map<HartId, bool>,
    pub hart_available: Map<HartId, bool>,
    pub ssip_pending: Map<HartId, bool>,
    pub failure_flag: bool,
}

pub spec const SBI_SUCCESS: sbiret = sbiret { error: 0, value: 0 };

pub spec const SBI_ERR_FAILED: sbiret = sbiret { error: -1, value: 0 };

pub spec const SBI_ERR_INVALID_PARAM: sbiret = sbiret { error: -3, value: 0 };

pub open spec fn HartIdsFromMask(hart_mask_base: unsigned_long, hart_mask: unsigned_long) -> Set<HartId>;

pub open spec fn IsHartEnabledByPlatform(s: S, h: HartId) -> bool;

pub open spec fn IsHartAvailableToSupervisor(s: S, h: HartId) -> bool;

pub open spec fn ResultEqual(result: sbiret, expected: sbiret) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason(s: S) -> bool;

pub open spec fn SupervisorSoftwareInterruptPending(s: S, h: HartId) -> bool;

} // verus!
