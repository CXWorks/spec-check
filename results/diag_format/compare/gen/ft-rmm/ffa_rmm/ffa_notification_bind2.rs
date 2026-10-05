pub open spec fn ffa_notification_bind2_spec(sender_id: UInt16, receiver_id: UInt16, flags: UInt64, notification_bitmap: Bitmap, result: Result, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_BIND2, current_instance) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidEndpointId(old_s, sender_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidEndpointId(old_s, receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (flags[0] == 1 && !PerVcpuNotificationsSupported(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (BitmapExceedsSupportedNotifications(old_s, notification_bitmap) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (notification_bitmap == 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Exists(i in notification_bitmap : IsNotificationBound(old_s, receiver_id, i) && NotificationSender(old_s, receiver_id, i) != sender_id) ==> ResultEqual(result, DENIED))
  && (Exists(i in notification_bitmap : IsNotificationPending(old_s, receiver_id, i)) ==> ResultEqual(result, DENIED))
  && (!IsCallerAllowedToInvoke(old_s, FFA_NOTIFICATION_BIND2) ==> ResultEqual(result, DENIED))
  && (HasAborted(old_s, sender_id) ==> ResultEqual(result, ABORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> ForAll(i in notification_bitmap : NotificationSender(new_s, receiver_id, i) == sender_id))
  && (result == FFA_SUCCESS ==> ForAll(i in notification_bitmap : IsPerVcpuNotification(new_s, receiver_id, i) == (flags[0] == 1)))
  && ((IsImplementedAtInstance(old_s, FFA_NOTIFICATION_BIND2, current_instance) &&
       IsValidEndpointId(old_s, sender_id) &&
       IsValidEndpointId(old_s, receiver_id) &&
       !(flags[0] == 1 && !PerVcpuNotificationsSupported(old_s)) &&
       !(BitmapExceedsSupportedNotifications(old_s, notification_bitmap)) &&
       !(notification_bitmap == 0) &&
       !(Exists(i in notification_bitmap : IsNotificationBound(old_s, receiver_id, i) && NotificationSender(old_s, receiver_id, i) != sender_id)) &&
       !(Exists(i in notification_bitmap : IsNotificationPending(old_s, receiver_id, i))) &&
       IsCallerAllowedToInvoke(old_s, FFA_NOTIFICATION_BIND2) &&
       !(HasAborted(old_s, sender_id)))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> ForAll(i in notification_bitmap : NotificationSender(new_s, receiver_id, i) == NotificationSender(old_s, receiver_id, i)))
  && (result != FFA_SUCCESS
    ==> ForAll(i in notification_bitmap : IsPerVcpuNotification(new_s, receiver_id, i) == IsPerVcpuNotification(old_s, receiver_id, i)))
}