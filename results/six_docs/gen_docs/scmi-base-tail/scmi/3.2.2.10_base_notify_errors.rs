pub open spec fn base_notify_errors_spec(result: int32, notify_enable: UInt32, old_s: S, new_s: S) -> bool {
    (notify_enable != 0 && notify_enable != 1 ==> result == SCMI_INVALID_PARAMETERS)
    && (result == SCMI_SUCCESS ==> true)
}