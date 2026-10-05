pub open spec fn ffa_notification_bind2_spec(result: FfaReturn, error_code: Int32, sender_id: UInt16, receiver_id: UInt16, flags: UInt64, notification_bitmap: UInt64, old_s: S, new_s: S) -> bool {
    (!IsImplementedAtInstance(FFA_NOTIFICATION_BIND2, current_instance) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidEndpointId(sender_id) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidEndpointId(receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags[0] == 1 && !PerVcpuNotificationsSupported() ==> ResultEqual(result, INVALID_PARAMETERS))
    && (BitmapExceedsSupportedNotifications(notification_bitmap) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (notification_bitmap == 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (Exists(i in notification_bitmap : IsNotificationBound(receiver_id, i) && NotificationSender(receiver_id, i) != sender_id) ==> ResultEqual(result, DENIED))
    && (Exists(i in notification_bitmap : IsNotificationPending(receiver_id, i)) ==> ResultEqual(result, DENIED))
    && (!IsCallerAllowedToInvoke(FFA_NOTIFICATION_BIND2) ==> ResultEqual(result, DENIED))
    && (HasAborted(sender_id) ==> ResultEqual(result, ABORTED))
    && (ResultEqual(result, FFA_SUCCESS) ==> ForAll(i in notification_bitmap : NotificationSender(new_s, receiver_id, i) == sender_id))
    && (ResultEqual(result, FFA_SUCCESS) ==> ForAll(i in notification_bitmap : IsPerVcpuNotification(new_s, receiver_id, i) == (flags[0] == 1)))
}