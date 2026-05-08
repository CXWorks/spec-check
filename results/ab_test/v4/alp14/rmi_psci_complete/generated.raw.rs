```rust
pub open spec fn RMI_PSCI_COMPLETE_spec(
    old_s: S,
    new_s: S,
    calling_rec_ptr: Address,
    target_rec_ptr: Address,
    status: u64,
    result: u64
) -> bool {
    let calling_rec = RecAt(old_s, calling_rec_ptr);
    let target_rec = RecAt(old_s, target_rec_ptr);
    
    (
        // alias failure
        (calling_rec_ptr == target_rec_ptr ==> result == RMI_ERROR_INPUT as u64) &&
        // calling_align failure
        (!AddrIsGranuleAligned(calling_rec_ptr) ==> result == RMI_ERROR_INPUT as u64) &&
        // calling_bound failure
        (!PaIsDelegable(calling_rec_ptr) ==> result == RMI_ERROR_INPUT as u64) &&
        // calling_state failure
        (GranuleAt(old_s, calling_rec_ptr).state != REC ==> result == RMI_ERROR_INPUT as u64) &&
        // target_align failure
        (!AddrIsGranuleAligned(target_rec_ptr) ==> result == RMI_ERROR_INPUT as u64) &&
        // target_bound failure
        (!PaIsDelegable(target_rec_ptr) ==> result == RMI_ERROR_INPUT as u64) &&
        // target_state failure
        (GranuleAt(old_s, target_rec_ptr).state != REC ==> result == RMI_ERROR_INPUT as u64) &&
        // pending failure
        (calling_rec.pending != REC_PENDING_PSCI ==> result == RMI_ERROR_INPUT as u64) &&
        // owner failure
        (target_rec.owner != calling_rec.owner ==> result == RMI_ERROR_INPUT as u64) &&
        // target failure
        (target_rec.mpidr != calling_rec.gprs[1] ==> result == RMI_ERROR_INPUT as u64) &&
        // status failure
        (!PsciReturnCodePermitted(calling_rec, target_rec, status) ==> result == RMI_ERROR_INPUT as u64) &&
        // success conditions
        (result == RMI_SUCCESS as u64 ==> (
            RecAt(new_s, calling_rec_ptr).pending == REC_PENDING_NONE &&
            (
                (status == PSCI_SUCCESS as u64 && calling_rec.gprs[0] == FID_PSCI_CPU_ON && target_rec.flags.runnable == RUNNABLE) ==>
                (RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(old_s, PSCI_ALREADY_ON as u64))
            ) &&
            (
                (status == PSCI_SUCCESS as u64 && calling_rec.gprs[0] == FID_PSCI_CPU_ON && target_rec.flags.runnable != RUNNABLE) ==>
                (
                    RecAt(new_s, target_rec_ptr).gprs[0] == calling_rec.gprs[3] &&
                    RecAt(new_s, target_rec_ptr).gprs[1] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[2] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[3] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[4] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[5] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[6] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[7] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[8] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[9] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[10] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[11] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[12] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[13] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[14] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[15] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[16] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[17] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[18] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[19] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[20] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[21] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[22] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[23] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[24] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[25] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[26] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[27] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[28] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[29] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[30] == 0 &&
                    RecAt(new_s, target_rec_ptr).gprs[31] == 0 &&
                    RecAt(new_s, target_rec_ptr).pc == calling_rec.gprs[2] &&
                    RecAt(new_s, target_rec_ptr).flags.runnable == RUNNABLE &&
                    RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(old_s, PSCI_SUCCESS as u64)
                )
            ) &&
            (
                (status == PSCI_SUCCESS as u64 && calling_rec.gprs[0] == FID_PSCI_AFFINITY_INFO && target_rec.flags.runnable == RUNNABLE) ==>
                (RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(old_s, PSCI_SUCCESS as u64))
            ) &&
            (
                (status == PSCI_SUCCESS as u64 && calling_rec.gprs[0] == FID_PSCI_AFFINITY_INFO && target_rec.flags.runnable != RUNNABLE) ==>
                (RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(old_s, PSCI_OFF as u64))
            ) &&
            (
                (status != PSCI_SUCCESS as u64) ==>
                (RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(old_s, status))
            ) &&
            RecAt(new_s, calling_rec_ptr).gprs[1] == 0 &&
            RecAt(new_s, calling_rec_ptr).gprs[2] == 0 &&
            RecAt(new_s, calling_rec_ptr).gp