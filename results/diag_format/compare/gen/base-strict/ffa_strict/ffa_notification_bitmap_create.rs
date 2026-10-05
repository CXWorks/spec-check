pub open spec fn ffa_notification_bitmap_create_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!FunctionImplementedAtInstance(fid) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsRecognizedVmId(Bits(vm_id, 15, 0)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (NotificationBitmapExists(Bits(vm_id, 15, 0)) ==> ResultEqual(result, DENIED))
    && (!NotificationBitmapAllocatable(Bits(vm_id, 15, 0)) ==> ResultEqual(result, NO_MEMORY))
    && (ResultEqual(result, FFA_SUCCESS) ==> NotificationBitmapExists(Bits(vm_id, 15, 0)))
    && (ResultEqual(result, FFA_SUCCESS) ==> AllocatedSpNotificationCount(Bits(vm_id, 15, 0)) == Bits(allocated_count, 8, 0))
}