pub open spec fn base_notify_errors__3_2_2_10_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == INVALID_PARAMETERS ==> (old_s.notify_enable != 0 && old_s.notify_enable != 1))
    && (result == SUCCESS ==> (old_s.notify_enable == 0 || old_s.notify_enable == 1))
    && (result == SUCCESS ==> new_s.notify_enable == old_s.notify_enable)
}