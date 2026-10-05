use vstd::prelude::*;
verus! {

pub type UInt31 = u32;

pub struct S {
    pub major_revision: nat,
    pub minor_revision: nat,
}

pub enum PsciFunction {
    PsciVersion,
    CpuSuspend,
    CpuOff,
    CpuOn,
    AffinityInfo,
    SystemOff,
    SystemReset,
    PsciFeatures,
}

pub spec const PSCI_VERSION: PsciFunction = PsciFunction::PsciVersion;

pub open spec fn IsImplemented(s: S, f: PsciFunction) -> bool;

pub open spec fn NOT_SUPPORTED(version: UInt31) -> bool;

pub open spec fn Bits(value: UInt31, hi: nat, lo: nat) -> nat;

pub open spec fn ImplementedMajorRevision(s: S) -> nat;

pub open spec fn ImplementedMinorRevision(s: S) -> nat;

pub open spec fn MajorRevision(s: S, version: UInt31) -> nat;

pub open spec fn MinorRevisionLowerThan(s: S, version: UInt31) -> nat;

pub open spec fn IsInRevision(f: PsciFunction, major: nat, minor: nat) -> bool;

pub open spec fn IsCompatibleInRevision(f: PsciFunction, version: UInt31, s: S) -> bool;

} // verus!
