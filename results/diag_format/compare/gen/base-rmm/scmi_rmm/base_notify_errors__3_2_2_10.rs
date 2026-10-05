pub open spec fn base_notify_errors__3_2_2_10_spec(result: Int32, notify_enable: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidNotifyEnable(notify_enable) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> (
        (notify_enable[0] == 1) ==> ErrorNotifyEnabled(new_s)
    ) && (notify_enable[0] == 0) ==> !ErrorNotifyEnabled(new_s))
}