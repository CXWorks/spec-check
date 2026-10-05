use vstd::prelude::*;

verus! {

pub type UInt = u64;

pub type UInt64 = u64;

pub type HartId = u64;

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub regs: Map<Register, u64>,
    pub mem: Map<int, u8>,
    pub steal_time_base: Map<HartId, UInt64>,
    pub steal_time_enabled: Map<HartId, bool>,
}

pub enum Register {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    Other(nat),
}

pub spec const a0: Register = Register::A0;

pub spec const a1: Register = Register::A1;

pub spec const a2: Register = Register::A2;

pub open spec fn ShmemPhysAddr(lo: UInt, hi: UInt) -> UInt64;

pub open spec fn CallingHart() -> HartId;

pub open spec fn IsAllOnes(x: UInt) -> bool;

pub open spec fn StealTimeShmemBase(s: S, hart: HartId) -> UInt64;

pub open spec fn StealTimeReportingEnabled(s: S, hart: HartId) -> bool;

pub open spec fn MemByte(s: S, addr: int) -> u8;

pub open spec fn RegUnchanged(old_s: S, reg: Register, new_s: S) -> bool;

} // verus!
