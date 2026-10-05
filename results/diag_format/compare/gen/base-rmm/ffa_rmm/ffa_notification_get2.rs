pub open spec fn ffa_notification_get2_spec(result: UInt32, error_code: Int32, sp_bitmap: Array<UInt64, 6>, vm_bitmap: Array<UInt64, 6>, spmc_bitmap: UInt64, hyp_bitmap: UInt64, old_s: S, new_s: S) -> bool {
    let fid: UInt32 = 0xC4000097;
    let flags: UInt64 = old_s.cmd_input_flags;
    let receiver_endpoint_id: UInt16 = old_s.cmd_input_receiver_endpoint_id;
    let receiver_vcpu_id: UInt16 = old_s.cmd_input_receiver_vcpu_id;
    let sp_bitmask: Array<UInt64, 6> = old_s.cmd_input_sp_bitmask;
    let vm_bitmask: Array<UInt64, 6> = old_s.cmd_input_vm_bitmask;
    let spmc_fw_bitmask: UInt64 = old_s.cmd_input_spmc_fw_bitmask;
    let hyp_fw_bitmask: UInt64 = old_s.cmd_input_hyp_fw_bitmask;
    let instance: UInt32 = old_s.cmd_input_instance;

    let flags_rsvd: bool = (flags & 0xFFFF_FFFF_FFFF_F000) != 0;
    let flags_ns_phys: bool = (instance == NON_SECURE_PHYSICAL) && ((flags & 0x0000_0000_0000_0002) != 0 || (flags & 0x0000_0000_0000_0008) != 0);
    let flags_0: bool = (flags & 0x0000_0000_0000_0001) != 0;
    let flags_1: bool = (flags & 0x0000_0000_0000_0004) != 0;
    let flags_2: bool = (flags & 0x0000_0000_0000_0010) != 0;
    let flags_3: bool = (flags & 0x0000_0000_0000_0020) != 0;
    let flags_empty: bool = (!flags_0 && !flags_1 && !flags_2 && !flags_3);

    let sp_exceeds: bool = ExceedsSupportedNotifications(sp_bitmask);
    let vm_exceeds: bool = ExceedsSupportedNotifications(vm_bitmask);
    let notif_range: bool = sp_exceeds || vm_exceeds;

    let is_recognized: bool = IsRecognizedPartitionId(receiver_endpoint_id);
    let is_implemented: bool = IsImplementedAtInstance(FFA_NOTIFICATION_GET2, instance);
    let is_allowed: bool = IsCallerAllowedToInvoke(FFA_NOTIFICATION_GET2);

    let result_is_ok: bool = ResultEqual(result, FFA_SUCCESS64);
    let result_is_err: bool = ResultEqual(result, FFA_ERROR);

    let sp_bitmap_valid: bool = flags_0 ==> (forall i in 0..6 { sp_bitmask[i] == 0 || (forall j in 0..64 { if (j < 64) { let bit_idx: UInt64 = (j % 64) as UInt64; let word_idx: UInt64 = (j / 64) as UInt64; if (word_idx < 6) { sp_bitmap[word_idx][bit_idx as UInt64] == IsPending(receiver_endpoint_id, receiver_vcpu_id, SP_BITMAP, j) } else { true } } }) });
    let vm_bitmap_valid: bool = flags_1 ==> (forall i in 0..6 { vm_bitmask[i] == 0 || (forall j in 0..64 { if (j < 64) { let bit_idx: UInt64 = (j % 64) as UInt64; let word_idx: UInt64 = (j / 64) as UInt64; if (word_idx < 6) { vm_bitmap[word_idx][bit_idx as UInt64] == IsPending(receiver_endpoint_id, receiver_vcpu_id, VM_BITMAP, j) } else { true } } }) });
    let spmc_bitmap_valid: bool = flags_2 ==> (forall j in 0..64 { if (j < 64) { let bit_idx: UInt64 = (j % 64) as UInt64; spmc_bitmap[bit_idx as UInt64] == IsPending(receiver_endpoint_id, receiver_vcpu_id, SPMC_FW_BITMAP, j) } else { true } });
    let hyp_bitmap_valid: bool = flags_3 ==> (forall j in 0..64 { if (j < 64) { let bit_idx: UInt64 = (j % 64) as UInt64; hyp_bitmap[bit_idx as UInt64] == IsPending(receiver_endpoint_id, receiver_vcpu_id, HYP_FW_BITMAP, j) } else { true } });

    let sp_masked_unchanged: bool = (forall i in 0..6 { if (sp_bitmask[i] != 0) { (forall j in 0..64 { if (j < 64) { let bit_idx: UInt64 = (j % 64) as UInt64; let word_idx: UInt64 = (j / 64) as UInt64; if (word_idx < 6) { let bit_val: bool = (sp_bitmap[word_idx][bit_idx as UInt64] != 0); NotificationState(receiver_endpoint_id, receiver_vcpu_id, SP_BITMAP, j) == old(NotificationState(receiver_endpoint_id, receiver_vcpu_id, SP_BITMAP, j)) } else { true } } }) });
    let vm_masked_unchanged: bool = (forall i in 0..6 { if (vm_bitmask[i] != 0) { (forall j in 0..64 { if (j < 64) { let bit_idx: UInt64 = (j % 64) as UInt64; let word_idx: UInt64 = (j / 64) as UInt64; if (word_idx < 6) { let bit_val: bool = (vm_bitmap[word_idx][bit_idx as UInt64] != 0); NotificationState(receiver_endpoint_id, receiver_vcpu_id, VM_BITMAP, j) == old(NotificationState(receiver_endpoint_id, receiver_vcpu_id, VM_BITMAP, j)) } else { true } } }) });
    let spmc_masked_unchanged: bool = (forall j in 0..64 { if ((spmc_fw_bitmask & (1 << j)) != 0) { NotificationState(receiver_endpoint_id, receiver_vcpu_id, SPMC_FW_BITMAP, j) == old(NotificationState(receiver_endpoint_id, receiver_vcpu_id, SPMC_FW_BITMAP, j)) });
    let hyp_masked_unchanged: bool = (forall j in 0..64 { if ((hyp_fw_bitmask & (1 << j)) != 0) { NotificationState(receiver_endpoint_id, receiver_vcpu_id, HYP_FW_BITMAP, j) == old(NotificationState(receiver_endpoint_id, receiver_vcpu_id, HYP_FW_BITMAP, j)) });

    let masked_unchanged: bool = sp_masked_unchanged && vm_masked_unchanged && spmc_masked_unchanged && hyp_masked_unchanged;

    (!is_implemented ==> ResultEqual(result, NOT_SUPPORTED))
    && (!is_allowed ==> ResultEqual(result, DENIED))
    && (!is_recognized ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags_rsvd ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags_ns_phys ==> ResultEqual(result, INVALID_PARAMETERS))
    && (notif_range ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags_empty ==> ResultEqual(result, INVALID_PARAMETERS))
    && (result_is_ok ==> (result == FFA_SUCCESS64 && error_code == 0 && sp_bitmap == 0 && vm_bitmap == 0 && spmc_bitmap == 0 && hyp_bitmap == 0))
    && (result_is_ok && flags_0 ==> sp_bitmap_valid)
    && (result_is_ok && flags_1 ==> vm_bitmap_valid)
    && (result_is_ok && flags_2 ==> spmc_bitmap_valid)
    && (result_is_ok && flags_3 ==> hyp_bitmap_valid)
    && (result_is_ok ==> masked_unchanged)
}