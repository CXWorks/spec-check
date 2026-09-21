pub open spec fn clock_describe_rates_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (old_s.cmd_input_clock_id as int < 0 || old_s.cmd_input_clock_id as int >= 0x1_0000_0000 || old_s.cmd_input_rate_index as int < 0 || old_s.cmd_input_rate_index as int >= 0x1_0000_0000))
    && (result == RSI_ERROR_STATE ==> (old_s.cmd_input_clock_id as int >= 0x1_0000_0000))
    && (result == RSI_INCOMPLETE ==> (old_s.cmd_input_rate_index as int >= 0x1_0000_0000))
    && (result == RSI_ERROR_UNKNOWN ==> (old_s.cmd_input_clock_id as int >= 0x1_0000_0000))
    && (result == RSI_SUCCESS ==> (old_s.cmd_input_clock_id as int < 0x1_0000_0000 && old_s.cmd_input_rate_index as int < 0x1_0000_0000))
}