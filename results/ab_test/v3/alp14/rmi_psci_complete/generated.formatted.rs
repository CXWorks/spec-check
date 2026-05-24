```verus
pub open spec fn rmi_psci_complete_spec(
    result: RmiCommandReturnCode,
    calling_rec_ptr: Address,
    target_rec_ptr: Address,
    status: PsciReturnCode,
    old_s: S,
    new_s: S
) -> bool {
    let calling_rec = RecAt(old_s, calling_rec_ptr);
    let target_rec = RecAt(old_s, target_rec_ptr);
    
    // Failure: alias
    (calling_rec_ptr == target_rec_ptr ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(calling_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(calling_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, calling_rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(target_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(target_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, target_rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (calling_rec.pending != REC_PENDING_PSCI ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (target_rec.owner != calling_rec.owner ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (target_rec.mpidr != calling_rec.gprs[1] ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PsciReturnCodePermitted(old_s, calling_rec, target_rec, status) ==> ResultEqual(result, RMI_ERROR_INPUT))
    
    // Success conditions
    && (
        (calling_rec_ptr != target_rec_ptr
         && AddrIsGranuleAligned(calling_rec_ptr)
         && PaIsDelegable(calling_rec_ptr)
         && GranuleAt(old_s, calling_rec_ptr).state == REC
         && AddrIsGranuleAligned(target_rec_ptr)
         && PaIsDelegable(target_rec_ptr)
         && GranuleAt(old_s, target_rec_ptr).state == REC
         && calling_rec.pending == REC_PENDING_PSCI
         && target_rec.owner == calling_rec.owner
         && target_rec.mpidr == calling_rec.gprs[1]
         && PsciReturnCodePermitted(old_s, calling_rec, target_rec, status))
        ==> (result.is_Ok()
             && RecAt(new_s, calling_rec_ptr).pending == REC_PENDING_NONE
             && (
                 (status == PSCI_SUCCESS
                  && calling_rec.gprs[0] == FID_PSCI_CPU_ON
                  && target_rec.flags.runnable == RUNNABLE)
                 ==> RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(old_s, PSCI_ALREADY_ON)
             )
             && (
                 (status == PSCI_SUCCESS
                  && calling_rec.gprs[0] == FID_PSCI_CPU_ON
                  && target_rec.flags.runnable != RUNNABLE)
                 ==> (RecAt(new_s, target_rec_ptr).gprs[0] == calling_rec.gprs[3]
                      && RecAt(new_s, target_rec_ptr).gprs[1] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[2] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[3] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[4] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[5] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[6] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[7] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[8] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[9] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[10] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[11] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[12] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[13] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[14] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[15] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[16] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[17] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[18] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[19] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[20] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[21] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[22] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[23] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[24] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[25] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[26] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[27] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[28] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[29] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[30] == 0
                      && RecAt(new_s, target_rec_ptr).gprs[31] == 0
                      && RecAt(new_s, target_rec_ptr).pc == calling_rec.gprs[2]
                      && RecAt(new_s, target_rec_ptr).flags.runnable == RUNNABLE
                      && RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(old_s, PSCI_SUCCESS))
             )
             && (
                 (status == PSCI_SUCCESS
                  && calling_rec.gprs[0] == FID_PSCI_AFFINITY_INFO
                  && target_rec.flags.runnable == RUNNABLE)
                 ==> RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(old_s, PSCI_SUCCESS)
             )
             && (
                 (status == PSCI_SUCCESS
                  && calling_rec.gprs[0] == FID_PSCI_AFFINITY_INFO
                  && target_rec.flags.runnable != RUNNABLE)
                 ==> RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(old_s, PSCI_OFF)
             )
             && (status != PSCI_SUCCESS