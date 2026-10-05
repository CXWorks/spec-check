pub open spec fn ffa_notification_bind_spec(receiver_id: UInt16, sender_id: UInt16, per_vcpu: Bool, bitmap_lo: Bits32, bitmap_hi: Bits32, result: Result, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_BIND) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidEndpointId(old_s, sender_id) || !IsValidEndpointId(old_s, receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (per_vcpu == true && !PerVcpuNotificationsSupported(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Exists(i : bitmap[i] == 1 && (NotificationIsBoundToOtherSender(old_s, receiver_id, i, sender_id) || NotificationIsPending(old_s, receiver_id, i))) ==> ResultEqual(result, DENIED))
  && (!CallerAllowedToInvoke(old_s, FFA_NOTIFICATION_BIND) ==> ResultEqual(result, DENIED))
  && (PartitionHasAborted(old_s, sender_id) ==> ResultEqual(result, ABORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> ForAll(i : bitmap[i] == 1 ==> NotificationBinding(new_s, receiver_id, i).sender == sender_id))
  && (result == FFA_SUCCESS ==> ForAll(i : bitmap[i] == 1 ==> NotificationBinding(new_s, receiver_id, i).per_vcpu == per_vcpu))
  && ((IsImplementedAtInstance(old_s, FFA_NOTIFICATION_BIND) &&
       (IsValidEndpointId(old_s, sender_id) && IsValidEndpointId(old_s, receiver_id)) &&
       !(per_vcpu == true && !PerVcpuNotificationsSupported(old_s)) &&
       !(Exists(i : bitmap[i] == 1 && (NotificationIsBoundToOtherSender(old_s, receiver_id, i, sender_id) || NotificationIsPending(old_s, receiver_id, i)))) &&
       CallerAllowedToInvoke(old_s, FFA_NOTIFICATION_BIND) &&
       !(PartitionHasAborted(old_s, sender_id)))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> ForAll(i : bitmap[i] == 1 ==> NotificationBinding(new_s, receiver_id, i).sender == NotificationBinding(old_s, receiver_id, i).sender))
  && (result != FFA_SUCCESS
    ==> ForAll(i : bitmap[i] == 1 ==> NotificationBinding(new_s, receiver_id, i).per_vcpu == NotificationBinding(old_s, receiver_id, i).per_vcpu))
}