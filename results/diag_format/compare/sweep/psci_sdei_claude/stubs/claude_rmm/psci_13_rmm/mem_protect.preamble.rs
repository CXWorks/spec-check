use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type Int = int;

pub struct S {
    pub mem_protect_implemented: bool,
    pub mem_protect_enabled: bool,
}

pub spec const NOT_SUPPORTED: int = -2;

pub open spec fn IsMemProtectImplemented(s: S) -> bool;

pub open spec fn MemProtectEnabled(s: S) -> bool;

pub open spec fn ResultEqual(result: Int, expected: Int) -> bool;

} // verus!
