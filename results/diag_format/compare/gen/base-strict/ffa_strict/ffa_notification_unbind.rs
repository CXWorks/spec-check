pub open spec fn ffa_notification_unbind_spec(result: u32, sender_receiver_ids: u32, bitmap_lo: u32, bitmap_hi: u32, old_s: S, new_s: S) -> bool {
    (!IsImplementedAtInstance(FFA_NOTIFICATION_UNBIND, CurrentFfaInstance()) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidPartitionId(Bits(sender_receiver_ids, 31, 16)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidPartitionId(Bits(sender_receiver_ids, 15, 0)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidNotificationBitmap(bitmap_lo, bitmap_hi) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsCallerAllowedToInvoke(FFA_NOTIFICATION_UNBIND) ==> ResultEqual(result, DENIED))
    && (exists|n: NotificationId| IsBitSet(Bitmap64(bitmap_hi, bitmap_lo), n) && IsNotificationBound(Bits(sender_receiver_ids, 15, 0), n) && BoundSender(Bits(sender_receiver_ids, 15, 0), n) != Bits(sender_receiver_ids, 31, 16) ==> ResultEqual(result, DENIED))
    && (exists|n: NotificationId| IsBitSet(Bitmap64(bitmap_hi, bitmap_lo), n) && IsNotificationPending(Bits(sender_receiver_ids, 15, 0), n) ==> ResultEqual(result, DENIED))
    && (HasPartitionAborted(Bits(sender_receiver_ids, 31, 16)) ==> ResultEqual(result, ABORTED))
    && (ResultEqual(result, FFA_SUCCESS) ==> forall|n: NotificationId| IsBitSet(Bitmap64(bitmap_hi, bitmap_lo), n) ==> !CanSenderSignal(Bits(sender_receiver_ids, 31, 16), Bits(sender_receiver_ids, 15, 0), n))
    && (ResultEqual(result, FFA_SUCCESS) ==> forall|n: NotificationId| !IsBitSet(Bitmap64(bitmap_hi, bitmap_lo), n) ==> NotificationBinding(Bits(sender_receiver_ids, 15, 0), n) == PreNotificationBinding(Bits(sender_receiver_ids, 15, 0), n))
}