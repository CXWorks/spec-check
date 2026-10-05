pub open spec fn ffa_notification_unbind2_spec(result: UInt32, bitmap: [UInt64; 8], old_s: S, new_s: S) -> bool {
    (!IsImplementedAtInstance(FFA_NOTIFICATION_UNBIND2, CurrentFfaInstance()) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidPartitionId(sender) || !IsValidPartitionId(receiver) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidNotificationBitmap(bitmap) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (exists|i: UInt64| NotificationBitmapBit(bitmap, i) == 1 && i >= NumSupportedNotifications() ==> ResultEqual(result, INVALID_PARAMETERS))
    && (forall|i: UInt64| NotificationBitmapBit(bitmap, i) == 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (exists|i: UInt64| NotificationBitmapBit(bitmap, i) == 1 && NotificationIsBound(receiver, i) && NotificationBoundSender(receiver, i) != sender ==> ResultEqual(result, DENIED))
    && (exists|i: UInt64| NotificationBitmapBit(bitmap, i) == 1 && NotificationIsPending(receiver, i) ==> ResultEqual(result, DENIED))
    && (!CallerMayInvokeNotificationUnbind2(CurrentCaller()) ==> ResultEqual(result, DENIED))
    && (PartitionHasAborted(sender) ==> ResultEqual(result, ABORTED))
    && (ResultEqual(result, FFA_SUCCESS) ==> forall|i: UInt64| NotificationBitmapBit(bitmap, i) == 1 ==> !SenderCanSignalNotification(sender, receiver, i))
    && (ResultEqual(result, FFA_SUCCESS) ==> forall|i: UInt64| NotificationBitmapBit(bitmap, i) == 0 ==> NotificationBinding(receiver, i) == OldNotificationBinding(receiver, i))
}