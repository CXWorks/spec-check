use vstd::prelude::*;
verus! {

pub type Int64 = i64;

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsExtensionAvailable(s: S, extension_id: Int64) -> bool;

pub open spec fn ImplDefinesProbeValue(s: S, extension_id: Int64) -> bool;

pub open spec fn ImplProbeValue(s: S, extension_id: Int64) -> Int64;

} // verus!
