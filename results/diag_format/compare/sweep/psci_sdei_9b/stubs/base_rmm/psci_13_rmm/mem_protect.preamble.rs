use vstd::prelude::*;
verus! {

pub type Int = int;

pub spec const NOT_SUPPORTED: Int = -95;

pub struct S {
    pub mem_protect_enabled: int,
}

impl S {
    pub uninterp spec fn MemProtectEnabled(self) -> int;
}

pub uninterp spec fn IsMemProtectImplemented() -> bool;

pub uninterp spec fn ResultEqual(a: Int, b: Int) -> bool;

pub uninterp spec fn MemProtectEnabled() -> bool;

pub uninterp spec fn Old(x: bool) -> bool;

} // verus!
