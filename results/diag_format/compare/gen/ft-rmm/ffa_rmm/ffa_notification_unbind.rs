pub open spec fn ffa_notification_unbind_spec(sender_id: UInt16, receiver_id: UInt16, bitmap_lo: UInt32, bitmap_hi: UInt32, result: Result<(), FfaStatusCode>, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtThisInstance(old_s, FFA_NOTIFICATION_UNBIND) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsRecognizedPartitionId(old_s, sender_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsRecognizedPartitionId(old_s, receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidNotificationBitmap(old_s, bitmap_hi, bitmap_lo) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (AnyNotificationBoundToOtherSender(old_s, receiver_id, bitmap_hi, bitmap_lo, sender_id) ==> ResultEqual(result, DENIED))
  && (AnyNotificationPending(old_s, receiver_id, bitmap_hi, bitmap_lo) ==> ResultEqual(result, DENIED))
  && (!CallerAllowedToInvoke(old_s, FFA_NOTIFICATION_UNBIND) ==> ResultEqual(result, DENIED))
  && (SenderPartitionAborted(old_s, sender_id) ==> ResultEqual(result, ABORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> !CanSenderSignal(new_s, receiver_id, sender_id, 0) for each notification 0 set in bitmap_hi:bitmap_lo)
  && ((IsImplementedAtThisInstance(old_s, FFA_NOTIFICATION_UNBIND) &&
       IsRecognizedPartitionId(old_s, sender_id) &&
       IsRecognizedPartitionId(old_s, receiver_id) &&
       IsValidNotificationBitmap(old_s, bitmap_hi, bitmap_lo) &&
       !AnyNotificationBoundToOtherSender(old_s, receiver_id, bitmap_hi, bitmap_lo, sender_id) &&
       !AnyNotificationPending(old_s, receiver_id, bitmap_hi, bitmap_lo) &&
       CallerAllowedToInvoke(old_s, FFA_NOTIFICATION_UNBIND) &&
       !SenderPartitionAborted(old_s, sender_id))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> CanSenderSignal(new_s, receiver_id, sender_id, 0))
}