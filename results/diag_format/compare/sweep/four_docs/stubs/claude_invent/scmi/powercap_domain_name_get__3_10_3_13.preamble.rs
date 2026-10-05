use vstd::prelude::*;
verus! {

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -4;

pub struct S {
    pub dummy: int,
}

pub open spec fn PowercapDomainExists(s: S, domain_id: u32) -> bool;
pub open spec fn PowercapDomainExtendedNameSupported(s: S, domain_id: u32) -> bool;
pub open spec fn IsNullTerminatedAsciiString(name: Seq<u8>, max_len: int) -> bool;
pub open spec fn PowercapDomainExtendedName(s: S, domain_id: u32) -> Seq<u8>;

} // verus!
