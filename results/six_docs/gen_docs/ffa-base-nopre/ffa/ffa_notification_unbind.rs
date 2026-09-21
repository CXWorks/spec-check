pub open spec fn ffa_notification_unbind_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_ERROR_INVALID_PARAMETERS ==> (old_s.sender_id == 0 || old_s.receiver_id == 0 || (old_s.notification_bitmap_lo == 0 && old_s.notification_bitmap_hi == 0)))
    && (result == FFA_ERROR_NOT_SUPPORTED ==> true)
    && (result == FFA_ERROR_DENIED ==> true)
    && (result == FFA_ERROR_ABORTED ==> true)
    && (result == FFA_SUCCESS ==> (old_s.sender_id == new_s.sender_id && old_s.receiver_id == new_s.receiver_id && old_s.notification_bitmap_lo == new_s.notification_bitmap_lo && old_s.notification_bitmap_hi == new_s.notification_bitmap_hi))
}