pub open spec fn ffa_notification_bind2_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (
        (old_s.sender_id as int) < 0 || (old_s.sender_id as int) > 0xFFFF ||
        (old_s.receiver_id as int) < 0 || (old_s.receiver_id as int) > 0xFFFF ||
        (old_s.flags as int) & 0xFFFFFFFE != 0 ||
        (old_s.notification_bitmap as int) == 0 ||
        (old_s.notification_bitmap as int) > (1u64 << 64) - 1
    ))
    && (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_DENIED ==> true)
    && (result == FFA_ABORTED ==> true)
    && (result == FFA_SUCCESS ==> true)
}