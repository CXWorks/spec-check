use vstd::prelude::*;
verus! {

pub struct S {
    pub sbi_spec_version_minor: int,
    pub sbi_spec_version_major: int,
}

pub open spec fn SbiSpecVersionMinor(s: S) -> int;

pub open spec fn SbiSpecVersionMajor(s: S) -> int;

} // verus!
