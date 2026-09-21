pub open spec fn ffa_notification_set_spec(result: int32, old_s: S, new_s: S) -> bool {
    // Extract parameters from registers (w0-w3)
    let function_id: UInt32 = 0x84000081;
    let sender_receiver_ids: UInt32 = (old_s.cmd_input_w1 as int);
    let flags: UInt32 = (old_s.cmd_input_w2 as int);
    let notification_bitmap_low: UInt32 = (old_s.cmd_input_w3 as int);
    let notification_bitmap_high: UInt32 = (old_s.cmd_input_w4 as int);

    // Extract bit fields
    let sender_id: UInt32 = (sender_receiver_ids >> 16) as UInt32;
    let receiver_id: UInt32 = (sender_receiver_ids & 0xFFFF) as UInt32;
    let per_vcpu_flag: bool = (flags & 0x1) != 0;
    let delay_schedule_flag: bool = (flags & 0x2) != 0;
    let receiver_vcpu_id: UInt32 = (flags >> 16) as UInt32;

    // Failure conditions
    // Unrecognized partition ID or invalid flags
    // Per-vCPU notification flag = b’0 and Receiver vCPU ID != 0
    (!per_vcpu_flag && receiver_vcpu_id != 0 ==> result == FFA_INVALID_PARAMETERS)
    // Per-vCPU notification flag = b’0 and a per-vCPU notification is specified in the Notification bitmap
    // (Assuming any bit set in bitmap_low or bitmap_high implies a per-vCPU notification if flag is 0)
    (!per_vcpu_flag && (notification_bitmap_low != 0 || notification_bitmap_high != 0) ==> result == FFA_INVALID_PARAMETERS)
    // Per-vCPU notification flag = b’1 and a global notification is specified in the Notification bitmap
    (per_vcpu_flag && (notification_bitmap_low == 0 && notification_bitmap_high == 0) ==> result == FFA_INVALID_PARAMETERS)
    // Per-vCPU notification flag = b’1 and Per-vCPU notifications are not supported
    // (This is an implementation state check, assumed to be handled by the environment or result == FFA_NOT_SUPPORTED)
    // NOT_SUPPORTED: This function is not implemented at this FF-A instance
    // (Implementation defined, no specific pre-condition on inputs other than instance validity)
    // DENIED: Sender is not permitted to signal at least one notification to the Receiver
    // Receiver does not support receipt of notifications
    // (Implementation defined, no specific pre-condition on inputs other than permission/state)
    // ABORTED: Receiver partition ran into an unexpected error and has aborted
    // (Implementation defined, no specific pre-condition on inputs other than receiver state)

    // Success condition
    // Returns FFA_SUCCESS without any further parameters on successful completion
    // Implies: function_id == 0x84000081
    // Implies: sender_id and receiver_id are valid (checked by implementation)
    // Implies: flags are valid (checked by failure conditions)
    // Implies: notification bitmap has at least one bit set (implied by "one or more set bits")
    (function_id == 0x84000081 &&
     (notification_bitmap_low != 0 || notification_bitmap_high != 0) &&
     (per_vcpu_flag || receiver_vcpu_id == 0) &&
     (if per_vcpu_flag then (notification_bitmap_low != 0 || notification_bitmap_high != 0) else true) &&
     result == FFA_SUCCESS)
}