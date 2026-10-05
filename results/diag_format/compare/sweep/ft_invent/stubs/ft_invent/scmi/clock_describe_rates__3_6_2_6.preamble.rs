use vstd::prelude::*;
verus! {

pub type uint32 = u32;

pub enum RsiCommandReturnCode {
    RsiSuccess,
    RsiErrorInput,
    RsiErrorState,
    RsiIncomplete,
}

pub struct S {
    pub dummy: int,
}

} // verus!
