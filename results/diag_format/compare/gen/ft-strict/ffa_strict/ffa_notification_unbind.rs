pub open spec fn ffa_notification_unbind_spec(sender_receiver_ids: UInt32, sender_id: UInt16, receiver_id: UInt16, bitmap_lo: UInt32, bitmap_hi: UInt32, result: Result<(), FfaStatusCode>, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_UNBIND, CurrentFfaInstance(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidPartitionId(old_s, Bits(sender_receiver_ids, 31, 16)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidPartitionId(old_s, Bits(sender_receiver_ids, 15, 0)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidNotificationBitmap(old_s, bitmap_lo, bitmap_hi) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsCallerAllowedToInvoke(old_s, FFA_NOTIFICATION_UNBIND) ==> ResultEqual(result, DENIED))
  && (exists|n: NotificationId| IsBitSet(old_s, Bitmap64(bitmap_hi, bitmap_lo), n) && IsNotificationBound(old_s, Bits(sender_receiver_ids, 15, 0), n) && BoundSender(old_s, Bits(sender_receiver_ids, 15, 0), n) != Bits(sender_receiver_ids, 31, 16) ==> ResultEqual(result, DENIED))
  && (exists|n: NotificationId| IsBitSet(old_s, Bitmap64(bitmap_hi, bitmap_lo), n) && IsNotificationPending(old_s, Bits(sender_receiver_ids, 15, 0), n) ==> ResultEqual(result, DENIED))
  && (HasPartitionAborted(old_s, Bits(sender_receiver_ids, 31, 16)) ==> ResultEqual(result, ABORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> forall|n: NotificationId| IsBitSet(new_s, Bitmap64(bitmap_hi, bitmap_lo), n) ==> !CanSenderSignal(new_s, Bits(sender_receiver_ids, 31, 16), Bits(sender_receiver_ids, 15, 0), n))
  && (result == FFA_SUCCESS ==> forall|n: NotificationId| !IsBitSet(new_s, Bitmap64(bitmap_hi, bitmap_lo), n) ==> NotificationBinding(new_s, Bits(sender_receiver_ids, 15, 0), n) == PreNotificationBinding(new_s, Bits(sender_receiver_ids, 15, 0), n))
  && ((IsImplementedAtInstance(old_s, FFA_NOTIFICATION_UNBIND, CurrentFfaInstance(old_s)) &&
       IsValidPartitionId(old_s, Bits(sender_receiver_ids, 31, 16)) &&
       IsValidPartitionId(old_s, Bits(sender_receiver_ids, 15, 0)) &&
       IsValidNotificationBitmap(old_s, bitmap_lo, bitmap_hi) &&
       IsCallerAllowedToInvoke(old_s, FFA_NOTIFICATION_UNBIND) &&
       !(exists|n: NotificationId| IsBitSet(old_s, Bitmap64(bitmap_hi, bitmap_lo), n) && IsNotificationBound(old_s, Bits(sender_receiver_ids, 15, 0), n) && BoundSender(old_s, Bits(sender_receiver_ids, 15, 0), n) != Bits(sender_receiver_ids, 31, 16)) &&
       !(exists|n: NotificationId| IsBitSet(old_s, Bitmap64(bitmap_hi, bitmap_lo), n) && IsNotificationPending(old_s, Bits(sender_receiver_ids, 15, 0), n)) &&
       !(HasPartitionAborted(old_s, Bits(sender_receiver_ids, 31, 16))))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> forall|n: NotificationId| CanSenderSignal(new_s, Bits(sender_receiver_ids, 31, 16), Bits(sender_receiver_ids, 15, 0), n) ==> CanSenderSignal(old_s, Bits(sender_receiver_ids, 31, 16), Bits(sender_receiver_ids, 15, 0), n))
  && (result != FFA_SUCCESS
    ==> forall|n: NotificationId| NotificationBinding(new_s, Bits(sender_receiver_ids, 15, 0), n) == NotificationBinding(old_s, Bits(sender_receiver_ids, 15, 0), n))
}