pub open spec fn ffa_notification_bitmap_destroy_spec(result: u32, old_s: S, new_s: S) -> bool {
    (!IsRecognizedPartitionId(Bits(old_s, 15, 0)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsImplementedAtInstance(FFA_NOTIFICATION_BITMAP_DESTROY, CurrentFfaInstance()) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsNotificationBitmapRegistered(Bits(old_s, 15, 0)) || !IsNotificationBitmapMasked(Bits(old_s, 15, 0)) || IsNotificationBitmapPending(Bits(old_s, 15, 0)) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, FFA_SUCCESS) ==> !IsNotificationBitmapRegistered(Bits(new_s, 15, 0)))
}