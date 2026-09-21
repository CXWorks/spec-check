pub open spec fn 3.4.2.7_system_power_state_notify_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (old_s.notify_enable == 0 || old_s.notify_enable == 1))
    && (result == 1 ==> (old_s.notify_enable == 0 || old_s.notify_enable == 1))
    && (result == -1 ==> (old_s.notify_enable != 0 && old_s.notify_enable != 1))
    && (result == -2 ==> (true))
    && (result == -3 ==> (true))
    && (new_s.notify_enable == old_s.notify_enable)
}