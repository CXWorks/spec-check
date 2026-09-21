pub open spec fn ffa_notification_bind_spec(result: int32, old_s: S, new_s: S) -> bool {
    // Failure: Invalid sender or receiver endpoint ID
    (result == FFA_ERROR_INVALID_PARAMETERS ==> (old_s.sender_id < 0 || old_s.sender_id > 0xFFFF || old_s.receiver_id < 0 || old_s.receiver_id > 0xFFFF))
    // Failure: Per-vCPU flag set when Per-vCPU notifications are not supported
    (result == FFA_ERROR_INVALID_PARAMETERS ==> (old_s.flags & 0x1 != 0 && !old_s.per_vcpu_supported))
    // Failure: Function not implemented at this FF-A instance
    (result == FFA_ERROR_NOT_SUPPORTED ==> true)
    // Failure: At least one notification is bound to another Sender or is currently pending
    (result == FFA_ERROR_DENIED ==> (old_s.notifications_bound.contains(old_s.sender_id) || old_s.notifications_pending.contains(old_s.sender_id)))
    // Failure: Caller is not allowed to invoke this ABI
    (result == FFA_ERROR_DENIED ==> !old_s.caller_allowed)
    // Failure: Sender partition ran into an unexpected error and has aborted
    (result == FFA_ERROR_ABORTED ==> old_s.sender_aborted)
    // Success: Returns FFA_SUCCESS
    (result == FFA_SUCCESS ==> (old_s.sender_id >= 0 && old_s.sender_id <= 0xFFFF && old_s.receiver_id >= 0 && old_s.receiver_id <= 0xFFFF && (old_s.flags & 0x1 == 0 || old_s.per_vcpu_supported) && !old_s.notifications_bound.contains(old_s.sender_id) && !old_s.notifications_pending.contains(old_s.sender_id) && old_s.caller_allowed && !old_s.sender_aborted))
}