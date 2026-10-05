use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub protocol_attributes: u32,
    pub num_agents: u32,
    pub num_protocols: u32,
}

} // verus!
