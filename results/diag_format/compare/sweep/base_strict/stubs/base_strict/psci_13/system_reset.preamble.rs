use vstd::prelude::*;

verus! {

pub type CallerId = u64;

pub struct MachineView {
    pub id: u64,
}

pub struct S {
    pub dummy: u64,
}

#[allow(non_upper_case_globals)]
pub spec const caller: CallerId = 0;

pub open spec fn MachineViewOf(c: CallerId) -> MachineView;

pub open spec fn SystemColdResetPerformed(m: MachineView) -> bool;

pub open spec fn CallIsVirtualized() -> bool;

pub open spec fn SystemPowerCycled() -> bool;

} // verus!
