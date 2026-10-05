use vstd::prelude::*;
verus! {

pub type Caller = u64;

pub struct MachineView {
    pub id: u64,
}

pub struct S {
    pub virtualized: bool,
    pub power_cycled: bool,
    pub cold_reset: bool,
}

pub open spec fn MachineViewOf(s: S, caller: Caller) -> MachineView;

pub open spec fn SystemColdResetPerformed(s: S, view: MachineView) -> bool;

pub open spec fn CallIsVirtualized(s: S) -> bool;

pub open spec fn SystemPowerCycled(s: S) -> bool;

} // verus!
