use vstd::prelude::*;

verus! {

pub struct S {
    pub migrate_info_type: i64,
}

pub open spec fn TrustedOsMigrateInfoType(s: S) -> i64;

} // verus!
