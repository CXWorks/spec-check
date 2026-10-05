pub open spec fn ffa_notification_get_spec(receiver_vcpu_id: UInt16, receiver_id: UInt16, flags_sp: UInt, flags_vm: UInt, flags_spm: UInt, flags_hyp: UInt, flags_reserved: UInt, result: Result<UInt32, FfaStatusCode>, sp_bitmap_lo: UInt64, sp_bitmap_hi: UInt64, vm_bitmap_lo: UInt64, vm_bitmap_hi: UInt64, spm_bitmap: UInt64, hyp_bitmap: UInt64, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_GET, CurrentInstance(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!CallerMayInvoke(old_s, FFA_NOTIFICATION_GET) ==> ResultEqual(result, DENIED))
  && (!IsRecognizedPartitionId(old_s, receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (flags_reserved != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (CurrentInstance(old_s) == NS_PHYSICAL && (flags_vm != 0 || flags_hyp != 0) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == FFA_SUCCESS ==> sp_bitmap_hi:sp_bitmap_lo) == PendingNotifications(new_s, receiver_id, receiver_vcpu_id, SP))
  && (result == FFA_SUCCESS && flags_vm == 1 ==> (vm_bitmap_hi:vm_bitmap_lo) == PendingNotifications(new_s, receiver_id, receiver_vcpu_id, VM))
  && (result == FFA_SUCCESS && flags_spm == 1 ==> spm_bitmap == PendingNotifications(new_s, receiver_id, receiver_vcpu_id, SPM_FRAMEWORK))
  && (result == FFA_SUCCESS && flags_hyp == 1 ==> hyp_bitmap == PendingNotifications(new_s, receiver_id, receiver_vcpu_id, HYP_FRAMEWORK))
  && ((!(IsImplementedAtInstance(old_s, FFA_NOTIFICATION_GET, CurrentInstance(old_s))) &&
       CallerMayInvoke(old_s, FFA_NOTIFICATION_GET) &&
       IsRecognizedPartitionId(old_s, receiver_id) &&
       !(flags_reserved != 0) &&
       !(CurrentInstance(old_s) == NS_PHYSICAL && (flags_vm != 0 || flags_hyp != 0)))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> sp_bitmap_hi:sp_bitmap_lo) == 0)
  && (result != FFA_SUCCESS
    ==> (vm_bitmap_hi:vm_bitmap_lo) == 0)
  && (result != FFA_SUCCESS
    ==> spm_bitmap == 0)
  && (result != FFA_SUCCESS
    ==> hyp_bitmap == 0)
}