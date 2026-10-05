pub open spec fn ffa_notification_bind2_spec(sender_receiver_ids: UInt16, flags: UInt64, notification_bitmap: [UInt64; 6], result: UInt32, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_BIND2, CurrentInstance(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidSenderId(old_s, Bits(sender_receiver_ids, 31, 16)) || !IsValidReceiverId(old_s, Bits(sender_receiver_ids, 15, 0)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 0, 0) == 1 && !PerVcpuNotificationsSupported(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (exists|i: UInt64| i >= NumSupportedNotifications(old_s) && i < 384 && BitmapBit(notification_bitmap, i) == 1 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (forall|i: UInt64| i < 384 ==> BitmapBit(notification_bitmap, i) == 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (exists|i: UInt64| i < NumSupportedNotifications(old_s) && BitmapBit(notification_bitmap, i) == 1 && (IsBoundToOtherSender(old_s, Bits(sender_receiver_ids, 15, 0), i, Bits(sender_receiver_ids, 31, 16)) || IsNotificationPending(old_s, Bits(sender_receiver_ids, 15, 0), i)) ==> ResultEqual(result, DENIED))
  && (!CallerAllowedToInvoke(old_s, FFA_NOTIFICATION_BIND2) ==> ResultEqual(result, DENIED))
  && (PartitionHasAborted(old_s, Bits(sender_receiver_ids, 31, 16)) ==> ResultEqual(result, ABORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> forall|i: UInt64| i < NumSupportedNotifications(old_s) && BitmapBit(notification_bitmap, i) == 1 ==> NotificationBoundSender(new_s, Bits(sender_receiver_ids, 15, 0), i) == Bits(sender_receiver_ids, 31, 16))
  && (result == FFA_SUCCESS ==> forall|i: UInt64| i < NumSupportedNotifications(old_s) && BitmapBit(notification_bitmap, i) == 1 ==> NotificationIsPerVcpu(new_s, Bits(sender_receiver_ids, 15, 0), i) == (Bits(flags, 0, 0) == 1))
  && ((IsImplementedAtInstance(old_s, FFA_NOTIFICATION_BIND2, CurrentInstance(old_s)) &&
       IsValidSenderId(old_s, Bits(sender_receiver_ids, 31, 16)) &&
       IsValidReceiverId(old_s, Bits(sender_receiver_ids, 15, 0)) &&
       !(Bits(flags, 0, 0) == 1 && !PerVcpuNotificationsSupported(old_s)) &&
       !(exists|i: UInt64| i >= NumSupportedNotifications(old_s) && i < 384 && BitmapBit(notification_bitmap, i) == 1) &&
       !(forall|i: UInt64| i < 384 ==> BitmapBit(notification_bitmap, i) == 0) &&
       !(exists|i: UInt64| i < NumSupportedNotifications(old_s) && BitmapBit(notification_bitmap, i) == 1 && (IsBoundToOtherSender(old_s, Bits(sender_receiver_ids, 15, 0), i, Bits(sender_receiver_ids, 31, 16)) || IsNotificationPending(old_s, Bits(sender_receiver_ids, 15, 0), i))) &&
       CallerAllowedToInvoke(old_s, FFA_NOTIFICATION_BIND2) &&
       !PartitionHasAborted(old_s, Bits(sender_receiver_ids, 31, 16)))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> forall|i: UInt64| i < NumSupportedNotifications(old_s) && BitmapBit(notification_bitmap, i) == 1 ==> NotificationBoundSender(new_s, Bits(sender_receiver_ids, 15, 0), i) == NotificationBoundSender(old_s, Bits(sender_receiver_ids, 15, 0), i))
  && (result != FFA_SUCCESS
    ==> forall|i: UInt64| i < NumSupportedNotifications(old_s) && BitmapBit(notification_bitmap, i) == 1 ==> NotificationIsPerVcpu(new_s, Bits(sender_receiver_ids, 15, 0), i) == NotificationIsPerVcpu(old_s, Bits(sender_receiver_ids, 15, 0), i))
}