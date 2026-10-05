pub open spec fn ffa_notification_bind2_spec(
    result: FfaResult,
    old_s: S,
    new_s: S,
    sender_receiver_ids: UInt32,
    flags: UInt64,
    notification_bitmap: Seq<UInt64>,
) -> bool {
    (!FfaFunctionImplementedAtInstance(old_s, 0xC4000094u32)
        ==> FfaResultIsError(result, NOT_SUPPORTED))
    && ((FfaFunctionImplementedAtInstance(old_s, 0xC4000094u32)
        && (!FfaIsValidEndpointId(old_s, ((sender_receiver_ids >> 16u32) & 0xFFFFu32) as UInt32)
            || !FfaIsValidEndpointId(old_s, (sender_receiver_ids & 0xFFFFu32) as UInt32)
            || ((flags & 1u64) == 1u64 && !FfaPerVcpuNotificationsSupported(old_s))
            || FfaNotificationBitmapExceedsSupported(old_s, notification_bitmap)
            || (forall|k: int| 0 <= k < notification_bitmap.len() ==> notification_bitmap[k] == 0u64)))
        ==> FfaResultIsError(result, INVALID_PARAMETERS))
    && ((FfaFunctionImplementedAtInstance(old_s, 0xC4000094u32)
        && FfaIsValidEndpointId(old_s, ((sender_receiver_ids >> 16u32) & 0xFFFFu32) as UInt32)
        && FfaIsValidEndpointId(old_s, (sender_receiver_ids & 0xFFFFu32) as UInt32)
        && !((flags & 1u64) == 1u64 && !FfaPerVcpuNotificationsSupported(old_s))
        && !FfaNotificationBitmapExceedsSupported(old_s, notification_bitmap)
        && (exists|k: int| 0 <= k < notification_bitmap.len() && notification_bitmap[k] != 0u64)
        && (FfaAnyNotificationBoundToOtherSenderOrPending(
                old_s,
                ((sender_receiver_ids >> 16u32) & 0xFFFFu32) as UInt32,
                (sender_receiver_ids & 0xFFFFu32) as UInt32,
                notification_bitmap)
            || !FfaCallerAllowedToInvokeNotificationBind2(
                old_s,
                ((sender_receiver_ids >> 16u32) & 0xFFFFu32) as UInt32,
                (sender_receiver_ids & 0xFFFFu32) as UInt32)))
        ==> FfaResultIsError(result, DENIED))
    && (FfaResultIsError(result, ABORTED)
        ==> FfaSenderPartitionAborted(old_s, ((sender_receiver_ids >> 16u32) & 0xFFFFu32) as UInt32))
    && ((FfaResultIsError(result, NOT_SUPPORTED)
        || FfaResultIsError(result, INVALID_PARAMETERS)
        || FfaResultIsError(result, DENIED))
        ==> new_s == old_s)
    && ((FfaFunctionImplementedAtInstance(old_s, 0xC4000094u32)
        && FfaIsValidEndpointId(old_s, ((sender_receiver_ids >> 16u32) & 0xFFFFu32) as UInt32)
        && FfaIsValidEndpointId(old_s, (sender_receiver_ids & 0xFFFFu32) as UInt32)
        && !((flags & 1u64) == 1u64 && !FfaPerVcpuNotificationsSupported(old_s))
        && !FfaNotificationBitmapExceedsSupported(old_s, notification_bitmap)
        && (exists|k: int| 0 <= k < notification_bitmap.len() && notification_bitmap[k] != 0u64)
        && !FfaAnyNotificationBoundToOtherSenderOrPending(
                old_s,
                ((sender_receiver_ids >> 16u32) & 0xFFFFu32) as UInt32,
                (sender_receiver_ids & 0xFFFFu32) as UInt32,
                notification_bitmap)
        && FfaCallerAllowedToInvokeNotificationBind2(
                old_s,
                ((sender_receiver_ids >> 16u32) & 0xFFFFu32) as UInt32,
                (sender_receiver_ids & 0xFFFFu32) as UInt32)
        && !FfaSenderPartitionAborted(old_s, ((sender_receiver_ids >> 16u32) & 0xFFFFu32) as UInt32))
        ==> (FfaResultIsSuccess(result)
            && FfaNotificationsBoundToSender(
                new_s,
                ((sender_receiver_ids >> 16u32) & 0xFFFFu32) as UInt32,
                (sender_receiver_ids & 0xFFFFu32) as UInt32,
                notification_bitmap,
                (flags & 1u64) == 1u64)
            && FfaNotificationBindingsUnchangedExcept(
                old_s,
                new_s,
                (sender_receiver_ids & 0xFFFFu32) as UInt32,
                notification_bitmap)))
}
