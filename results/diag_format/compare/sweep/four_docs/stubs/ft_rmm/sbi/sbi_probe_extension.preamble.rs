use vstd::prelude::*;

verus! {

pub type long = i64;

#[allow(non_camel_case_types)]
pub type sbiret = i64;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsExtensionAvailable(s: S, extension_id: long) -> bool;

pub open spec fn IsImplementationDefinedProbeValue(ret: sbiret) -> bool;

} // verus!
