pub open spec fn ffa_notification_info_get_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_NO_DATA ==> true)
    && (result == FFA_SUCCESS ==> true)
}