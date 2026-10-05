use vstd::prelude::*;
verus! {

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub struct S {
    pub cmd_input_clock_id: u64,
    pub cmd_input_rate_index: u64,
    pub cmd_output_num_rates_flags: u64,
    pub cmd_output_rates: Seq<u64>,
}

} // verus!
