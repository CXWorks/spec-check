pub open spec fn ffa_notification_set_spec(result: int, old_s: S, new_s: S) -> bool {
    let sender_id: u32 = (old_s.cmd_input_0 >> 16) as u32;
    let receiver_id: u32 = (old_s.cmd_input_0) as u32;
    let flags: u32 = old_s.cmd_input_1;
    let bitmap_lo: u32 = old_s.cmd_input_2;
    let bitmap_hi: u32 = old_s.cmd_input_3;
    let bitmap: u64 = ((bitmap_hi as u64) << 32) | (bitmap_lo as u64);
    let per_vcpu_flag: bool = (flags & 1u32) != 0;
    let receiver_vcpu_id: u32 = (flags >> 16) as u32;
    let delay_schedule_flag: bool = (flags & 2u32) != 0;
    let is_valid_bitmap: bool = bitmap != 0;
    let is_per_vcpu_notifications_supported: bool = true; // Context does not define a state field for this; treat as unconstrained for the purpose of this spec.
    let is_per_vcpu_flag_valid: bool = if per_vcpu_flag { is_per_vcpu_notifications_supported } else { true };
    let is_receiver_vcpu_id_valid: bool = if !per_vcpu_flag { receiver_vcpu_id == 0 } else { true };
    let is_per_vcpu_notification_in_bitmap: bool = if per_vcpu_flag { is_valid_bitmap } else { !is_valid_bitmap };
    let is_global_notification_in_bitmap: bool = if !per_vcpu_flag { is_valid_bitmap } else { !is_valid_bitmap };
    let is_per_vcpu_flag_and_global_notification: bool = per_vcpu_flag && is_global_notification_in_bitmap;
    let is_per_vcpu_flag_and_no_per_vcpu_support: bool = per_vcpu_flag && !is_per_vcpu_notifications_supported;
    let is_invalid_parameters: bool = (result == FFA_INVALID_PARAMETERS);
    let is_not_supported: bool = (result == FFA_NOT_SUPPORTED);
    let is_denied: bool = (result == FFA_DENIED);
    let is_aborted: bool = (result == FFA_ABORTED);
    let is_success: bool = (result == FFA_SUCCESS);
    (is_invalid_parameters ==> (
        (!is_per_vcpu_flag_valid) ||
        (!is_receiver_vcpu_id_valid) ||
        (is_per_vcpu_notification_in_bitmap) ||
        (is_per_vcpu_flag_and_global_notification) ||
        (is_per_vcpu_flag_and_no_per_vcpu_support)
    ))
    && (is_not_supported ==> true)
    && (is_denied ==> true)
    && (is_aborted ==> true)
    && (is_success ==> (
        is_per_vcpu_flag_valid &&
        is_receiver_vcpu_id_valid &&
        !is_per_vcpu_notification_in_bitmap &&
        !is_per_vcpu_flag_and_global_notification &&
        !is_per_vcpu_flag_and_no_per_vcpu_support
    ))
}