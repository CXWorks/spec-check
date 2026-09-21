pub open spec fn clock_rate_notify_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> clock_id_is_invalid(old_s, clock_id))
    && (result == INVALID_PARAMETERS ==> (notify_enable != 0 || (notify_enable & 0x1) != 0))
    && (result == SUCCESS ==> (notify_enable == 0 || (notify_enable & 0x1) == 0))
}