use vstd::prelude::*;

verus! {

pub type UInt = u64;

pub type Int = int;

pub struct S {
    pub mem_protect_enabled: bool,
    pub mem_protect_implemented: bool,
}

pub spec const NOT_SUPPORTED: int = -2;

pub open spec fn IsMemProtectImplemented(s: S) -> bool;

pub open spec fn ResultEqual(a: int, b: int) -> bool;

pub open spec fn MemProtectEnabled(s: S) -> bool;

pub open spec fn Old(s: S, v: bool) -> bool;

} // verus!
