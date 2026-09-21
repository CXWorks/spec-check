pub open spec fn ffa_notification_bind2_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_ERROR_INVALID_PARAMETERS ==>
        (old_s.sender_id == 0 || old_s.receiver_id == 0 ||
         old_s.per_vcpu_flag == 1 && !old_s.per_vcpu_supported ||
         old_s.notification_bitmap_count == 0))
    && (result == FFA_ERROR_NOT_SUPPORTED ==>
        !old_s.ffa_notification_bind2_supported)
    && (result == FFA_ERROR_DENIED ==>
        (exists i, j where i != j && old_s.notification_bitmap[i] == 1 && old_s.notification_bound_to[i] == j ||
         exists i where old_s.notification_bitmap[i] == 1 && old_s.notification_pending[i]))
    && (result == FFA_ERROR_ABORTED ==>
        old_s.sender_partition_aborted)
    && (result == FFA_SUCCESS ==>
        old_s.sender_id == new_s.sender_id &&
        old_s.receiver_id == new_s.receiver_id &&
        old_s.per_vcpu_flag == new_s.per_vcpu_flag &&
        old_s.notification_bitmap_count == new_s.notification_bitmap_count &&
        old_s.notification_bitmap == new_s.notification_bitmap &&
        (forall i where old_s.notification_bitmap[i] == 1 => new_s.notification_bound_to[i] == old_s.sender_id) &&
        (forall i where old_s.notification_bitmap[i] == 1 => !old_s.notification_pending[i]))
}