pub open spec fn ffa_partition_info_get_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (old_s.cmd_input_flags & 0x1 != 0 || old_s.cmd_input_uuid == 0))
    && (result == FFA_BUSY ==> (old_s.cmd_input_flags & 0x1 == 0 && (old_s.cmd_input_rx_buffer_free == false || old_s.cmd_input_rx_buffer_mapped == false)))
    && (result == FFA_NO_MEMORY ==> (old_s.cmd_input_flags & 0x1 == 0 && old_s.cmd_input_rx_buffer_size < 1))
    && (result == FFA_DENIED ==> (old_s.cmd_input_flags & 0x1 == 0 && old_s.callee_state != FF_A_STATE_READY))
    && (result == FFA_NOT_SUPPORTED ==> (old_s.cmd_input_function_id != 0x84000068))
    && (result == FFA_NOT_READY ==> (old_s.cmd_input_flags & 0x1 == 0 && old_s.callee_state != FF_A_STATE_READY))
    && (result == FFA_SUCCESS ==> (old_s.cmd_input_flags & 0x1 == 0 && result >= 1 && new_s.cmd_output_count == result && new_s.cmd_output_size == old_s.cmd_input_size))
    && (result == FFA_SUCCESS ==> (old_s.cmd_input_flags & 0x1 == 1 && result >= 1 && new_s.cmd_output_count == result && new_s.cmd_output_size == 0))
}