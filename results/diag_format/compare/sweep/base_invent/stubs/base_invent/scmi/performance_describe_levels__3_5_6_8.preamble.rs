use vstd::prelude::*;

verus! {

pub type int32 = i32;

pub type uint32 = u32;

pub struct array<T, const N: usize = 0> {
    pub data: Seq<uint32>,
    pub _marker: core::marker::PhantomData<T>,
}

impl<T, const N: usize> array<T, N> {
    pub uninterp spec fn spec_index(self, i: int) -> uint32;

    pub uninterp spec fn len(self) -> nat;
}

pub struct S {
    pub num_domains: uint32,
}

pub const SUCCESS: int32 = 0;

pub const NOT_FOUND: int32 = -3;

pub uninterp spec fn IsDomainValid(s: S, domain_id: uint32) -> bool;

} // verus!
