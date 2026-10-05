pub open spec fn ffa_notification_get2_spec(receiver_vcpu_id: UInt16, receiver_endpoint_id: UInt16, flags: UInt64, sp_bitmask: [UInt64; 10], vm_bitmask: [UInt64; 6], spmc_fw_bitmask: UInt64, hyp_fw_bitmask: UInt64, reserved: UInt64, result: Result<(), FfaStatusCode>, error_code: Int32, rsvd: UInt64, sp_bitmap: [UInt64; 10], vm_bitmap: [UInt64; 6], spmc_bitmap: UInt64, hyp_bitmap: UInt64, reserved1: UInt64, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_GET2, 0) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsCallerAllowedToInvoke(old_s, FFA_NOTIFICATION_GET2) ==> ResultEqual(result, DENIED))
  && (!IsRecognizedPartitionId(old_s, receiver_endpoint_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (flags[63..4] != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (0 == NON_SECURE_PHYSICAL && (flags[1] != 0 || flags[3] != 0) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (ExceedsSupportedNotifications(old_s, sp_bitmask) || ExceedsSupportedNotifications(old_s, vm_bitmask) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsEmptyNotificationBitmapSpecified(old_s, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == FFA_SUCCESS64 ==> rsvd == 0)
  && (result == FFA_SUCCESS64 && flags[0] == 1 ==> for each i where sp_bitmask[i] == 0: sp_bitmap[i] == IsPending(new_s, receiver_endpoint_id, receiver_vcpu_id, SP_BITMAP, i))
  && (result == FFA_SUCCESS64 && flags[1] == 1 ==> for each i where vm_bitmask[i] == 0: vm_bitmap[i] == IsPending(new_s, receiver_endpoint_id, receiver_vcpu_id, VM_BITMAP, i))
  && (result == FFA_SUCCESS64 && flags[2] == 1 ==> for each i where spmc_fw_bitmask[i] == 0: spmc_bitmap[i] == IsPending(new_s, receiver_endpoint_id, receiver_vcpu_id, SPMC_FW_BITMAP, i))
  && (result == FFA_SUCCESS64 && flags[3] == 1 ==> for each i where hyp_fw_bitmask[i] == 0: hyp_bitmap[i] == IsPending(new_s, receiver_endpoint_id, receiver_vcpu_id, HYP_FW_BITMAP, i))
  && (result == FFA_SUCCESS64 ==> for each bitmap selected in flags and each i set in its bitmask: NotificationState(new_s, receiver_endpoint_id, receiver_vcpu_id, bitmap, i) == NotificationState(old_s, receiver_endpoint_id, receiver_vcpu_id, bitmap, i))
  && ((IsImplementedAtInstance(old_s, FFA_NOTIFICATION_GET2, 0) &&
       IsCallerAllowedToInvoke(old_s, FFA_NOTIFICATION_GET2) &&
       IsRecognizedPartitionId(old_s, receiver_endpoint_id) &&
       !(flags[63..4] != 0) &&
       !(0 == NON_SECURE_PHYSICAL && (flags[1] != 0 || flags[3] != 0)) &&
       !(ExceedsSupportedNotifications(old_s, sp_bitmask) || ExceedsSupportedNotifications(old_s, vm_bitmask)) &&
       !(IsEmptyNotificationBitmapSpecified(old_s, flags)))
    ==> result == FFA_SUCCESS64)
  && (result != FFA_SUCCESS64
    ==> rsvd == 0)
  && (result != FFA_SUCCESS64
    ==> sp_bitmap[0] == 0)
  && (result != FFA_SUCCESS64
    ==> sp_bitmap[1] == 0)
  && (result != FFA_SUCCESS64
    ==> sp_bitmap[2] == 0)
  && (result != FFA_SUCCESS64
    ==> sp_bitmap[3] == 0)
  && (result != FFA_SUCCESS64
    ==> sp_bitmap[4] == 0)
  && (result != FFA_SUCCESS64
    ==> sp_bitmap[5] == 0)
  && (result != FFA_SUCCESS64
    ==> sp_bitmap[6] == 0)
  && (result != FFA_SUCCESS64
    ==> sp_bitmap[7] == 0)
  && (result != FFA_SUCCESS64
    ==> sp_bitmap[8] == 0)
  && (result != FFA_SUCCESS64
    ==> sp_bitmap[9] == 0)
  && (result != FFA_SUCCESS64
    ==> vm_bitmap[0] == 0)
  && (result != FFA_SUCCESS64
    ==> vm_bitmap[1] == 0)
  && (result != FFA_SUCCESS64
    ==> vm_bitmap[2] == 0)
  && (result != FFA_SUCCESS64
    ==> vm_bitmap[3] == 0)
  && (result != FFA_SUCCESS64
    ==> vm_bitmap[4] == 0)
  && (result != FFA_SUCCESS64
    ==> vm_bitmap[5] == 0)
  && (result != FFA_SUCCESS64
    ==> spmc_bitmap == 0)
  && (result != FFA_SUCCESS64
    ==> hyp_bitmap == 0)
  && (!(result == FFA_SUCCESS64 && (flags[0] == 1)) ==> for each i: sp_bitmap[i] == sp_bitmap[i])
  && (!(result == FFA_SUCCESS64 && (flags[1] == 1)) ==> for each i: vm_bitmap[i] == vm_bitmap[i])
  && (!(result == FFA_SUCCESS64 && (flags[2] == 1)) ==> spmc_bitmap == spmc_bitmap)
  && (!(result == FFA_SUCCESS64 && (flags[3] == 1)) ==> hyp_bitmap == hyp_bitmap)
}