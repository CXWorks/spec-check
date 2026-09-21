pub open spec fn ffa_notification_bind_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (
        (old_s.sender_id < 0) || (old_s.sender_id > 0xFFFF) ||
        (old_s.receiver_id < 0) || (old_s.receiver_id > 0xFFFF) ||
        (old_s.flags & 0x1) != 0
    ))
    && (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_DENIED ==> true)
    && (result == FFA_ABORTED ==> true)
    && (result == FFA_SUCCESS ==> true)
}