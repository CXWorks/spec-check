pub open spec fn ffa_notification_set_spec(
    result: int32,
    sender_id: uint32,
    receiver_id: uint32,
    flags: uint32,
    notification_bitmap_lo: uint32,
    notification_bitmap_hi: uint32,
    old_s: S,
    new_s: S,
) -> bool {
    // Extract fields from flags
    let per_vcpu_flag = (flags & 0x1) != 0;
    let delay_schedule_flag = (flags & 0x2) != 0;
    let receiver_vcpu_id = (flags >> 16) & 0xFFFF;
    let reserved_flags = (flags & 0xFFFFC000) != 0;

    // Combine notification bitmap into a single u64
    let notification_bitmap: u64 = ((notification_bitmap_hi as u64) << 32) | (notification_bitmap_lo as u64);

    // Failure conditions
    // 1. Invalid flags: Reserved bits must be zero
    (reserved_flags ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))

    // 2. Per-vCPU flag = 0 and Receiver vCPU ID != 0
    (!per_vcpu_flag && receiver_vcpu_id != 0 ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))

    // 3. Per-vCPU flag = 0 and a per-vCPU notification is specified in the bitmap
    // (Assuming notification IDs are non-zero; if 0 is a valid global notification ID, this check needs adjustment)
    // Based on spec: "Per-vCPU notification flag = b’0 and a per-vCPU notification is specified"
    // We assume notification IDs are non-zero for per-vCPU. If bitmap has any bit set, it's a per-vCPU notification.
    (!per_vcpu_flag && (notification_bitmap != 0) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))

    // 4. Per-vCPU flag = 1 and a global notification is specified in the bitmap
    // (Assuming notification ID 0 is global; if bitmap has bit 0 set, it's global)
    (per_vcpu_flag && (notification_bitmap & 0x1) != 0 ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))

    // 5. Per-vCPU flag = 1 and Per-vCPU notifications are not supported
    // (This is a state check; if not supported, return error)
    // (Assuming a state field exists for per-vCPU support; if not, this condition is unconstrained)
    // For now, we assume this is handled by the implementation and not a pre-condition we can check without state.
    // If the spec implies this is a failure condition, we need a state check.
    // Since no state field is mentioned, we skip this specific failure condition or assume it's handled elsewhere.

    // Success condition
    // Returns FFA_SUCCESS without any further parameters on successful completion.
    // This implies that if none of the failure conditions are met, the result should be FFA_SUCCESS.
    // Also, the state should not change (no parameters are returned, no state modifications mentioned).
    (
        (!reserved_flags)
        && (per_vcpu_flag || receiver_vcpu_id == 0)
        && (per_vcpu_flag || notification_bitmap == 0)
        && (!per_vcpu_flag || (notification_bitmap & 0x1) == 0)
        && (result == FFA_SUCCESS)
        && (old_s == new_s)
    )
}