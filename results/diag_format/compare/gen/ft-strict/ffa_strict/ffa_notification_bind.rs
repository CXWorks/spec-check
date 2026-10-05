pub open spec fn ffa_notification_bind_spec(receiver_id: UInt16, sender_id: UInt16, flags: UInt32, notification_bitmap_lo: UInt32, notification_bitmap_hi: UInt32, result: Result<(), FfaStatusCode>, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_BIND, CurrentFfaInstance(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidSenderId(old_s, sender_id) || !IsValidReceiverId(old_s, receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 0, 0) == 1 && !PerVcpuNotificationsSupported(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (exists|n: NotificationId| BitmapBitSet(old_s, notification_bitmap_lo, notification_bitmap_hi, n) && IsBoundToOtherSender(old_s, receiver_id, n, sender_id) ==> ResultEqual(result, DENIED))
  && (exists|n: NotificationId| BitmapBitSet(old_s, notification_bitmap_lo, notification_bitmap_hi, n) && IsNotificationPending(old_s, receiver_id, n) ==> ResultEqual(result, DENIED))
  && (!CallerAllowedToInvoke(old_s, FFA_NOTIFICATION_BIND) ==> ResultEqual(result, DENIED))
  && (PartitionHasAborted(old_s, sender_id) ==> ResultEqual(result, ABORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> forall|n: NotificationId| BitmapBitSet(new_s, notification_bitmap_lo, notification_bitmap_hi, n) ==> IsBoundToSender(new_s, receiver_id, n, sender_id))
  && (result == FFA_SUCCESS ==> forall|n: NotificationId| BitmapBitSet(new_s, notification_bitmap_lo, notification_bitmap_hi, n) ==> (IsPerVcpuNotification(new_s, receiver_id, n) == (Bits(flags, 0, 0) == 1)))
  && ((IsImplementedAtInstance(old_s, FFA_NOTIFICATION_BIND, CurrentFfaInstance(old_s)) &&
       (IsValidSenderId(old_s, sender_id) && IsValidReceiverId(old_s, receiver_id)) &&
       !((Bits(flags, 0, 0) == 1) && !PerVcpuNotificationsSupported(old_s)) &&
       !(exists|n: NotificationId| BitmapBitSet(old_s, notification_bitmap_lo, notification_bitmap_hi, n) && IsBoundToOtherSender(old_s, receiver_id, n, sender_id)) &&
       !(exists|n: NotificationId| BitmapBitSet(old_s, notification_bitmap_lo, notification_bitmap_hi, n) && IsNotificationPending(old_s, receiver_id, n)) &&
       CallerAllowedToInvoke(old_s, FFA_NOTIFICATION_BIND) &&
       !PartitionHasAborted(old_s, sender_id))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> forall|n: NotificationId| BitmapBitSet(new_s, notification_bitmap_lo, notification_bitmap_hi, n) ==> !(IsBoundToSender(new_s, receiver_id, n, sender_id)))
  && (result != FFA_SUCCESS
    ==> forall|n: NotificationId| BitmapBitSet(new_s, notification_bitmap_lo, notification_bitmap_hi, n) ==> !(IsPerVcpuNotification(new_s, receiver_id, n) == (Bits(flags, 0, 0) == 1)))
  && (result != FFA_SUCCESS
    ==> IsBoundToSender(new_s, receiver_id, 0, sender_id) == IsBoundToSender(old_s, receiver_id, 0, sender_id))
}