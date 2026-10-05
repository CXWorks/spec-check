pub open spec fn ffa_notification_bitmap_create_spec(vm_id: UInt32, vcpu_count: UInt32, notification_count: UInt32, result: FfaCommandReturnCode, notification_count: UInt32, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsRecognizedVmId(old_s, vm_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_BITMAP_CREATE, 0) ==> ResultEqual(result, NOT_SUPPORTED))
  && (NotificationBitmapExists(old_s, vm_id) ==> ResultEqual(result, DENIED))
  && (!CanAllocateNotificationBitmap(old_s, vm_id) ==> ResultEqual(result, NO_MEMORY))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> NotificationBitmapExists(new_s, vm_id))
  && (result == FFA_SUCCESS ==> TotalNotificationCount(new_s, NotificationBitmap(new_s, vm_id)) == notification_count + 64)
  && ((IsRecognizedVmId(old_s, vm_id) &&
       IsImplementedAtInstance(old_s, FFA_NOTIFICATION_BITMAP_CREATE, 0) &&
       !NotificationBitmapExists(old_s, vm_id) &&
       CanAllocateNotificationBitmap(old_s, vm_id))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> !NotificationBitmapExists(new_s, vm_id))
  && (result != FFA_SUCCESS
    ==> TotalNotificationCount(new_s, NotificationBitmap(new_s, vm_id)) == TotalNotificationCount(old_s, NotificationBitmap(old_s, vm_id)))
  && (result != FFA_SUCCESS
    ==> NotificationBitmap(new_s, vm_id) == NotificationBitmap(old_s, vm_id))
}