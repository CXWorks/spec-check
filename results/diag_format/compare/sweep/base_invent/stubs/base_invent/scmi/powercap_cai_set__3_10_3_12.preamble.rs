use vstd::prelude::*;

verus! {

pub type int32 = u32;

pub struct PowercapDomain {
    pub flags: u32,
    pub cai: u32,
    pub cpli: u32,
    pub supports_cpc: bool,
}

pub struct S {
    pub powercap_domains: Seq<PowercapDomain>,
    pub domain_id: int,
}

} // verus!
