pub open spec fn ffa_notification_unbind2_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_ERROR_INVALID_PARAMETERS ==> (
        (old_s.partition_id == 0 || old_s.partition_id > old_s.max_partition_id)
        || (old_s.notification_bitmap == 0)
        || (old_s.notification_bitmap_count > old_s.max_notification_count)
    ))
    && (result == FFA_ERROR_NOT_SUPPORTED ==> (
        !old_s.is_ffa_instance_valid()
    ))
    && (result == FFA_ERROR_DENIED ==> (
        (exists i in 0..old_s.notification_bitmap_count where old_s.notification_bitmap[i] == 1 && (
            old_s.notification_bound_to[i] != old_s.sender_id
            || old_s.notification_pending[i]
        ))
        || !old_s.is_caller_allowed()
    ))
    && (result == FFA_ERROR_ABORTED ==> old_s.sender_partition_aborted)
    && (result == FFA_SUCCESS ==> (
        forall i in 0..old_s.notification_bitmap_count where old_s.notification_bitmap[i] == 1 =>
            new_s.notification_bound_to[i] == 0
        && new_s.notification_pending[i] == false
    ))
}