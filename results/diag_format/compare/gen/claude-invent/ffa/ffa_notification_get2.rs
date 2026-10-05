pub open spec fn ffa_notification_get2_spec(
    result: FfaReturnCode,
    receiver_id: UInt32,
    flags: UInt64,
    sp_mask: Seq<UInt64>,
    vm_mask: Seq<UInt64>,
    spmc_mask: UInt64,
    hyp_mask: UInt64,
    sp_bitmap: Seq<UInt64>,
    vm_bitmap: Seq<UInt64>,
    spmc_bitmap: UInt64,
    hyp_bitmap: UInt64,
    old_s: S,
    new_s: S,
) -> bool {
    let vcpu_id = (receiver_id >> 16u32) & 0xFFFFu32;
    let endpoint_id = receiver_id & 0xFFFFu32;
    let flags_bad = ((flags >> 4u64) != 0u64)
        || (IsNonSecurePhysicalInstance(old_s)
            && (((flags & 0x2u64) != 0u64) || ((flags & 0x8u64) != 0u64)));
    let exceeds_supported = NotificationMaskExceedsSupported(old_s, sp_mask)
        || NotificationMaskExceedsSupported(old_s, vm_mask);
    let empty_bitmap = IsEmptyNotificationBitmapSpecified(old_s, endpoint_id, vcpu_id, flags);
    let invalid = !IsValidPartitionId(old_s, endpoint_id) || flags_bad || exceeds_supported || empty_bitmap;
    let implemented = IsFfaNotificationGet2Implemented(old_s);
    let allowed = IsCallerAllowedFfaNotificationGet2(old_s);
    (!implemented ==> result == NOT_SUPPORTED)
    && ((implemented && !allowed) ==> result == DENIED)
    && ((implemented && allowed && invalid) ==> result == INVALID_PARAMETERS)
    && ((implemented && allowed && !invalid && sp_mask.len() == 6 && vm_mask.len() == 6) ==> (
        result == FFA_SUCCESS64
        && ((flags & 0x1u64) != 0u64 ==> (
            sp_bitmap.len() == 6
            && (forall|i: int| 0 <= i < 6 ==> (
                sp_bitmap[i] == (SpNotificationBitmap(old_s, endpoint_id, vcpu_id, i) & !sp_mask[i])
                && SpNotificationBitmap(new_s, endpoint_id, vcpu_id, i)
                    == (SpNotificationBitmap(old_s, endpoint_id, vcpu_id, i) & sp_mask[i])))))
        && ((flags & 0x1u64) == 0u64 ==> (
            forall|i: int| 0 <= i < 6 ==>
                SpNotificationBitmap(new_s, endpoint_id, vcpu_id, i)
                    == SpNotificationBitmap(old_s, endpoint_id, vcpu_id, i)))
        && ((flags & 0x2u64) != 0u64 ==> (
            vm_bitmap.len() == 6
            && (forall|i: int| 0 <= i < 6 ==> (
                vm_bitmap[i] == (VmNotificationBitmap(old_s, endpoint_id, vcpu_id, i) & !vm_mask[i])
                && VmNotificationBitmap(new_s, endpoint_id, vcpu_id, i)
                    == (VmNotificationBitmap(old_s, endpoint_id, vcpu_id, i) & vm_mask[i])))))
        && ((flags & 0x2u64) == 0u64 ==> (
            forall|i: int| 0 <= i < 6 ==>
                VmNotificationBitmap(new_s, endpoint_id, vcpu_id, i)
                    == VmNotificationBitmap(old_s, endpoint_id, vcpu_id, i)))
        && ((flags & 0x4u64) != 0u64 ==> (
            spmc_bitmap == (SpmFrameworkNotificationBitmap(old_s, endpoint_id, vcpu_id) & !spmc_mask)
            && SpmFrameworkNotificationBitmap(new_s, endpoint_id, vcpu_id)
                == (SpmFrameworkNotificationBitmap(old_s, endpoint_id, vcpu_id) & spmc_mask)))
        && ((flags & 0x4u64) == 0u64 ==>
            SpmFrameworkNotificationBitmap(new_s, endpoint_id, vcpu_id)
                == SpmFrameworkNotificationBitmap(old_s, endpoint_id, vcpu_id))
        && ((flags & 0x8u64) != 0u64 ==> (
            hyp_bitmap == (HypervisorFrameworkNotificationBitmap(old_s, endpoint_id, vcpu_id) & !hyp_mask)
            && HypervisorFrameworkNotificationBitmap(new_s, endpoint_id, vcpu_id)
                == (HypervisorFrameworkNotificationBitmap(old_s, endpoint_id, vcpu_id) & hyp_mask)))
        && ((flags & 0x8u64) == 0u64 ==>
            HypervisorFrameworkNotificationBitmap(new_s, endpoint_id, vcpu_id)
                == HypervisorFrameworkNotificationBitmap(old_s, endpoint_id, vcpu_id))
    ))
}
