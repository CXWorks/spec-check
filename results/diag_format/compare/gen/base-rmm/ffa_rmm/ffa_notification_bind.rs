pub open spec fn ffa_notification_bind_spec(result: UInt32, old_s: S, new_s: S) -> bool {
    (!IsImplementedAtInstance(FFA_NOTIFICATION_BIND) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidEndpointId(sender_id(old_s)) || !IsValidEndpointId(receiver_id(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (per_vcpu(old_s) == 1 && !PerVcpuNotificationsSupported() ==> ResultEqual(result, INVALID_PARAMETERS))
    && (Exists(i : bitmap[i] == 1 && (NotificationIsBoundToOtherSender(receiver_id(old_s), i, sender_id(old_s)) || NotificationIsPending(receiver_id(old_s), i))) ==> ResultEqual(result, DENIED))
    && (!CallerAllowedToInvoke(FFA_NOTIFICATION_BIND) ==> ResultEqual(result, DENIED))
    && (PartitionHasAborted(sender_id(old_s)) ==> ResultEqual(result, ABORTED))
    && (ResultEqual(result, FFA_SUCCESS) ==> ForAll(i : bitmap[i] == 1 ==> NotificationBinding(receiver_id(old_s), i).sender == sender_id(old_s)))
    && (ResultEqual(result, FFA_SUCCESS) ==> ForAll(i : bitmap[i] == 1 ==> NotificationBinding(receiver_id(old_s), i).per_vcpu == per_vcpu(old_s)))
}