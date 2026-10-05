use vstd::prelude::*;

verus! {

pub struct S {
    pub dummy: int,
}

pub open spec fn IsMessageImplementedAndAvailable(s: S, message_id: u32) -> bool;

pub open spec fn HasDedicatedFastChannel(s: S, message_id: u32) -> bool;

} // verus!
