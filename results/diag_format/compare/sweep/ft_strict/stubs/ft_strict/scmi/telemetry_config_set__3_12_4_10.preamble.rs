use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type RmiStatusCode = u32;

pub const SUCCESS: RmiStatusCode = 0;
pub const INVALID_PARAMETERS: RmiStatusCode = 1;
pub const OUT_OF_RANGE: RmiStatusCode = 2;

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

pub struct S {
    pub dummy: int,
}

pub open spec fn Bits(value: UInt32, high: int, low: int) -> int;

pub open spec fn SignedBits(value: UInt32, high: int, low: int) -> int;

pub open spec fn ProtocolAttributes1() -> UInt32;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn IsValidEventGroup(s: S, group_identifier: UInt32) -> bool;

pub open spec fn AnyDeEnabled(s: S, kind: int, group_identifier: UInt32) -> bool;

pub open spec fn EnabledDeOrGroupLimitReached(s: S, kind: int, group_identifier: UInt32, mode: int) -> bool;

pub open spec fn TelemetryEnabled(s: S, kind: int, group_identifier: UInt32) -> bool;

pub open spec fn AllInterfacesSupportOnDemand(s: S) -> bool;

pub open spec fn TelemetryMode(s: S, kind: int, group_identifier: UInt32) -> int;

pub open spec fn TelemetryDisabledAfterReadingComplete(s: S, kind: int, group_identifier: UInt32) -> bool;

pub open spec fn SamplingRate(s: S, kind: int, group_identifier: UInt32) -> int;

pub open spec fn Pow10(s: S, exponent: int) -> int;

} // verus!
