pub open spec fn ffa_notification_get_spec(result: UInt32, receiver_vcpu_id: UInt16, receiver_id: UInt16, flags_sp: bool, flags_vm: bool, flags_spm: bool, flags_hyp: bool, flags_reserved: UInt32, old_s: S, new_s: S) -> bool {
    (!IsImplementedAtInstance(FFA_NOTIFICATION_GET, CurrentInstance()) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!CallerMayInvoke(FFA_NOTIFICATION_GET) ==> ResultEqual(result, DENIED))
    && (!IsRecognizedPartitionId(receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags_reserved != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (CurrentInstance() == NS_PHYSICAL && (flags_vm != 0 || flags_hyp != 0) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, FFA_SUCCESS) ==> (flags_sp == 1 ==> ((sp_bitmap_hi(new_s) as UInt64) << 32 | sp_bitmap_lo(new_s)) == PendingNotifications(receiver_id, receiver_vcpu_id, SP))
    && (flags_vm == 1 ==> (vm_bitmap_hi(new_s) as UInt64) << 32 | vm_bitmap_lo(new_s) == PendingNotifications(receiver_id, receiver_vcpu_id, VM))
    && (flags_spm == 1 ==> spm_bitmap(new_s) == PendingNotifications(receiver_id, receiver_vcpu_id, SPM_FRAMEWORK))
    && (flags_hyp == 1 ==> hyp_bitmap(new_s) == PendingNotifications(receiver_id, receiver_vcpu_id, HYP_FRAMEWORK)))
}