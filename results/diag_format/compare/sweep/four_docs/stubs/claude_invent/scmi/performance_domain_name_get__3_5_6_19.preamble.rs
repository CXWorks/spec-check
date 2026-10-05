use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -4;

pub open spec fn PerfDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn PerfDomainExtendedNameSupported(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsNullTerminatedAscii(name: Seq<u8>) -> bool;

pub open spec fn PerfDomainExtendedName(s: S, domain_id: UInt32) -> Seq<u8>;

} // verus!
