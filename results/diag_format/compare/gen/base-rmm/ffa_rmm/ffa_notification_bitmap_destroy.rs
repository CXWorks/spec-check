pub open spec fn ffa_notification_bitmap_destroy_spec(result: UInt32, old_s: S, new_s: S) -> bool {
    (!IsRecognizedPartitionId(old_s.vm_id) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_BITMAP_DESTROY) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!NotificationBitmapRegistered(old_s, old_s.vm_id) || !NotificationBitmapMasked(old_s, old_s.vm_id) || NotificationBitmapPending(old_s, old_s.vm_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, FFA_SUCCESS) ==> !NotificationBitmapRegistered(new_s, old_s.vm_id))
}