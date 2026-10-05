use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub open spec fn SystemShutdown(s: S) -> bool;

pub open spec fn SystemColdReboot(s: S) -> bool;

pub open spec fn SystemWarmReboot(s: S) -> bool;

pub open spec fn SupervisorRunsNatively(s: S) -> bool;

pub open spec fn PhysicalPowerDownOfEntireSystem(s: S) -> bool;

pub open spec fn PhysicalPowerCycleOfEntireSystem(s: S) -> bool;

pub open spec fn PowerCycleOfMainProcessorAndPartsOfSystem(s: S) -> bool;

} // verus!
