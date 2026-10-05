use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type RmiStatusCode = u32;

pub const SUCCESS: RmiStatusCode = 0;
pub const NOT_FOUND: RmiStatusCode = 1;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(&self) -> bool {
        self is Ok
    }

    pub open spec fn is_Err(&self) -> bool {
        self is Err
    }
}

pub struct PerfLevel {
    pub entry: Seq<UInt32>,
}

pub type PerfLevels = Seq<PerfLevel>;

pub struct S {
    pub perf_domains: Seq<UInt32>,
}

pub open spec fn IsValidPerfDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn NumEntries(s: S, perf_levels: PerfLevels) -> int;

pub open spec fn NumPerfLevels(s: S, domain_id: UInt32) -> int;

pub open spec fn PerfLevelAtAscendingPosition(s: S, domain_id: UInt32, pos: int) -> PerfLevel;

pub open spec fn IsAscendingByLevelValue(s: S, perf_levels: PerfLevels) -> bool;

} // verus!
