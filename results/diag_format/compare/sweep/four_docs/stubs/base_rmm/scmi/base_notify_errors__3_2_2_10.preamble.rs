use vstd::prelude::*;
verus! {

pub type Int32 = i32;

pub type UInt32 = Seq<u32>;

pub struct S {
    pub error_notify_enabled: bool,
}

pub spec const SUCCESS: Int32 = 0;

pub spec const INVALID_PARAMETERS: Int32 = 1;

pub open spec fn IsValidNotifyEnable(notify_enable: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn ErrorNotifyEnabled(s: S) -> bool;

} // verus!
