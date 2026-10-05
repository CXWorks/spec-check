use vstd::prelude::*;

verus! {

pub struct S {
    pub platform_error_fatal: bool,
    pub failed_commands: Map<u32, u32>,
}

pub open spec fn PlatformErrorIsFatal(s: S) -> bool;

pub open spec fn CommandFailedOnPlatform(s: S, message_header: u32, status: u32) -> bool;

} // verus!
