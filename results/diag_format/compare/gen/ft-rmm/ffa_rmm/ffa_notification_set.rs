pub open spec fn ffa_notification_set_spec(receiver_id: UInt16, sender_id: UInt16, per_vcpu: UInt, delay_sri: UInt, flags_reserved: UInt16, receiver_vcpu_id: UInt16, bitmap_lo: UInt32, bitmap_hi: UInt32, result: Result, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_SET, instance) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsRecognizedPartitionId(old_s, sender_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsRecognizedPartitionId(old_s, receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!AreValidFlags(old_s, flags, instance) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (per_vcpu == 0 && receiver_vcpu_id != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (per_vcpu == 0 && BitmapHasPerVcpuNotification(old_s, receiver_id, bitmap) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (per_vcpu == 1 && BitmapHasGlobalNotification(old_s, receiver_id, bitmap) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (per_vcpu == 1 && !PerVcpuNotificationsSupported(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!SenderMaySignalAll(old_s, sender_id, receiver_id, bitmap) ==> ResultEqual(result, DENIED))
  && (!ReceiverSupportsNotifications(old_s, receiver_id) ==> ResultEqual(result, DENIED))
  && (ReceiverHasAborted(old_s, receiver_id) ==> ResultEqual(result, ABORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> per_vcpu == 0 ==> for each i where bitmap[i] == 1: NotificationSignaled(new_s, receiver_id, i))
  && (result == FFA_SUCCESS ==> per_vcpu == 1 ==> for each i where bitmap[i] == 1: NotificationSignaled(new_s, receiver_id, receiver_vcpu_id, i))
  && ((IsImplementedAtInstance(old_s, FFA_NOTIFICATION_SET, instance) &&
       IsRecognizedPartitionId(old_s, sender_id) &&
       IsRecognizedPartitionId(old_s, receiver_id) &&
       AreValidFlags(old_s, flags, instance) &&
       !(per_vcpu == 0 && receiver_vcpu_id != 0) &&
       !(per_vcpu == 0 && BitmapHasPerVcpuNotification(old_s, receiver_id, bitmap)) &&
       !(per_vcpu == 1 && BitmapHasGlobalNotification(old_s, receiver_id, bitmap)) &&
       !(per_vcpu == 1 && !PerVcpuNotificationsSupported(old_s)) &&
       SenderMaySignalAll(old_s, sender_id, receiver_id, bitmap) &&
       ReceiverSupportsNotifications(old_s, receiver_id) &&
       !ReceiverHasAborted(old_s, receiver_id))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> NotificationState(new_s, receiver_id).global == NotificationState(old_s, receiver_id).global)
  && (result != FFA_SUCCESS
    ==> NotificationState(new_s, receiver_id, receiver_vcpu_id as int).per_vcpu == NotificationState(old_s, receiver_id, receiver_vcpu_id as int).per_vcpu)
  && (result != FFA_SUCCESS
    ==> ScheduleReceiverInterrupt(new_s).pending == ScheduleReceiverInterrupt(old_s).pending)
}