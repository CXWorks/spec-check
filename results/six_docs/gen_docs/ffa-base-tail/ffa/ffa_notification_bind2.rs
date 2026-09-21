pub open spec fn ffa_notification_bind2_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (
        (old_s.sender_id as int) < 0
        || (old_s.sender_id as int) > 0xFFFF
        || (old_s.receiver_id as int) < 0
        || (old_s.receiver_id as int) > 0xFFFF
        || (old_s.per_vcpu as int) == 1 && !old_s.per_vcpu_supported
        || (old_s.notification_count as int) > old_s.max_notifications
        || old_s.notification_count == 0
    ))
    && (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_DENIED ==> true)
    && (result == FFA_ABORTED ==> true)
    && (result == FFA_SUCCESS ==> true)
}