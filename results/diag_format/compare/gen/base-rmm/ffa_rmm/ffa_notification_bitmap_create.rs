pub open spec fn ffa_notification_bitmap_create_spec(result: Int32, old_s: S, new_s: S, vm_id: UInt32, vcpu_count: UInt32, notification_count: UInt32) -> bool {
    (!IsRecognizedVmId(vm_id) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsImplementedAtInstance(FFA_NOTIFICATION_BITMAP_CREATE, old_s.instance) ==> ResultEqual(result, NOT_SUPPORTED))
    && (NotificationBitmapExists(vm_id, old_s) ==> ResultEqual(result, DENIED))
    && (!CanAllocateNotificationBitmap(vm_id, old_s) ==> ResultEqual(result, NO_MEMORY))
    && (ResultEqual(result, FFA_SUCCESS) ==> NotificationBitmapExists(vm_id, new_s))
    && (ResultEqual(result, FFA_SUCCESS) ==> TotalNotificationCount(NotificationBitmap(vm_id, new_s)) == notification_count + 64)
}