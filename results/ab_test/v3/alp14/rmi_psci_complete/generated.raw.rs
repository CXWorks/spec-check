pub open spec fn rmi_psci_complete_spec(result: RmiCommandReturnCode, calling_rec_ptr: Address, target_rec_ptr: Address, status: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (calling_rec_ptr == target_rec_ptr ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(calling_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(calling_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, calling_rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(target_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(target_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, target_rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RecAt(old_s, calling_rec_ptr).pending != REC_PENDING_PSCI ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RecAt(old_s, target_rec_ptr).owner != RecAt(old_s, calling_rec_ptr).owner ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RecAt(old_s, target_rec_ptr).mpidr != RecAt(old_s, calling_rec_ptr).gprs[1] ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PsciReturnCodePermitted(old_s, RecAt(old_s, calling_rec_ptr), RecAt(old_s, target_rec_ptr), status) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((GranuleAt(old_s, calling_rec_ptr).state == REC
        && AddrIsGranuleAligned(calling_rec_ptr)
        && PaIsDelegable(calling_rec_ptr)
        && GranuleAt(old_s, target_rec_ptr).state == REC
        && AddrIsGranuleAligned(target_rec_ptr)
        && PaIsDelegable(target_rec_ptr)
        && RecAt(old_s, calling_rec_ptr).pending == REC_PENDING_PSCI
        && RecAt(old_s, target_rec_ptr).owner == RecAt(old_s, calling_rec_ptr).owner
        && RecAt(old_s, target_rec_ptr).mpidr == RecAt(old_s, calling_rec_ptr).gprs[1]
        && PsciReturnCodePermitted(old_s, RecAt(old_s, calling_rec_ptr), RecAt(old_s, target_rec_ptr), status))
      ==> (RecAt(new_s, calling_rec_ptr).pending == REC_PENDING_NONE
        && ((status == PSCI_SUCCESS
            && RecAt(old_s, calling_rec_ptr).gprs[0] == FID_PSCI_CPU_ON
            && RecAt(old_s, target_rec_ptr).flags.runnable == RUNNABLE)
          ==> RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(old_s, PSCI_ALREADY_ON))
        && ((status == PSCI_SUCCESS
            && RecAt(old_s, calling_rec_ptr).gprs[0] == FID_PSCI_CPU_ON
            && RecAt(old_s, target_rec_ptr).flags.runnable != RUNNABLE)
          ==> (RecAt(new_s, target_rec_ptr).gprs[0] == RecAt(old_s, calling_rec_ptr).gprs[3]
            && RecAt(new_s, target_rec_ptr).gprs[1] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[2] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[3] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[4] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[5] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[6] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[7] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[8] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[9] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[10] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[11] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[12] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[13] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[14] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[15] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[16] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[17] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[18] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[19] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[20] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[21] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[22] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[23] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[24] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[25] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[26] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[27] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[28] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[29] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[30] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).gprs[31] == Zeros(64)
            && RecAt(new_s, target_rec_ptr).pc == RecAt(old_s, calling_rec_ptr).gprs[2]
            && RecAt(new_s, target_rec_ptr).flags.runnable == RUNNABLE
            && RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(old_s, PSCI_SUCCESS)))
        && ((status == PSCI_SUCCESS
            && RecAt(old_s, calling_rec_ptr).gprs[0] == FID_PSCI_AFFINITY_INFO
            && RecAt(old_s, target_rec_ptr).flags.runnable == RUNNABLE)
          ==> RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEn