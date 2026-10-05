pub open spec fn ffa_notification_bitmap_destroy_spec(vm_id: UInt32, result: Result<UInt32, FfaStatusCode>, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsRecognizedPartitionId(old_s, vm_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_BITMAP_DESTROY, CurrentFfaInstance()) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsNotificationBitmapRegistered(old_s, vm_id) || !IsNotificationBitmapMasked(old_s, vm_id) || IsNotificationBitmapPending(old_s, vm_id) ==> ResultEqual(result, DENIED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> !IsNotificationBitmapRegistered(new_s, vm_id))
  && ((IsRecognizedPartitionId(old_s, vm_id) &&
       IsImplementedAtInstance(old_s, FFA_NOTIFICATION_BITMAP_DESTROY, CurrentFfaInstance()) &&
       (IsNotificationBitmapRegistered(old_s, vm_id) && IsNotificationBitmapMasked(old_s, vm_id) && !IsNotificationBitmapPending(old_s, vm_id)))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> IsNotificationBitmapRegistered(new_s, vm_id))
}