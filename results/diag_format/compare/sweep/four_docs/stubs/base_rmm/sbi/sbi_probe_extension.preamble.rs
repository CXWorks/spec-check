use vstd::prelude::*;

verus! {

pub type ExtensionId = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const extension_id: ExtensionId = 0x10;

pub open spec fn IsExtensionAvailable(eid: ExtensionId) -> bool;

pub open spec fn IsImplementationDefinedProbeValue(v: i64) -> bool;

} // verus!
