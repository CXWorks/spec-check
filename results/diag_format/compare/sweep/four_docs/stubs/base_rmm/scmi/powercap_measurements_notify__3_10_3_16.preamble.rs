use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -3;
pub const INVALID_PARAMETERS: Int32 = -2;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn domain_id(s: S) -> UInt32;

pub open spec fn notify_enable(s: S) -> UInt32;

pub open spec fn power_thresh_low(s: S) -> UInt32;

pub open spec fn power_thresh_high(s: S) -> UInt32;

pub open spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn AreValidParameters(s: S, domain_id: UInt32, notify_enable: UInt32, power_thresh_low: UInt32, power_thresh_high: UInt32) -> bool;

pub open spec fn MeasurementsNotifyEnabled(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PowerThreshLow(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PowerThreshHigh(s: S, domain_id: UInt32) -> UInt32;

} // verus!
