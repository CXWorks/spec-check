pub open spec fn ffa_notification_get2_spec(receiver_id: UInt32, flags: UInt64, sp_bitmask: [UInt64; 6], vm_bitmask: [UInt64; 6], spmc_bitmask: UInt64, hyp_bitmask: UInt64, reserved: UInt64, result: Result<(), FfaStatusCode>, error_code: Int32, sp_bitmap: [UInt64; 6], vm_bitmap: [UInt64; 6], spmc_bitmap: UInt64, hyp_bitmap: UInt64, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_GET2, CurrentInstance(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!CallerAllowedToInvoke(old_s, Caller(old_s), FFA_NOTIFICATION_GET2) ==> ResultEqual(result, DENIED))
  && (!IsRecognizedPartitionId(old_s, Bits(receiver_id, 15, 0)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 63, 4) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (CurrentInstance(old_s) == NS_PHYSICAL && (Bits(flags, 1, 1) != 0 || Bits(flags, 3, 3) != 0) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 0, 0) == 1 && ExceedsSupportedNotifications(old_s, sp_bitmask) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 1, 1) == 1 && ExceedsSupportedNotifications(old_s, vm_bitmask) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (EmptyNotificationBitmapSpecified(old_s, flags, receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == FFA_SUCCESS64 ==> reserved == 0)
  && (result == FFA_SUCCESS64 && Bits(flags, 0, 0) == 1 ==> sp_bitmap == RetrievedNotifications(new_s, PendingSpNotifications(new_s, receiver_id), sp_bitmask))
  && (result == FFA_SUCCESS64 && Bits(flags, 1, 1) == 1 ==> vm_bitmap == RetrievedNotifications(new_s, PendingVmNotifications(new_s, receiver_id), vm_bitmask))
  && (result == FFA_SUCCESS64 && Bits(flags, 2, 2) == 1 ==> spmc_bitmap == RetrievedNotifications(new_s, PendingSpmFrameworkNotifications(new_s, receiver_id), spmc_bitmap))
  && (result == FFA_SUCCESS64 && Bits(flags, 3, 3) == 1 ==> hyp_bitmap == RetrievedNotifications(new_s, PendingHypFrameworkNotifications(new_s, receiver_id), hyp_bitmap))
  && (result == FFA_SUCCESS64 ==> MaskedNotificationsRemainInCurrentState(new_s, receiver_id, sp_bitmask, vm_bitmask, spmc_bitmap, hyp_bitmap))
  && ((IsImplementedAtInstance(old_s, FFA_NOTIFICATION_GET2, CurrentInstance(old_s)) &&
       CallerAllowedToInvoke(old_s, Caller(old_s), FFA_NOTIFICATION_GET2) &&
       IsRecognizedPartitionId(old_s, Bits(receiver_id, 15, 0)) &&
       !(Bits(flags, 63, 4) != 0) &&
       !(CurrentInstance(old_s) == NS_PHYSICAL && (Bits(flags, 1, 1) != 0 || Bits(flags, 3, 3) != 0)) &&
       !(Bits(flags, 0, 0) == 1 && ExceedsSupportedNotifications(old_s, sp_bitmask)) &&
       !(Bits(flags, 1, 1) == 1 && ExceedsSupportedNotifications(old_s, vm_bitmask)) &&
       !(EmptyNotificationBitmapSpecified(old_s, flags, receiver_id)))
    ==> result == FFA_SUCCESS64)
  && (result != FFA_SUCCESS64
    ==> sp_bitmap == [0; 6])
  && (result != FFA_SUCCESS64
    ==> vm_bitmap == [0; 6])
  && (result != FFA_SUCCESS64
    ==> spmc_bitmap == 0)
  && (result != FFA_SUCCESS64
    ==> hyp_bitmap == 0)
  && (!(result == FFA_SUCCESS64 && (Bits(flags, 0, 0) == 0)) ==> sp_bitmap == [0; 6])
}