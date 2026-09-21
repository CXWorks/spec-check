pub open spec fn ffa_notification_set2_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==>
        (
            (old_s.flags & 1u64 == 0u64) && (old_s.receiver_vcpu_id != 0)
            ||
            (old_s.flags & 1u64 == 0u64) && (old_s.notification_bitmap & 0x1 != 0u64)
            ||
            (old_s.flags & 1u64 != 0u64) && (old_s.notification_bitmap & 0x1 == 0u64)
            ||
            (old_s.flags & 1u64 != 0u64) && (old_s.per_vcpu_supported == false)
            ||
            (old_s.notification_bitmap == 0u64)
            ||
            (old_s.notification_bitmap > (1u64 << old_s.supported_notifications))
        ))
    &&
    (result == FFA_NOT_SUPPORTED ==>
        (old_s.function_implemented == false))
    &&
    (result == FFA_DENIED ==>
        (
            (old_s.sender_permitted == false) && (old_s.notification_bitmap != 0u64)
            ||
            (old_s.receiver_supported == false) && (old_s.notification_bitmap != 0u64)
        ))
    &&
    (result == FFA_ABORTED ==>
        (old_s.receiver_aborted == true))
    &&
    (result == FFA_SUCCESS ==>
        (old_s.flags == new_s.flags) &&
        (old_s.sender_id == new_s.sender_id) &&
        (old_s.receiver_id == new_s.receiver_id) &&
        (old_s.notification_bitmap == new_s.notification_bitmap) &&
        (old_s.per_vcpu_supported == new_s.per_vcpu_supported) &&
        (old_s.supported_notifications == new_s.supported_notifications) &&
        (old_s.function_implemented == new_s.function_implemented) &&
        (old_s.sender_permitted == new_s.sender_permitted) &&
        (old_s.receiver_supported == new_s.receiver_supported) &&
        (old_s.receiver_aborted == new_s.receiver_aborted))
}