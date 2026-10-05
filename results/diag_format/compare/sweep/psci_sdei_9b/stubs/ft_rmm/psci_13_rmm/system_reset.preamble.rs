use vstd::prelude::*;
verus! {

pub type Caller = u64;

pub type MachineView = u64;

pub struct S {
    pub dummy: u64,
}

pub open spec fn MachineViewOf(s: S, caller: Caller) -> MachineView;

pub open spec fn SystemColdResetPerformed(s: S, view: MachineView) -> bool;

pub open spec fn IsResponseVirtualized(s: S) -> bool;

pub open spec fn SystemPowerCycled(s: S, view: MachineView) -> bool;

} // verus!
