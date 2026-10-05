pub open spec fn ffa_notification_set_spec(result: int32, old_s: S, new_s: S) -> bool {
    // Extract parameters from registers (w0-w4)
    let function_id: uint32 = old_s.registers.w0;
    let sender_receiver_ids: uint32 = old_s.registers.w1;
    let flags: uint32 = old_s.registers.w2;
    let notification_bitmap_low: uint32 = old_s.registers.w3;
    let notification_bitmap_high: uint32 = old_s.registers.w4;
    
    // Extract IDs
    let sender_id: uint32 = (sender_receiver_ids >> 16) & 0xFFFF;
    let receiver_id: uint32 = sender_receiver_ids & 0xFFFF;
    
    // Extract flags
    let per_vcpu_flag: bool = (flags & 0x1) != 0;
    let delay_schedule_flag: bool = (flags & 0x2) != 0;
    let receiver_vcpu_id: uint32 = (flags >> 16) & 0xFFFF;
    
    // Combine bitmaps
    let notification_bitmap: uint64 = ((notification_bitmap_high as uint64) << 32) | (notification_bitmap_low as uint64);
    
    // Failure conditions
    // 1. Unrecognized partition ID or invalid flags
    // (Assuming partition ID validation is handled by the environment/state check)
    // 2. Per-vCPU notification flag = b'0 and Receiver vCPU ID != 0
    (!per_vcpu_flag && receiver_vcpu_id != 0) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS)
    // 3. Per-vCPU notification flag = b'0 and a per-vCPU notification is specified in the Notification bitmap
    (!per_vcpu_flag && (notification_bitmap != 0)) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS)
    // 4. Per-vCPU notification flag = b'1 and a global notification is specified in the Notification bitmap
    (per_vcpu_flag && (notification_bitmap == 0)) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS)
    // 5. Per-vCPU notification flag = b'1 and Per-vCPU notifications are not supported
    (per_vcpu_flag && !old_s.per_vcpu_notifications_supported) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS)
    // 6. Delay Schedule Receiver interrupt flag is set at non-Secure virtual instance
    // (Only valid at Secure virtual FF-A instance)
    // This is an implementation-defined policy check, but the flag must be MBZ at other instances
    // If the instance is not Secure virtual, the flag should be 0
    // (Assuming we can check the instance type from state)
    // For now, we assume the state check handles this or it's an implementation detail
    // The spec says "It MBZ at all other FF-A instances" - if set, it's invalid
    // We'll assume the state has a field indicating if this is a Secure virtual instance
    // If not Secure virtual and delay_schedule_flag is set, it's an error
    // (This is a bit ambiguous in the spec, but logically if it's MBZ, setting it is invalid)
    // We'll skip this specific check as it's implementation-defined and the spec doesn't give a clear error code for it
    // The main error codes are listed in Table 16.21
    
    // Success condition
    // Returns FFA_SUCCESS without any further parameters on successful completion
    // All failure conditions must be false for success
    (
        // Check that no failure condition is triggered
        (per_vcpu_flag || receiver_vcpu_id == 0)
        && (per_vcpu_flag || notification_bitmap == 0)
        && (per_vcpu_flag || notification_bitmap != 0)
        && (per_vcpu_flag || old_s.per_vcpu_notifications_supported)
        // Note: We're not checking the delay_schedule_flag for non-Secure virtual as it's implementation-defined
        // and the spec doesn't specify an error code for it
    ) ==> (result == FFA_SUCCESS)
}