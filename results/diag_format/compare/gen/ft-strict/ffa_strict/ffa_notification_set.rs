pub open spec fn ffa_notification_set_spec(sender_id: UInt16, receiver_id: UInt16, per_vcpu: UInt, delay_sri: UInt, flags_reserved: UInt, receiver_vcpu_id: UInt16, bitmap_lo: UInt64, bitmap_hi: UInt64, result: Result, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_SET, CallerInstance(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidPartitionId(old_s, sender_id) || !IsValidPartitionId(old_s, receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!AreValidNotificationSetFlags(old_s, per_vcpu, delay_sri, flags_reserved, receiver_vcpu_id, CallerInstance(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (per_vcpu == 0 && receiver_vcpu_id != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (per_vcpu == 0 && (exists|n: UInt64| n < 64 && IsBitSet(old_s, NotificationBitmap(bitmap_lo, bitmap_hi), n) && IsPerVcpuNotification(old_s, receiver_id, n)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (per_vcpu == 1 && (exists|n: UInt64| n < 64 && IsBitSet(old_s, NotificationBitmap(bitmap_lo, bitmap_hi), n) && IsGlobalNotification(old_s, receiver_id, n)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (per_vcpu == 1 && !PerVcpuNotificationsSupported(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (exists|n: UInt64| n < 64 && IsBitSet(old_s, NotificationBitmap(bitmap_lo, bitmap_hi), n) && !IsPermittedToSignal(old_s, sender_id, receiver_id, n) ==> ResultEqual(result, DENIED))
  && (!SupportsNotificationReceipt(old_s, receiver_id) ==> ResultEqual(result, DENIED))
  && (HasAborted(old_s, receiver_id) ==> ResultEqual(result, ABORTED))
  && (result == FFA_SUCCESS ==> result_fid == FFA_SUCCESS)
  && (result == FFA_SUCCESS ==> per_vcpu == 0 ==> (forall|n: UInt64| (n < 64 && IsBitSet(new_s, NotificationBitmap(bitmap_lo, bitmap_hi), n)) ==> IsGlobalNotificationPending(new_s, receiver_id, n)))
  && (result == FFA_SUCCESS ==> per_vcpu == 1 ==> (forall|n: UInt64| (n < 64 && IsBitSet(new_s, NotificationBitmap(bitmap_lo, bitmap_hi), n)) ==> IsPerVcpuNotificationPending(new_s, receiver_id, receiver_vcpu_id, n)))
  && (result == FFA_SUCCESS ==> forall|n: UInt64| (n < 64 && !IsBitSet(new_s, NotificationBitmap(bitmap_lo, bitmap_hi), n)) ==> !IsSignaledByCall(new_s, receiver_id, n))
  && (result == FFA_SUCCESS ==> ScheduleReceiverInterruptPerImplDefinedPolicy(new_s, receiver_id, delay_sri))
  && ((IsImplementedAtInstance(old_s, FFA_NOTIFICATION_SET, CallerInstance(old_s)) &&
       (IsValidPartitionId(old_s, sender_id) && IsValidPartitionId(old_s, receiver_id)) &&
       AreValidNotificationSetFlags(old_s, per_vcpu, delay_sri, flags_reserved, receiver_vcpu_id, CallerInstance(old_s)) &&
       !(per_vcpu == 0 && receiver_vcpu_id != 0) &&
       !(per_vcpu == 0 && (exists|n: UInt64| n < 64 && IsBitSet(old_s, NotificationBitmap(bitmap_lo, bitmap_hi), n) && IsPerVcpuNotification(old_s, receiver_id, n))) &&
       !(per_vcpu == 1 && (exists|n: UInt64| n < 64 && IsBitSet(old_s, NotificationBitmap(bitmap_lo, bitmap_hi), n) && IsGlobalNotification(old_s, receiver_id, n))) &&
       !(per_vcpu == 1 && !PerVcpuNotificationsSupported(old_s)) &&
       !(exists|n: UInt64| n < 64 && IsBitSet(old_s, NotificationBitmap(bitmap_lo, bitmap_hi), n) && !IsPermittedToSignal(old_s, sender_id, receiver_id, n)) &&
       SupportsNotificationReceipt(old_s, receiver_id) &&
       !HasAborted(old_s, receiver_id))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> NotificationPendingState(new_s, receiver_id).global == NotificationPendingState(old_s, receiver_id).global)
  && (result != FFA_SUCCESS
    ==> NotificationPendingState(new_s, receiver_id).per_vcpu == NotificationPendingState(old_s, receiver_id).per_vcpu)
  && (result != FFA_SUCCESS
    ==> ScheduleReceiverInterruptState(new_s) == ScheduleReceiverInterruptState(old_s))
}