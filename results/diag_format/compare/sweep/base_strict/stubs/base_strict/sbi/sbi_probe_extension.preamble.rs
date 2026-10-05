use vstd::prelude::*;
verus! {

pub type Int64 = i64;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsExtensionAvailable(extension_id: Int64) -> bool;

pub open spec fn ImplDefinesProbeValue(extension_id: Int64) -> bool;

pub open spec fn ImplProbeValue(extension_id: Int64) -> Int64;

} // verus!
