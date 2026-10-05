use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub struct S {
    pub cmd_input_domain_id: UInt32,
}

pub open spec fn Bits64(value: int, hi: int, lo: int) -> int;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn PowercapDomainExists(domain_id: int) -> bool;

pub open spec fn MaiChangeNotifySupported(domain_id: int) -> int;

pub open spec fn PowerMeasChangeNotifySupported(domain_id: int) -> int;

pub open spec fn AsyncPowerCapSetSupported(domain_id: int) -> int;

pub open spec fn DomainNameLength(domain_id: int) -> int;

pub open spec fn PowerCapConfigSupported(domain_id: int) -> int;

pub open spec fn PowerMonitoringSupported(domain_id: int) -> int;

pub open spec fn MaiConfigSupported(domain_id: int) -> int;

pub open spec fn HasFastChannel(domain_id: int) -> int;

pub open spec fn PowerCapChangeNotifySupported(domain_id: int) -> int;

pub open spec fn CaiConfigSupported(domain_id: int) -> int;

pub open spec fn CaiChangeNotifySupported(domain_id: int) -> int;

pub open spec fn NameMatchesDomain(name: [UInt8; 16], domain_id: int, extended_name: int) -> bool;

} // verus!
