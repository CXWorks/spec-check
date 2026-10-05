use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub struct S {
    pub num_power_domains: nat,
    pub power_domain_exists: Seq<bool>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub const domain_id: UInt32 = 0;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn PowerDomainExists(s: S, domain_id: int) -> bool;

pub open spec fn PowerStateChangeNotifySupported(domain_id: int) -> bool;

pub open spec fn PowerStateAsyncSetSupported(domain_id: int) -> bool;

pub open spec fn PowerStateSyncSetSupported(domain_id: int) -> bool;

pub open spec fn PowerStateChangeRequestedNotifySupported(domain_id: int) -> bool;

pub open spec fn PowerDomainNameLength(domain_id: int) -> int;

pub open spec fn PowerDomainName(domain_id: int) -> Seq<UInt8>;

pub open spec fn NullTerminated(s: Seq<UInt8>) -> Seq<UInt8>;

} // verus!
