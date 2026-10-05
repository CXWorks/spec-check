pub open spec fn ffa_notification_bitmap_destroy_spec(vm_id: UInt16, result: UInt32, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsRecognizedPartitionId(old_s, vm_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_BITMAP_DESTROY) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!NotificationBitmapRegistered(old_s, vm_id) || !NotificationBitmapMasked(old_s, vm_id) || NotificationBitmapPending(old_s, vm_id) ==> ResultEqual(result, DENIED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> !NotificationBitmapRegistered(new_s, vm_id))
  && ((IsRecognizedPartitionId(old_s, vm_id) &&
       IsImplementedAtInstance(old_s, FFA_NOTIFICATION_BITMAP_DESTROY) &&
       (NotificationBitmapRegistered(old_s, vm_id) &&
        NotificationBitmapMasked(old_s, vm_id) &&
        !NotificationBitmapPending(old_s, vm_id)))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> NotificationBitmapRegistered(new_s, vm_id))
}