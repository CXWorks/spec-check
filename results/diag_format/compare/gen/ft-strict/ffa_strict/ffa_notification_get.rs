pub open spec fn ffa_notification_get_spec(receiver_id: UInt32, flags: UInt32, result: Result<UInt32, FfaStatusCode>, sp_bitmap_lo: UInt32, sp_bitmap_hi: UInt32, vm_bitmap_lo: UInt32, vm_bitmap_hi: UInt32, spm_bitmap: UInt32, hyp_bitmap: UInt32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_GET, CurrentInstance(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsCallerAllowedToInvoke(old_s, Caller(), FFA_NOTIFICATION_GET) ==> ResultEqual(result, DENIED))
  && (!IsRecognizedPartitionId(old_s, Bits(receiver_id, 15, 0)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 31, 4) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsNonSecurePhysicalInstance(old_s) && (Bits(flags, 1, 1) != 0 || Bits(flags, 3, 3) != 0) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (Bits(flags, 0, 0) == 1 ==> (sp_bitmap_lo == Bits(PendingSpNotifications(new_s, Bits(receiver_id, 15, 0), Bits(receiver_id, 31, 16)), 31, 0) && sp_bitmap_hi == Bits(PendingSpNotifications(new_s, Bits(receiver_id, 15, 0), Bits(receiver_id, 31, 16)), 63, 32)))
  && (Bits(flags, 1, 1) == 1 ==> (vm_bitmap_lo == Bits(PendingVmNotifications(new_s, Bits(receiver_id, 15, 0), Bits(receiver_id, 31, 16)), 31, 0) && vm_bitmap_hi == Bits(PendingVmNotifications(new_s, Bits(receiver_id, 15, 0), Bits(receiver_id, 31, 16)), 63, 32)))
  && (Bits(flags, 2, 2) == 1 ==> spm_bitmap == PendingSpmFrameworkNotifications(new_s, Bits(receiver_id, 15, 0), Bits(receiver_id, 31, 16)))
  && (Bits(flags, 3, 3) == 1 ==> hyp_bitmap == PendingHypervisorFrameworkNotifications(new_s, Bits(receiver_id, 15, 0), Bits(receiver_id, 31, 16)))
  && ((IsImplementedAtInstance(old_s, FFA_NOTIFICATION_GET, CurrentInstance(old_s)) &&
       IsCallerAllowedToInvoke(old_s, Caller(), FFA_NOTIFICATION_GET) &&
       IsRecognizedPartitionId(old_s, Bits(receiver_id, 15, 0)) &&
       !(Bits(flags, 31, 4) != 0) &&
       !(IsNonSecurePhysicalInstance(old_s) && (Bits(flags, 1, 1) != 0 || Bits(flags, 3, 3) != 0)))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> sp_bitmap_lo == 0)
  && (result != FFA_SUCCESS
    ==> sp_bitmap_hi == 0)
  && (result != FFA_SUCCESS
    ==> vm_bitmap_lo == 0)
  && (result != FFA_SUCCESS
    ==> vm_bitmap_hi == 0)
  && (result != FFA_SUCCESS
    ==> spm_bitmap == 0)
  && (result != FFA_SUCCESS
    ==> hyp_bitmap == 0)
}