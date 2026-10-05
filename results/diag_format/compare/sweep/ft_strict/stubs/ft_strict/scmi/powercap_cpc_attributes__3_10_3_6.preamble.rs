use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt8 = u8;

pub struct CPLi_DESC {
    pub cpli: UInt32,
    pub flags: UInt32,
    pub min_power_cap: UInt32,
    pub max_power_cap: UInt32,
    pub power_cap_step: UInt32,
    pub min_cai: UInt32,
    pub max_cai: UInt32,
    pub cai_step: UInt32,
    pub name: [UInt8; 16],
}

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -2;
pub const NOT_FOUND: Int32 = -4;
pub const OUT_OF_RANGE: Int32 = -6;
#[allow(non_upper_case_globals)]
pub const result: Int32 = 1000;

pub open spec fn IsCommandSupported(s: S, protocol_id: int, message_id: int) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn DomainSupportsCpc(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidCpliDescIndex(s: S, domain_id: UInt32, desc_index: UInt32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> int;

pub open spec fn MaxCpliDescsPerTransport(s: S) -> int;

pub open spec fn NumCpliDescs(s: S, domain_id: UInt32) -> int;

pub open spec fn CpliDescriptor<T>(s: S, domain_id: UInt32, index: T) -> CPLi_DESC;

pub open spec fn CpliSupportsPowerCapChange(s: S, domain_id: UInt32, cpli: UInt32) -> bool;

pub open spec fn IsPowerCapConfigurable(s: S, domain_id: UInt32, cpli: UInt32) -> bool;

pub open spec fn IsCaiConfigurable(s: S, domain_id: UInt32, cpli: UInt32) -> bool;

pub open spec fn PlatformReportsCai(s: S, domain_id: UInt32, cpli: UInt32) -> bool;

pub open spec fn IsNullTerminatedAscii(name: [UInt8; 16], len: int) -> bool;

} // verus!
