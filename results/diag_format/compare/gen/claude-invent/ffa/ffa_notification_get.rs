pub open spec fn ffa_notification_get_spec(receiver_id: UInt32, flags: UInt32, result: FfaResult, sp_bitmap_lo: UInt32, sp_bitmap_hi: UInt32, vm_bitmap_lo: UInt32, vm_bitmap_hi: UInt32, spm_bitmap: UInt32, hyp_bitmap: UInt32, old_s: S, new_s: S) -> bool {
    (!IsNotificationGetImplemented(old_s) ==> FfaResultIsError(result) && new_s == old_s)
    && (!IsCallerAllowedNotificationGet(old_s) ==> FfaResultIsError(result) && new_s == old_s)
    && ((!IsValidPartitionId(old_s, receiver_id & 0xFFFFu32)
        || (flags & 0xFFFF_FFF0u32) != 0
        || (IsNonSecurePhysicalInstance(old_s) && (flags & 0xAu32) != 0))
        ==> FfaResultIsError(result) && new_s == old_s)
    && ((!IsNotificationGetImplemented(old_s)
        && IsCallerAllowedNotificationGet(old_s)
        && IsValidPartitionId(old_s, receiver_id & 0xFFFFu32)
        && (flags & 0xFFFF_FFF0u32) == 0
        && !(IsNonSecurePhysicalInstance(old_s) && (flags & 0xAu32) != 0))
        ==> FfaResultErrorEqual(result, NOT_SUPPORTED))
    && ((IsNotificationGetImplemented(old_s)
        && !IsCallerAllowedNotificationGet(old_s)
        && IsValidPartitionId(old_s, receiver_id & 0xFFFFu32)
        && (flags & 0xFFFF_FFF0u32) == 0
        && !(IsNonSecurePhysicalInstance(old_s) && (flags & 0xAu32) != 0))
        ==> FfaResultErrorEqual(result, DENIED))
    && ((IsNotificationGetImplemented(old_s)
        && IsCallerAllowedNotificationGet(old_s)
        && (!IsValidPartitionId(old_s, receiver_id & 0xFFFFu32)
            || (flags & 0xFFFF_FFF0u32) != 0
            || (IsNonSecurePhysicalInstance(old_s) && (flags & 0xAu32) != 0)))
        ==> FfaResultErrorEqual(result, INVALID_PARAMETERS))
    && ((IsNotificationGetImplemented(old_s)
        && IsCallerAllowedNotificationGet(old_s)
        && IsValidPartitionId(old_s, receiver_id & 0xFFFFu32)
        && (flags & 0xFFFF_FFF0u32) == 0
        && !(IsNonSecurePhysicalInstance(old_s) && (flags & 0xAu32) != 0))
        ==> (FfaResultIsSuccess(result)
            && ((flags & 0x1u32) != 0 ==>
                ((((sp_bitmap_hi as u64) << 32u64) | (sp_bitmap_lo as u64))
                    == PendingSpNotifications(old_s, receiver_id & 0xFFFFu32, receiver_id >> 16u32)))
            && ((flags & 0x2u32) != 0 ==>
                ((((vm_bitmap_hi as u64) << 32u64) | (vm_bitmap_lo as u64))
                    == PendingVmNotifications(old_s, receiver_id & 0xFFFFu32, receiver_id >> 16u32)))
            && ((flags & 0x4u32) != 0 ==>
                spm_bitmap == PendingSpmFrameworkNotifications(old_s, receiver_id & 0xFFFFu32, receiver_id >> 16u32))
            && ((flags & 0x8u32) != 0 ==>
                hyp_bitmap == PendingHypervisorFrameworkNotifications(old_s, receiver_id & 0xFFFFu32, receiver_id >> 16u32))))
}
