pub open spec fn ffa_msg_send_direct_resp2_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (old_s.cmd_input_w1 as int) < 0 || (old_s.cmd_input_w1 as int) >= 0x1_0000_0000_0000 || (old_s.cmd_input_w1 as int) & 0xFFFF_0000_0000 != 0)
    && (result == FFA_DENIED ==> true)
    && (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_ABORTED ==> true)
    && (result == FFA_SUCCESS ==> true)
    && (result != FFA_SUCCESS ==> true)
}