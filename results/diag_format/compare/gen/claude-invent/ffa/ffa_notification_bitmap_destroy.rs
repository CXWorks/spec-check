pub open spec fn ffa_notification_bitmap_destroy_spec(function_id: UInt32, vm_id: UInt32, result: FfaStatusCode, old_s: S, new_s: S) -> bool {
    (!FfaFunctionImplementedAtInstance(old_s, function_id) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && ((FfaFunctionImplementedAtInstance(old_s, function_id)
        && !FfaIsRecognizedVmId(old_s, vm_id & 0xFFFFu32))
        ==> (result == INVALID_PARAMETERS && new_s == old_s))
    && ((FfaFunctionImplementedAtInstance(old_s, function_id)
        && FfaIsRecognizedVmId(old_s, vm_id & 0xFFFFu32)
        && (!FfaNotificationBitmapRegistered(old_s, vm_id & 0xFFFFu32)
            || !FfaNotificationBitmapMasked(old_s, vm_id & 0xFFFFu32)
            || FfaNotificationBitmapPending(old_s, vm_id & 0xFFFFu32)))
        ==> (result == DENIED && new_s == old_s))
    && ((FfaFunctionImplementedAtInstance(old_s, function_id)
        && FfaIsRecognizedVmId(old_s, vm_id & 0xFFFFu32)
        && FfaNotificationBitmapRegistered(old_s, vm_id & 0xFFFFu32)
        && FfaNotificationBitmapMasked(old_s, vm_id & 0xFFFFu32)
        && !FfaNotificationBitmapPending(old_s, vm_id & 0xFFFFu32))
        ==> (result == FFA_SUCCESS
            && !FfaNotificationBitmapRegistered(new_s, vm_id & 0xFFFFu32)))
}
