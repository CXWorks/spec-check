use vstd::prelude::*;
verus! {

pub type UInt = u64;

pub type Address = u64;

pub struct S {
    pub dummy: int,
}

pub open spec fn HartStartRequested(s: S, hartid: UInt, start_addr: Address, opaque: UInt) -> bool;

} // verus!
