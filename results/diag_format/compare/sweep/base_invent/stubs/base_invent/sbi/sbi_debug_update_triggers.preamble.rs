use vstd::prelude::*;

verus! {

pub struct sbiret {
    pub error: i64,
    pub arg0: i64,
}

pub struct DebugTrigger {
    pub r#type: int,
    pub chain: bool,
}

pub struct S {
    pub shared_memory: Seq<int>,
    pub debug_triggers: Seq<DebugTrigger>,
}

} // verus!
