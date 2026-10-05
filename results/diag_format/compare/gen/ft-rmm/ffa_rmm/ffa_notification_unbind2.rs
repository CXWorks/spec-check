pub open spec fn ffa_notification_unbind2_spec(sender_id: UInt16, receiver_id: UInt16, bitmap: [UInt64; 6], result: FFAFunction, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidPartitionId(old_s, sender_id) || !IsValidPartitionId(old_s, receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidNotificationBitmap(old_s, bitmap) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (ExceedsSupportedNotifications(old_s, bitmap) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsEmptyBitmap(old_s, bitmap) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_UNBIND2) ==> ResultEqual(result, NOT_SUPPORTED))
  && (AnyBoundToOtherSender(old_s, receiver_id, sender_id, bitmap) ==> ResultEqual(result, DENIED))
  && (AnyNotificationPending(old_s, receiver_id, bitmap) ==> ResultEqual(result, DENIED))
  && (!CallerMayInvoke(old_s, FFA_NOTIFICATION_UNBIND2) ==> ResultEqual(result, DENIED))
  && (PartitionAborted(old_s, sender_id) ==> ResultEqual(result, ABORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> (for all i where BitmapBit(bitmap, i) == 1: !IsBoundToSender(new_s, receiver_id, i, sender_id)))
  && ((IsValidPartitionId(old_s, sender_id) &&
       IsValidNotificationBitmap(old_s, bitmap) &&
       !ExceedsSupportedNotifications(old_s, bitmap) &&
       !IsEmptyBitmap(old_s, bitmap) &&
       IsImplementedAtInstance(old_s, FFA_NOTIFICATION_UNBIND2) &&
       !AnyBoundToOtherSender(old_s, receiver_id, sender_id, bitmap) &&
       !AnyNotificationPending(old_s, receiver_id, bitmap) &&
       CallerMayInvoke(old_s, FFA_NOTIFICATION_UNBIND2) &&
       !PartitionAborted(old_s, sender_id))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> result == FFA_ERROR)
  && (result == FFA_SUCCESS
    ==> error_code == 0)
}