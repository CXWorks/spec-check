pub open spec fn ffa_notification_unbind2_spec(sender: UInt16, receiver: UInt16, bitmap: [UInt64; 6], result: Result<UInt32, Int32>, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_UNBIND2, CurrentFfaInstance(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidPartitionId(old_s, sender) || !IsValidPartitionId(old_s, receiver) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidNotificationBitmap(old_s, bitmap) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (exists|i: UInt64| NotificationBitmapBit(old_s, bitmap, i) == 1 && i >= NumSupportedNotifications(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (forall|i: UInt64| NotificationBitmapBit(old_s, bitmap, i) == 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (exists|i: UInt64| NotificationBitmapBit(old_s, bitmap, i) == 1 && NotificationIsBound(old_s, receiver, i) && NotificationBoundSender(old_s, receiver, i) != sender ==> ResultEqual(result, DENIED))
  && (exists|i: UInt64| NotificationBitmapBit(old_s, bitmap, i) == 1 && NotificationIsPending(old_s, receiver, i) ==> ResultEqual(result, DENIED))
  && (!CallerMayInvokeNotificationUnbind2(old_s, CurrentCaller(old_s)) ==> ResultEqual(result, DENIED))
  && (PartitionHasAborted(old_s, sender) ==> ResultEqual(result, ABORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> forall|i: UInt64| NotificationBitmapBit(old_s, bitmap, i) == 1 ==> !SenderCanSignalNotification(new_s, sender, receiver, i))
  && (result == FFA_SUCCESS ==> forall|i: UInt64| NotificationBitmapBit(old_s, bitmap, i) == 0 ==> NotificationBinding(new_s, receiver, i) == NotificationBinding(old_s, receiver, i))
  && ((IsImplementedAtInstance(old_s, FFA_NOTIFICATION_UNBIND2, CurrentFfaInstance(old_s)) &&
       (IsValidPartitionId(old_s, sender) && IsValidPartitionId(old_s, receiver)) &&
       IsValidNotificationBitmap(old_s, bitmap) &&
       !(exists|i: UInt64| NotificationBitmapBit(old_s, bitmap, i) == 1 && i >= NumSupportedNotifications(old_s)) &&
       !(forall|i: UInt64| NotificationBitmapBit(old_s, bitmap, i) == 0) &&
       !(exists|i: UInt64| NotificationBitmapBit(old_s, bitmap, i) == 1 && NotificationIsBound(old_s, receiver, i) && NotificationBoundSender(old_s, receiver, i) != sender) &&
       !(exists|i: UInt64| NotificationBitmapBit(old_s, bitmap, i) == 1 && NotificationIsPending(old_s, receiver, i)) &&
       CallerMayInvokeNotificationUnbind2(old_s, CurrentCaller(old_s)) &&
       !(PartitionHasAborted(old_s, sender)))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> forall|i: UInt64| NotificationBitmapBit(old_s, bitmap, i) == 1 ==> SenderCanSignalNotification(new_s, sender, receiver, i))
  && (result != FFA_SUCCESS
    ==> forall|i: UInt64| NotificationBitmapBit(old_s, bitmap, i) == 0 ==> NotificationBinding(new_s, receiver, i) == NotificationBinding(old_s, receiver, i))
}