use vstd::prelude::*;
verus! {

pub struct S {
    pub negotiated_version: u32,
    pub in_use: bool,
    pub downgrade_allowed: bool,
    pub caller_is_spmc_with_separate_spmd: bool,
}

pub open spec fn FfaCalleeImplementsAnyVersion(s: S) -> bool;

pub open spec fn FfaCalleeImplementsVersion(s: S, v: u32) -> bool;

pub open spec fn FfaNegotiatedVersion(s: S) -> u32;

pub open spec fn FfaHighestImplementedVersionBelow(s: S, v: u32) -> u32;

pub open spec fn FfaLowestImplementedVersionAbove(s: S, v: u32) -> u32;

pub open spec fn FfaCallerIsSpmcWithSeparateSpmd(s: S) -> bool;

pub open spec fn FfaInUseByCaller(s: S) -> bool;

pub open spec fn FfaNegotiatedVersionDowngradeAllowed(s: S) -> bool;

pub open spec fn FfaStateUnchangedExceptNegotiatedVersion(old_s: S, new_s: S) -> bool;

} // verus!
