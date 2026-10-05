use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub enum RsiCommandReturnCode {
    Error,
    NotFound,
    Busy,
}

pub struct Clock {
    pub notify_enable: UInt32,
}

pub struct S {
    pub clocks: Seq<Clock>,
}

pub spec const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());

pub spec const RSI_NOT_FOUND: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::NotFound);

pub open spec fn ClockAt(s: S, clock_id: int) -> Clock;

} // verus!
