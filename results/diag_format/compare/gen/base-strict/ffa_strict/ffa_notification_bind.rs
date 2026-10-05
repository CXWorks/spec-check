pub open spec fn ffa_notification_bind_spec(result: u32, old_s: S, new_s: S) -> bool {
    (!IsImplementedAtInstance(FFA_NOTIFICATION_BIND, CurrentFfaInstance()) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidSenderId(old_s.sender_id) || !IsValidReceiverId(old_s.receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (Bits(old_s.flags, 0, 0) == 1 && !PerVcpuNotificationsSupported() ==> ResultEqual(result, INVALID_PARAMETERS))
    && (exists|n: NotificationId| BitmapBitSet(old_s.notification_bitmap_lo, old_s.notification_bitmap_hi, n) && IsBoundToOtherSender(old_s.receiver_id, n, old_s.sender_id) ==> ResultEqual(result, DENIED))
    && (exists|n: NotificationId| BitmapBitSet(old_s.notification_bitmap_lo, old_s.notification_bitmap_hi, n) && IsNotificationPending(old_s.receiver_id, n) ==> ResultEqual(result, DENIED))
    && (!CallerAllowedToInvoke(FFA_NOTIFICATION_BIND) ==> ResultEqual(result, DENIED))
    && (PartitionHasAborted(old_s.sender_id) ==> ResultEqual(result, ABORTED))
    && (ResultEqual(result, FFA_SUCCESS) ==> forall|n: NotificationId| BitmapBitSet(old_s.notification_bitmap_lo, old_s.notification_bitmap_hi, n) ==> IsBoundToSender(new_s.receiver_id, n, new_s.sender_id))
    && (ResultEqual(result, FFA_SUCCESS) ==> forall|n: NotificationId| BitmapBitSet(old_s.notification_bitmap_lo, old_s.notification_bitmap_hi, n) ==> (IsPerVcpuNotification(new_s.receiver_id, n) == (Bits(old_s.flags, 0, 0) == 1)))
}