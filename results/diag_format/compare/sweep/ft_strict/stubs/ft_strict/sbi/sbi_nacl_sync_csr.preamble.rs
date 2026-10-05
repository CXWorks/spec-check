use vstd::prelude::*;

verus! {

// The spec writes `unsigned long` as two words, which is not a valid Rust/Verus type.
// The intended mapping is unsigned long => u64 and long => i64.
// Rewrite `unsigned long` as `unsigned_long` in the signature before type-checking.
pub type unsigned_long = u64;
pub type long = i64;

pub struct S {
    pub hcsr_synced: Map<u64, bool>,
    pub all_synced: bool,
}

pub open spec fn AllOnes() -> unsigned_long;

pub open spec fn AllImplementedHCsrsSynchronized(s: S) -> bool;

pub open spec fn HCsrSynchronized(s: S, csr_num: unsigned_long) -> bool;

} // verus!
