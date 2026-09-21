pub open spec fn ffa_notification_unbind2_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (
        (old_s.partition_id == 0 || old_s.partition_id > 0x10000) ||
        (old_s.notification_bitmap == 0) ||
        (old_s.notification_bitmap > (1u64 << 384) - 1)
    ))
    && (result == FFA_NOT_SUPPORTED ==> (
        !old_s.is_ffa_notification_unbind2_supported
    ))
    && (result == FFA_DENIED ==> (
        (old_s.notification_bitmap != 0) &&
        (
            (exists i: int, 0 <= i && i < 384 &&
                (old_s.notification_bitmap >> i & 1) == 1 &&
                (old_s.notification_bound_to_sender[i] != 0 || old_s.notification_pending[i])) ||
            (old_s.caller_not_allowed_to_unbind)
        )
    ))
    && (result == FFA_ABORTED ==> (
        old_s.sender_partition_aborted
    ))
    && (result == FFA_SUCCESS ==> (
        (forall i: int, 0 <= i && i < 384 =>
            (old_s.notification_bitmap >> i & 1) == 1 ==> (
                new_s.notification_bitmap == (old_s.notification_bitmap & !(1u64 << i)) &&
                new_s.notification_bound_to_sender[i] == 0 &&
                new_s.notification_pending[i] == 0
            )
        )
    ))
}