use vstd::prelude::*;
verus! {

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -4;

pub struct S {
    pub dummy: u64,
}

pub open spec fn VoltageDomainExists(s: S, domain_id: u32) -> bool;
pub open spec fn VoltageDomainExtendedNameSupported(s: S, domain_id: u32) -> bool;
pub open spec fn IsNullTerminatedAsciiString(name: Seq<u8>, max_len: int) -> bool;
pub open spec fn VoltageDomainExtendedName(s: S, domain_id: u32) -> Seq<u8>;

} // verus!
