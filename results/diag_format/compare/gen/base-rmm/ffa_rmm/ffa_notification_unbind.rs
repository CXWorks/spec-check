pub open spec fn ffa_notification_unbind_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsImplementedAtThisInstance(old_s, FFA_NOTIFICATION_UNBIND) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsRecognizedPartitionId(old_s, sender_id(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsRecognizedPartitionId(old_s, receiver_id(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidNotificationBitmap(old_s, bitmap_hi(old_s), bitmap_lo(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (AnyNotificationBoundToOtherSender(old_s, receiver_id(old_s), bitmap_hi(old_s), bitmap_lo(old_s), sender_id(old_s)) ==> ResultEqual(result, DENIED))
    && (AnyNotificationPending(old_s, receiver_id(old_s), bitmap_hi(old_s), bitmap_lo(old_s)) ==> ResultEqual(result, DENIED))
    && (!CallerAllowedToInvoke(old_s, FFA_NOTIFICATION_UNBIND) ==> ResultEqual(result, DENIED))
    && (SenderPartitionAborted(old_s, sender_id(old_s)) ==> ResultEqual(result, ABORTED))
    && (ResultEqual(result, FFA_SUCCESS) ==> !CanSenderSignal(new_s, receiver_id(old_s), sender_id(old_s), n) for each n in SetNotifications(old_s, bitmap_hi(old_s), bitmap_lo(old_s)))
}