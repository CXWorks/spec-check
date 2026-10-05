use vstd::prelude::*;
verus! {

pub struct S {
    pub platform_imposes_shutdown_timeout: bool,
}

pub open spec fn PlatformImposesShutdownTimeout(s: S) -> bool;

} // verus!
