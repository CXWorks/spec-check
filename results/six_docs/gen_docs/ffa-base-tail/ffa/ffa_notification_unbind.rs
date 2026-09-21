pub open spec fn ffa_notification_unbind_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (old_s == new_s))
    && (result == FFA_NOT_SUPPORTED ==> (old_s == new_s))
    && (result == FFA_DENIED ==> (old_s == new_s))
    && (result == FFA_ABORTED ==> (old_s == new_s))
    && (result == FFA_SUCCESS ==> (old_s == new_s))
}