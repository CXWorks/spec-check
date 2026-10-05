use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub enum RmiStatusCode {
    Success,
    ErrorInput,
    ErrorRealm,
    ErrorRec,
    ErrorRtt,
}

pub struct S {
    pub response_virtualized: bool,
    pub power_cycled: bool,
    pub cold_reset: bool,
}

pub type CallerId = u64;

pub struct MachineView {
    pub id: u64,
}

#[allow(non_upper_case_globals)]
pub spec const caller: CallerId = 0;

pub open spec fn IsResponseVirtualized(s: S) -> bool;

pub open spec fn MachineViewOf(c: CallerId) -> MachineView;

pub open spec fn SystemPowerCycled(m: MachineView) -> bool;

pub open spec fn SystemColdResetPerformed(m: MachineView) -> bool;

} // verus!
