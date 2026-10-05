use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub mem_protect_implemented: bool,
    pub mem_protect_enabled: bool,
}

pub const NOT_SUPPORTED: i32 = -2;

pub open spec fn MemProtectImplemented(s: S) -> bool;

pub open spec fn MemProtectEnabled(s: S) -> bool;

} // verus!
