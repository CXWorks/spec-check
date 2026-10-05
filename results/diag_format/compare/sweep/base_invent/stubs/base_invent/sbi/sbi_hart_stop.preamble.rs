use vstd::prelude::*;
verus! {

pub type sbiret = Result<u64, i64>;

pub struct S {
    pub hart_count: u64,
}

} // verus!
