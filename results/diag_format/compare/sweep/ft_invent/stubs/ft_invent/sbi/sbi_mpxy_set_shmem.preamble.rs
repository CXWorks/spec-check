use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub struct Realm {
    pub shmem_phys_lo: UInt64,
    pub shmem_phys_hi: UInt64,
}

pub struct S {
    pub realms: Seq<Realm>,
}

pub open spec fn RealmAt(s: S, idx: int) -> Realm;

} // verus!
