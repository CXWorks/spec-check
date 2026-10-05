use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type RmiStatusCode = u64;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(&self) -> bool;
}

pub struct S {
    pub dummy: int,
}

pub const NOT_FOUND: RmiStatusCode = 1;
pub const SUCCESS: Int32 = 0;

pub open spec fn IsValidPerformanceDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual<A, B>(a: A, b: B) -> bool;

pub open spec fn CurrentPerformanceLimitMax(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn CurrentPerformanceLimitMin(s: S, domain_id: UInt32) -> UInt32;

} // verus!
