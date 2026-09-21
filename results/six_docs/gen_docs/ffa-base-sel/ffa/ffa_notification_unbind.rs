pub open spec fn ffa_notification_unbind_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (old_s.sender_id as int < 0 || old_s.sender_id as int >= 0x1_0000_0000 || old_s.receiver_id as int < 0 || old_s.receiver_id as int >= 0x1_0000_0000 || (old_s.notification_bitmap_lo as int) == 0 && (old_s.notification_bitmap_hi as int) == 0))
    && (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_DENIED ==> true)
    && (result == FFA_ABORTED ==> true)
    && (result == FFA_SUCCESS ==> (old_s.sender_id == new_s.sender_id && old_s.receiver_id == new_s.receiver_id && old_s.notification_bitmap_lo == new_s.notification_bitmap_lo && old_s.notification_bitmap_hi == new_s.notification_bitmap_hi))
}