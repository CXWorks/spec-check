pub open spec fn ffa_notification_get_spec(result: FfaReturnCode, old_s: FfaState, new_s: FfaState) -> bool {
    (result == FFA_ERROR_INVALID_PARAMETERS ==> (old_s.receiver_id != new_s.receiver_id || (old_s.flags != new_s.flags)))
    && (result == FFA_ERROR_DENIED ==> (old_s.receiver_id != new_s.receiver_id || (old_s.flags != new_s.flags)))
    && (result == FFA_ERROR_NOT_SUPPORTED ==> (old_s.receiver_id != new_s.receiver_id || (old_s.flags != new_s.flags)))
    && (result == FFA_SUCCESS ==> (old_s.receiver_id == new_s.receiver_id && old_s.flags == new_s.flags))
}