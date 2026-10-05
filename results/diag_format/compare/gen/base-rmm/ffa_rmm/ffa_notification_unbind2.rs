pub open spec fn ffa_notification_unbind2_spec(result: FfaReturnCode, error_code: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidPartitionId(old_s, sender_id(old_s)) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
    && (!IsValidPartitionId(old_s, receiver_id(old_s)) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
    && (!IsValidNotificationBitmap(old_s, bitmap(old_s)) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
    && (ExceedsSupportedNotifications(old_s, bitmap(old_s)) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
    && (IsEmptyBitmap(old_s, bitmap(old_s)) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
    && (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_UNBIND2) ==> ResultEqual(result, FFA_ERROR_NOT_SUPPORTED))
    && (AnyBoundToOtherSender(old_s, receiver_id(old_s), sender_id(old_s), bitmap(old_s)) ==> ResultEqual(result, FFA_ERROR_DENIED))
    && (AnyNotificationPending(old_s, receiver_id(old_s), bitmap(old_s)) ==> ResultEqual(result, FFA_ERROR_DENIED))
    && (!CallerMayInvoke(old_s, FFA_NOTIFICATION_UNBIND2) ==> ResultEqual(result, FFA_ERROR_DENIED))
    && (PartitionAborted(old_s, sender_id(old_s)) ==> ResultEqual(result, FFA_ERROR_ABORTED))
    && (ResultEqual(result, FFA_SUCCESS) ==> !IsBoundToSender(new_s, receiver_id(old_s), bitmap(old_s), sender_id(old_s)))
}