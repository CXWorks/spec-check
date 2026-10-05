pub open spec fn system_power_state_notify__3_4_2_7_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0)
    && (result == -1 ==> (old_s.notify_enable == 0 || old_s.notify_enable > 1))
    && (result == -2 ==> true)
}