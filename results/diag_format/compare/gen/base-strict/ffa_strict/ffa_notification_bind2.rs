pub open spec fn ffa_notification_bind2_spec(result: u32, sender_receiver_ids: u32, flags: u64, notification_bitmap: u64, old_s: S, new_s: S) -> bool {
    (!IsImplementedAtInstance(FFA_NOTIFICATION_BIND2, CurrentInstance()) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidSenderId(Bits(sender_receiver_ids, 31, 16)) || !IsValidReceiverId(Bits(sender_receiver_ids, 15, 0)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (Bits(flags, 0, 0) == 1 && !PerVcpuNotificationsSupported() ==> ResultEqual(result, INVALID_PARAMETERS))
    && (exists|i: u64| i >= NumSupportedNotifications() && i < 384 && BitmapBit(notification_bitmap, i) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (forall|i: u64| i < 384 ==> BitmapBit(notification_bitmap, i) == 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (exists|i: u64| i < NumSupportedNotifications() && BitmapBit(notification_bitmap, i) == 1 && (IsBoundToOtherSender(Bits(sender_receiver_ids, 15, 0), i, Bits(sender_receiver_ids, 31, 16)) || IsNotificationPending(Bits(sender_receiver_ids, 15, 0), i)) ==> ResultEqual(result, DENIED))
    && (!CallerAllowedToInvoke(FFA_NOTIFICATION_BIND2) ==> ResultEqual(result, DENIED))
    && (PartitionHasAborted(Bits(sender_receiver_ids, 31, 16)) ==> ResultEqual(result, ABORTED))
    && (ResultEqual(result, FFA_SUCCESS) ==> forall|i: u64| i < NumSupportedNotifications() && BitmapBit(notification_bitmap, i) == 1 ==> (NotificationBoundSender(Bits(sender_receiver_ids, 15, 0), i) == Bits(sender_receiver_ids, 31, 16) && NotificationIsPerVcpu(Bits(sender_receiver_ids, 15, 0), i) == (Bits(flags, 0, 0) == 1)))
}