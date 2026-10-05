use vstd::prelude::*;
verus! {

pub struct S {
    pub extensions: Set<int>,
    pub hart_id: int,
}

pub spec const extension_id: int = 0x10;

pub open spec fn SbiExtensionAvailable(s: S, ext_id: int) -> bool;

} // verus!
