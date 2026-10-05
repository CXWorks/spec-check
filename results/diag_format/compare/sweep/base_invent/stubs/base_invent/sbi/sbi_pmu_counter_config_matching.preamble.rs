use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

impl sbiret {
    pub open spec fn is_Ok(self) -> bool;

    pub open spec fn is_Err(self) -> bool;
}

pub struct S {
    pub cmd_input_counter_idx_base: u64,
    pub cmd_input_counter_idx_mask: u64,
    pub cmd_input_config_flags: int,
    pub cmd_input_event_idx: u64,
    pub cmd_input_event_data: u64,
    pub pmu_counters: Seq<u64>,
}

} // verus!
