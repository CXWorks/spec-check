pub open spec fn ffa_notification_bitmap_create_spec(
    vm_id: UInt32,
    vcpu_count: UInt32,
    notification_count: UInt32,
    result: Result<UInt32, FfaErrorCode>,
    old_s: S,
    new_s: S,
) -> bool {
    (!FfaFunctionImplementedAtInstance(old_s, 0x8400007Du32) ==> result.is_Err())
    && (!FfaVmIdIsRecognized(old_s, vm_id & 0xFFFFu32) ==> result.is_Err())
    && (FfaNotificationBitmapExists(old_s, vm_id & 0xFFFFu32) ==> result.is_Err())
    && (!FfaNotificationBitmapMemoryAvailable(old_s, vcpu_count, notification_count & 0x1FFu32) ==> result.is_Err())
    && ((result.is_Err() && result.get_Err_0() == NOT_SUPPORTED)
        ==> !FfaFunctionImplementedAtInstance(old_s, 0x8400007Du32))
    && ((result.is_Err() && result.get_Err_0() == INVALID_PARAMETERS)
        ==> !FfaVmIdIsRecognized(old_s, vm_id & 0xFFFFu32))
    && ((result.is_Err() && result.get_Err_0() == DENIED)
        ==> FfaNotificationBitmapExists(old_s, vm_id & 0xFFFFu32))
    && ((result.is_Err() && result.get_Err_0() == NO_MEMORY)
        ==> !FfaNotificationBitmapMemoryAvailable(old_s, vcpu_count, notification_count & 0x1FFu32))
    && (result.is_Err() ==> (
        result.get_Err_0() == NOT_SUPPORTED
        || result.get_Err_0() == INVALID_PARAMETERS
        || result.get_Err_0() == DENIED
        || result.get_Err_0() == NO_MEMORY
    ))
    && (result.is_Err() ==> new_s == old_s)
    && ((FfaFunctionImplementedAtInstance(old_s, 0x8400007Du32)
        && FfaVmIdIsRecognized(old_s, vm_id & 0xFFFFu32)
        && !FfaNotificationBitmapExists(old_s, vm_id & 0xFFFFu32)
        && FfaNotificationBitmapMemoryAvailable(old_s, vcpu_count, notification_count & 0x1FFu32))
        ==> (result.is_Ok()
            && FfaNotificationBitmapExists(new_s, vm_id & 0xFFFFu32)
            && FfaNotificationBitmapSpNotificationCount(new_s, vm_id & 0xFFFFu32) == (result.get_Ok_0() & 0x1FFu32)))
}
