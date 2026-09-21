pub open spec fn ffa_notification_bind_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (old_s.sender_id < 0 || old_s.sender_id > 0xFFFF || old_s.receiver_id < 0 || old_s.receiver_id > 0xFFFF))
    && (result == FFA_INVALID_PARAMETERS ==> (old_s.flags & 0x1 != 0 && !old_s.per_vcpu_supported))
    && (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_DENIED ==> true)
    && (result == FFA_ABORTED ==> true)
    && (result == FFA_SUCCESS ==> (old_s.sender_id == new_s.sender_id && old_s.receiver_id == new_s.receiver_id && old_s.flags == new_s.flags && old_s.notification_bitmap == new_s.notification_bitmap))
}