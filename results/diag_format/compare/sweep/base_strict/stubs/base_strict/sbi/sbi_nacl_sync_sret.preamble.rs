use vstd::prelude::*;

verus! {

// NOTE: This preamble cannot make the function type-check as written.
// The function calls each predicate with zero arguments (e.g. `SretEmulated()`)
// and also with one argument (e.g. `SretEmulated(new_s)`).
// Rust and Verus do not allow overloading, so one function name cannot accept
// both. Each predicate below takes the state `S`. To fix the function, change
// each zero-argument call to pass `old_s`, e.g. `SretEmulated(old_s)`.
// That keeps the intended old-state ==> new-state preservation meaning.

pub type SbiErrorCode = i64;

pub struct S {
    pub nacl_shared_memory_csrs_synchronized: bool,
    pub nacl_shared_memory_hfences_synchronized: bool,
    pub sret_emulated: bool,
    pub returns_to_caller: bool,
}

pub open spec fn NaclSharedMemoryCsrsSynchronized(s: S) -> bool;

pub open spec fn NaclSharedMemoryHfencesSynchronized(s: S) -> bool;

pub open spec fn SretEmulated(s: S) -> bool;

pub open spec fn ReturnsToCaller(s: S) -> bool;

} // verus!
