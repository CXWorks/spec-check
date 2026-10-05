use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct Array<T, const K: usize> {
    pub elems: Seq<T>,
}

impl<T, const K: usize> Array<T, K> {
    pub open spec fn spec_index(self, i: int) -> T;
}

pub struct CPLi_DESC {
    pub cpli: UInt32,
    pub flags: UInt32,
    pub min_power_cap: UInt32,
    pub max_power_cap: UInt32,
    pub power_cap_step: UInt32,
    pub min_cai: UInt32,
    pub max_cai: UInt32,
    pub cai_step: UInt32,
    pub name: Seq<u8>,
}

pub struct S {
    pub domain: UInt32,
    pub index: UInt32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1int) as i32;
pub spec const NOT_FOUND: Int32 = (-2int) as i32;
pub spec const OUT_OF_RANGE: Int32 = (-3int) as i32;

pub open spec fn IsCommandSupported(protocol_id: int, message_id: int) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn domain_id(s: S) -> UInt32;

pub open spec fn desc_index(s: S) -> UInt32;

pub open spec fn IsValidPowercapDomain(domain: UInt32) -> bool;

pub open spec fn DomainSupportsCpc(domain: UInt32) -> bool;

pub open spec fn IsValidCpliDescIndex(domain: UInt32, index: UInt32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn N<T, const K: usize>(a: Array<T, K>) -> UInt32;

pub open spec fn MaxCpliDescsPerTransport() -> UInt32;

pub open spec fn NumCpliDescs(domain: UInt32) -> UInt32;

pub open spec fn CpliDescriptor(domain: UInt32, index: int) -> CPLi_DESC;

pub open spec fn CpliSupportsPowerCapChange(domain: UInt32, cpli: UInt32) -> bool;

pub open spec fn IsPowerCapConfigurable(domain: UInt32, cpli: UInt32) -> bool;

pub open spec fn IsCaiConfigurable(domain: UInt32, cpli: UInt32) -> bool;

pub open spec fn PlatformReportsCai(domain: UInt32, cpli: UInt32) -> bool;

pub open spec fn IsNullTerminatedAscii(name: Seq<u8>, len: int) -> bool;

} // verus!
