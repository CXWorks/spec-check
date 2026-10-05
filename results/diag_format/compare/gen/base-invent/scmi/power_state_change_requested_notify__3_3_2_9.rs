pub open spec fn power_state_change_requested_notify__3_3_2_9_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> true)
    && (result == -1 ==> true)
    && (result == -2 ==> true)
}