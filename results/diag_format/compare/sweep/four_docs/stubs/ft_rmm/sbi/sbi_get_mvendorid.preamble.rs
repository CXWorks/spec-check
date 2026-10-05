use vstd::prelude::*;

verus! {

pub type SbiLong = i64;

pub struct sbiret {
    pub error: SbiLong,
    pub value: SbiLong,
}

pub struct S {
    pub mvendorid: SbiLong,
}

pub open spec fn IsLegalMvendoridValue(value: SbiLong) -> bool;

} // verus!
