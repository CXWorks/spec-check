pub open spec fn RMI_PSCI_COMPLETE_spec(
    s: S,
    calling_rec_ptr: Address,
    target_rec_ptr: Address,
    status: PsciReturnCode,
    result: RmiCommandReturnCode,
) -> bool {
    let calling_rec = RecAt(s, calling_rec_ptr);
    let target_rec = RecAt(s, target_rec_ptr);

    // Failure conditions
    if calling_rec_ptr == target_rec_ptr {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !AddrIsGranuleAligned(calling_rec_ptr) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !PaIsDelegable(calling_rec_ptr) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if GranuleAt(s, calling_rec_ptr).state != REC {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !AddrIsGranuleAligned(target_rec_ptr) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !PaIsDelegable(target_rec_ptr) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if GranuleAt(s, target_rec_ptr).state != REC {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if calling_rec.pending != REC_PENDING_PSCI {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if target_rec.owner != calling_rec.owner {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if target_rec.mpidr != calling_rec.gprs[1] {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !PsciReturnCodePermitted(calling_rec, target_rec, status) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else {
        // Success conditions
        result.is_Ok() && calling_rec.pending == REC_PENDING_NONE && (
        // on_already condition
        (status == PSCI_SUCCESS && calling_rec.gprs[0] == FID_PSCI_CPU_ON
            && target_rec.flags.runnable == RUNNABLE) ==> calling_rec.gprs[0]
            == PsciReturnCodeEncode(s, PSCI_ALREADY_ON)) && (
        // on_success condition
        (status == PSCI_SUCCESS && calling_rec.gprs[0] == FID_PSCI_CPU_ON
            && target_rec.flags.runnable != RUNNABLE) ==> (target_rec.gprs[0] == calling_rec.gprs[3]
            && target_rec.gprs[1] == 0 && target_rec.gprs[2] == 0 && target_rec.gprs[3] == 0
            && target_rec.gprs[4] == 0 && target_rec.gprs[5] == 0 && target_rec.gprs[6] == 0
            && target_rec.gprs[7] == 0 && target_rec.gprs[8] == 0 && target_rec.gprs[9] == 0
            && target_rec.gprs[10] == 0 && target_rec.gprs[11] == 0 && target_rec.gprs[12] == 0
            && target_rec.gprs[13] == 0 && target_rec.gprs[14] == 0 && target_rec.gprs[15] == 0
            && target_rec.gprs[16] == 0 && target_rec.gprs[17] == 0 && target_rec.gprs[18] == 0
            && target_rec.gprs[19] == 0 && target_rec.gprs[20] == 0 && target_rec.gprs[21] == 0
            && target_rec.gprs[22] == 0 && target_rec.gprs[23] == 0 && target_rec.gprs[24] == 0
            && target_rec.gprs[25] == 0 && target_rec.gprs[26] == 0 && target_rec.gprs[27] == 0
            && target_rec.gprs[28] == 0 && target_rec.gprs[29] == 0 && target_rec.gprs[30] == 0
            && target_rec.gprs[31] == 0 && target_rec.pc == calling_rec.gprs[2]
            && target_rec.flags.runnable == RUNNABLE && calling_rec.gprs[0] == PsciReturnCodeEncode(
            s,
            PSCI_SUCCESS,
        ))) && (
        // affinity_on condition
        (status == PSCI_SUCCESS && calling_rec.gprs[0] == FID_PSCI_AFFINITY_INFO
            && target_rec.flags.runnable == RUNNABLE) ==> calling_rec.gprs[0]
            == PsciReturnCodeEncode(s, PSCI_SUCCESS)) && (
        // affinity_off condition
        (status == PSCI_SUCCESS && calling_rec.gprs[0] == FID_PSCI_AFFINITY_INFO
            && target_rec.flags.runnable != RUNNABLE) ==> calling_rec.gprs[0]
            == PsciReturnCodeEncode(s, PSCI_OFF)) && (
        // status condition
        (status != PSCI_SUCCESS) ==> calling_rec.gprs[0] == PsciReturnCodeEncode(s, status)) && (
        // args condition
        calling_rec.gprs[1] == 0 && calling_rec.gprs[2] == 0 && calling_rec.gprs[3] == 0)
    }
}