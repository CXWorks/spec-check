pub open spec fn ffa_notification_set2_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (
        (old_s.cmd_input_flags & 1 == 0) && (old_s.cmd_input_receiver_vcpu_id != 0)
        || (old_s.cmd_input_flags & 1 == 0) && (old_s.cmd_input_notification_bitmap & 0 != 0)
        || (old_s.cmd_input_flags & 1 == 1) && (old_s.cmd_input_notification_bitmap & 0 == 0)
        || (old_s.cmd_input_flags & 1 == 1) && (old_s.supports_per_vcpu_notifications == false)
        || (old_s.cmd_input_notification_bitmap == 0)
    ))
    && (result == FFA_NOT_SUPPORTED ==> (old_s.supports_ffa_notification_set2 == false))
    && (result == FFA_DENIED ==> (
        (old_s.sender_permitted_to_signal_notifications == false)
        || (old_s.receiver_supports_notifications == false)
    ))
    && (result == FFA_ABORTED ==> (old_s.receiver_partition_aborted == true))
    && (result == FFA_SUCCESS ==> (
        old_s.supports_ffa_notification_set2 == true
        && old_s.supports_per_vcpu_notifications == (old_s.cmd_input_flags & 1 == 1)
        && (old_s.cmd_input_flags & 1 == 0 ==> old_s.cmd_input_receiver_vcpu_id == 0)
        && (old_s.cmd_input_flags & 1 == 1 ==> old_s.cmd_input_notification_bitmap != 0)
        && (old_s.cmd_input_notification_bitmap != 0 ==> (old_s.cmd_input_notification_bitmap & 0 != 0))
        && (old_s.cmd_input_notification_bitmap != 0 ==> (old_s.cmd_input_notification_bitmap & 0 != 0))
    ))
}