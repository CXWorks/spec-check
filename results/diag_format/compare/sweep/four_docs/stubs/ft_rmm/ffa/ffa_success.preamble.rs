use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;

pub struct Invoker {
    pub id: UInt16,
    pub vcpu: UInt16,
}

pub struct S {
    pub delivered_results: Seq<UInt32>,
    pub current_invoker: Invoker,
}

pub open spec fn PreviousInvoker() -> Invoker;

pub open spec fn ResultsDeliveredTo(s: S, invoker: Invoker, results: [UInt32; 6]) -> bool;

} // verus!
