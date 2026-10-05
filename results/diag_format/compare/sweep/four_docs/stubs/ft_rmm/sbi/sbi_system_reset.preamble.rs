use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub open spec fn SystemIsShutDown(s: S) -> bool;

pub open spec fn SystemIsColdRebooted(s: S) -> bool;

pub open spec fn SystemIsWarmRebooted(s: S) -> bool;

pub open spec fn CallDoesNotReturn(s: S) -> bool;

} // verus!
