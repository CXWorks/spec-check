pub open spec fn ffa_notification_get_spec(result: UInt32, sp_bitmap_lo: UInt32, sp_bitmap_hi: UInt32, vm_bitmap_lo: UInt32, vm_bitmap_hi: UInt32, spm_bitmap: UInt32, hyp_bitmap: UInt32, error_code: Int32, old_s: S, new_s: S) -> bool {
    (!IsImplementedAtInstance(FFA_NOTIFICATION_GET, CurrentInstance()) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsCallerAllowedToInvoke(Caller(), FFA_NOTIFICATION_GET) ==> ResultEqual(result, DENIED))
    && (!IsRecognizedPartitionId(Bits(receiver_id(old_s), 15, 0)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (Bits(flags(old_s), 31, 4) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (IsNonSecurePhysicalInstance(CurrentInstance()) && (Bits(flags(old_s), 1, 1) != 0 || Bits(flags(old_s), 3, 3) != 0) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, FFA_SUCCESS) ==> (
        (Bits(flags(old_s), 0, 0) == 1 ==> (sp_bitmap_lo == Bits(PendingSpNotifications(Bits(receiver_id(old_s), 15, 0), Bits(receiver_id(old_s), 31, 16)), 31, 0) && sp_bitmap_hi == Bits(PendingSpNotifications(Bits(receiver_id(old_s), 15, 0), Bits(receiver_id(old_s), 31, 16)), 63, 32)))
        && (Bits(flags(old_s), 1, 1) == 1 ==> (vm_bitmap_lo == Bits(PendingVmNotifications(Bits(receiver_id(old_s), 15, 0), Bits(receiver_id(old_s), 31, 16)), 31, 0) && vm_bitmap_hi == Bits(PendingVmNotifications(Bits(receiver_id(old_s), 15, 0), Bits(receiver_id(old_s), 31, 16)), 63, 32)))
        && (Bits(flags(old_s), 2, 2) == 1 ==> spm_bitmap == PendingSpmFrameworkNotifications(Bits(receiver_id(old_s), 15, 0), Bits(receiver_id(old_s), 31, 16)))
        && (Bits(flags(old_s), 3, 3) == 1 ==> hyp_bitmap == PendingHypervisorFrameworkNotifications(Bits(receiver_id(old_s), 15, 0), Bits(receiver_id(old_s), 31, 16)))
    ))
    && (ResultEqual(result, FFA_SUCCESS) ==> new_s == old_s)
}