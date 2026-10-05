use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub open spec fn SbiExtensionAvailable(s: S, extension_id: i64) -> bool;

} // verus!
