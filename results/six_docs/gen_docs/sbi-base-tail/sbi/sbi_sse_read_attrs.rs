pub open spec fn sbi_sse_read_attrs_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (old_s.cmd_input_event_id as int < 0 || old_s.cmd_input_event_id as int >= 0x1_0000_0000 || old_s.cmd_input_attr_count == 0))
    && (result == SBI_SBI_ERR_BAD_RANGE ==> (old_s.cmd_input_event_id as int >= 0x1_0000_0000 || old_s.cmd_input_event_id as int < 0))
    && (result == SBI_SBI_ERR_INVALID_ADDRESS ==> (old_s.cmd_input_output_phys_lo as int % (old_s.xlen as int / 8) != 0))
    && (result == SBI_SBI_ERR_NOT_SUPPORTED ==> true)
    && (result == SBI_SBI_ERR_FAILED ==> true)
    && (result == SBI_SBI_SUCCESS ==> (old_s.cmd_input_event_id as int >= 0 && old_s.cmd_input_event_id as int < 0x1_0000_0000 && old_s.cmd_input_attr_count > 0 && old_s.cmd_input_event_id as int < 0x1_0000_0000))
}