use vstd::prelude::*;
verus! {

pub type UnsignedLong = u64;

pub struct Sbiret {
    pub success: bool,
    pub error: i64,
    pub value: u64,
}

pub enum Endian {
    Little,
    Big,
}

pub type HartId = u64;

pub struct S {
    pub current_hart: HartId,
    pub shared_mem: Seq<u8>,
}

pub spec const XLEN: int = 64;

pub spec const LITTLE: Endian = Endian::Little;

pub spec const BIG: Endian = Endian::Big;

pub open spec fn SharedMemWords(s: S, offset: int, count: int, width: int, endian: Endian) -> Seq<int>;

pub open spec fn CurrentHart(s: S) -> HartId;

pub open spec fn DebugTriggerStateAndConfig(s: S, hart: HartId, trig_idx: int) -> Seq<int>;

} // verus!
