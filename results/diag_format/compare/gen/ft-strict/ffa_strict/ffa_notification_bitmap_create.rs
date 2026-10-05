pub open spec fn ffa_notification_bitmap_create_spec(vm_id: UInt32, vcpu_count: UInt32, notification_count: UInt32, allocated_count: UInt32, result: Result<(), FfaStatusCode>, old_s: S, new_s: S) -> bool {
  (!FunctionImplementedAtInstance(old_s, 0x8400007D) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsRecognizedVmId(old_s, 15, 0) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (NotificationBitmapExists(old_s, 15, 0) ==> ResultEqual(result, DENIED))
  && (!NotificationBitmapAllocatable(old_s, 15, 0) ==> ResultEqual(result, NO_MEMORY))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> NotificationBitmapExists(new_s, 15, 0))
  && (result == FFA_SUCCESS ==> AllocatedSpNotificationCount(new_s, 15, 0) == 8, 0)
  && ((FunctionImplementedAtInstance(old_s, 0x8400007D) &&
       IsRecognizedVmId(old_s, 15, 0) &&
       !NotificationBitmapExists(old_s, 15, 0) &&
       NotificationBitmapAllocatable(old_s, 15, 0))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> !NotificationBitmapExists(new_s, 15, 0))
  && (result != FFA_SUCCESS
    ==> AllocatedSpNotificationCount(new_s, 15, 0) == AllocatedSpNotificationCount(old_s, 15, 0))
}