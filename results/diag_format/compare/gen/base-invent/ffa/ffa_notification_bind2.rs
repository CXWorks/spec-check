pub open spec fn ffa_notification_bind2_spec(result: int32, old_s: S, new_s: S) -> bool {
    // Failure: Invalid sender or receiver endpoint ID
    (result == FFA_ERROR_INVALID_PARAMETERS ==> (old_s.sender_id as int < 0 || old_s.sender_id as int > 0xFFFF || old_s.receiver_id as int < 0 || old_s.receiver_id as int > 0xFFFF))
    // Failure: Per-vCPU flag set when Per-vCPU notifications are not supported
    (result == FFA_ERROR_INVALID_PARAMETERS ==> (old_s.flags & 1 != 0 && !old_s.per_vcpu_supported))
    // Failure: Notification set exceeds the supported number of notifications
    (result == FFA_ERROR_INVALID_PARAMETERS ==> (old_s.notification_bitmap_count as int > old_s.max_notifications))
    // Failure: Empty notification bitmap specified
    (result == FFA_ERROR_INVALID_PARAMETERS ==> (old_s.notification_bitmap_count as int == 0))
    // Failure: This function is not implemented at this FF-A instance
    (result == FFA_ERROR_NOT_SUPPORTED ==> !old_s.ffa_notification_bind2_supported)
    // Failure: At least one notification is bound to another Sender or is currently pending
    (result == FFA_ERROR_DENIED ==> (old_s.notification_bound || old_s.notification_pending))
    // Failure: Caller is not allowed to invoke this ABI
    (result == FFA_ERROR_DENIED ==> !old_s.caller_allowed)
    // Failure: Sender partition ran into an unexpected error and has aborted
    (result == FFA_ERROR_ABORTED ==> old_s.sender_aborted)
    // Success: Returns FFA_SUCCESS
    (result == FFA_SUCCESS ==> (old_s.sender_id == new_s.sender_id && old_s.receiver_id == new_s.receiver_id && old_s.flags == new_s.flags && old_s.notification_bitmap == new_s.notification_bitmap))
}