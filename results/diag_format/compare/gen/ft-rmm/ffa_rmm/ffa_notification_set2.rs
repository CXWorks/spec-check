pub open spec fn ffa_notification_set2_spec(sender_id: UInt16, receiver_id: UInt16, per_vcpu: UInt, delay_sri: UInt, receiver_vcpu_id: UInt48, bitmap: [UInt64; 6], result: Result<(), FfaStatusCode>, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_SET2, ffa_instance) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsRecognizedPartitionId(old_s, sender_id) || !IsRecognizedPartitionId(old_s, receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidFlags(old_s, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (per_vcpu == 0 && receiver_vcpu_id != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (per_vcpu == 0 && BitmapContainsPerVcpuNotification(old_s, receiver_id, bitmap) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (per_vcpu == 1 && BitmapContainsGlobalNotification(old_s, receiver_id, bitmap) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (per_vcpu == 1 && !PerVcpuNotificationsSupported(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (BitmapExceedsSupportedNotifications(old_s, bitmap) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsEmptyBitmap(old_s, bitmap) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!SenderPermittedToSignalAll(old_s, sender_id, receiver_id, bitmap) ==> ResultEqual(result, DENIED))
  && (!SupportsNotificationReceipt(old_s, receiver_id) ==> ResultEqual(result, DENIED))
  && (HasAborted(old_s, receiver_id) ==> ResultEqual(result, ABORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> per_vcpu == 0 ==> (for all i where bitmap[i] == 1, NotificationSignaled(new_s, receiver_id, i)))
  && (result == FFA_SUCCESS ==> per_vcpu == 1 ==> (for all i where bitmap[i] == 1, NotificationSignaled(new_s, receiver_id, receiver_vcpu_id, i)))
  && ((IsImplementedAtInstance(old_s, FFA_NOTIFICATION_SET2, ffa_instance) &&
       IsRecognizedPartitionId(old_s, sender_id) &&
       IsRecognizedPartitionId(old_s, receiver_id) &&
       IsValidFlags(old_s, flags) &&
       !(per_vcpu == 0 && receiver_vcpu_id != 0) &&
       !(per_vcpu == 0 && BitmapContainsPerVcpuNotification(old_s, receiver_id, bitmap)) &&
       !(per_vcpu == 1 && BitmapContainsGlobalNotification(old_s, receiver_id, bitmap)) &&
       !(per_vcpu == 1 && !PerVcpuNotificationsSupported(old_s)) &&
       !(BitmapExceedsSupportedNotifications(old_s, bitmap)) &&
       !(IsEmptyBitmap(old_s, bitmap)) &&
       SenderPermittedToSignalAll(old_s, sender_id, receiver_id, bitmap) &&
       SupportsNotificationReceipt(old_s, receiver_id) &&
       !(HasAborted(old_s, receiver_id)))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> NotificationSignaled(new_s, receiver_id, 0) == NotificationSignaled(old_s, receiver_id, 0))
  && (result != FFA_SUCCESS
    ==> NotificationSignaled(new_s, receiver_id, receiver_vcpu_id, 0) == NotificationSignaled(old_s, receiver_id, receiver_vcpu_id, 0))
}