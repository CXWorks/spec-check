use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type PsciFunction = u32;

pub const PSCI_VERSION: PsciFunction = 0x8400_0000u32;

pub const NOT_SUPPORTED: i32 = -1i32;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsImplemented(f: PsciFunction) -> bool;

pub open spec fn ResultEqual(r: UInt32, code: i32) -> bool;

pub open spec fn Bits(v: UInt32, hi: int, lo: int) -> nat;

pub open spec fn ImplementedMajorRevision() -> nat;

pub open spec fn ImplementedMinorRevision() -> nat;

pub open spec fn MajorRevision(v: UInt32) -> nat;

pub open spec fn MinorRevisionLowerThan(v: UInt32) -> nat;

pub open spec fn IsInRevision(f: PsciFunction, major: nat, minor: nat) -> bool;

pub open spec fn IsCompatibleInRevision(f: PsciFunction, v: UInt32) -> bool;

} // verus!
