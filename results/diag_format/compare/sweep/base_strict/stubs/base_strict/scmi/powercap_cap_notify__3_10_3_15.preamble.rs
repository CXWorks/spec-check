use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: Int32 = 0;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const NOT_FOUND: Int32 = -4;

pub const domain_id: UInt32 = 0;
pub const notify_enable: UInt32 = 1;
pub const caller: UInt32 = 2;

pub open spec fn IsValidPowercapDomain(s: S, d: UInt32) -> bool;

pub open spec fn IsValidNotifyEnable(s: S, ne: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> int;

pub open spec fn CapChangeNotifyEnabled(agent: UInt32, d: UInt32) -> bool;

} // verus!
