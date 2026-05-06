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
    let fail_alias = calling_rec_ptr == target_rec_ptr ==> ResultEqual(result, RMI_ERROR_INPUT);
    let fail_calling_align = !AddrIsGranuleAligned(calling_rec_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let fail_calling_bound = !PaIsDelegable(calling_rec_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let fail_calling_state = GranuleAt(s, calling_rec_ptr).state != REC ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let fail_target_align = !AddrIsGranuleAligned(target_rec_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let fail_target_bound = !PaIsDelegable(target_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let fail_target_state = GranuleAt(s, target_rec_ptr).state != REC ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let fail_pending = calling_rec.pending != REC_PENDING_PSCI ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let fail_owner = target_rec.owner != calling_rec.owner ==> ResultEqual(result, RMI_ERROR_INPUT);
    let fail_target = target_rec.mpidr != calling_rec.gprs[1] ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let fail_status = !PsciReturnCodePermitted(s, calling_rec, target_rec, status) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    // Success conditions
    let succ_pending = !result.is_Err() ==> calling_rec.pending == REC_PENDING_NONE;
    let succ_on_already = (status == PSCI_SUCCESS && calling_rec.gprs[0] == FID_PSCI_CPU_ON
        && target_rec.flags.runnable == RUNNABLE) ==> (calling_rec.gprs[0] == PsciReturnCodeEncode(
        s,
        PSCI_ALREADY_ON,
    ));
    let succ_on_success = (status == PSCI_SUCCESS && calling_rec.gprs[0] == FID_PSCI_CPU_ON
        && target_rec.flags.runnable != RUNNABLE) ==> (target_rec.gprs[0] == calling_rec.gprs[3]
        && target_rec.gprs[1] == Zeros(64) && target_rec.gprs[2] == Zeros(64) && target_rec.gprs[3]
        == Zeros(64) && target_rec.gprs[4] == Zeros(64) && target_rec.gprs[5] == Zeros(64)
        && target_rec.gprs[6] == Zeros(64) && target_rec.gprs[7] == Zeros(64) && target_rec.gprs[8]
        == Zeros(64) && target_rec.gprs[9] == Zeros(64) && target_rec.gprs[10] == Zeros(64)
        && target_rec.gprs[11] == Zeros(64) && target_rec.gprs[12] == Zeros(64)
        && target_rec.gprs[13] == Zeros(64) && target_rec.gprs[14] == Zeros(64)
        && target_rec.gprs[15] == Zeros(64) && target_rec.gprs[16] == Zeros(64)
        && target_rec.gprs[17] == Zeros(64) && target_rec.gprs[18] == Zeros(64)
        && target_rec.gprs[19] == Zeros(64) && target_rec.gprs[20] == Zeros(64)
        && target_rec.gprs[21] == Zeros(64) && target_rec.gprs[22] == Zeros(64)
        && target_rec.gprs[23] == Zeros(64) && target_rec.gprs[24] == Zeros(64)
        && target_rec.gprs[25] == Zeros(64) && target_rec.gprs[26] == Zeros(64)
        && target_rec.gprs[27] == Zeros(64) && target_rec.gprs[28] == Zeros(64)
        && target_rec.gprs[29] == Zeros(64) && target_rec.gprs[30] == Zeros(64)
        && target_rec.gprs[31] == Zeros(64) && target_rec.pc == calling_rec.gprs[2]
        && target_rec.flags.runnable == RUNNABLE && calling_rec.gprs[0] == PsciReturnCodeEncode(
        s,
        PSCI_SUCCESS,
    ));
    let succ_affinity_on = (status == PSCI_SUCCESS && calling_rec.gprs[0] == FID_PSCI_AFFINITY_INFO
        && target_rec.flags.runnable == RUNNABLE) ==> (calling_rec.gprs[0] == PsciReturnCodeEncode(
        s,
        PSCI_SUCCESS,
    ));
    let succ_affinity_off = (status == PSCI_SUCCESS && calling_rec.gprs[0] == FID_PSCI_AFFINITY_INFO
        && target_rec.flags.runnable != RUNNABLE) ==> (calling_rec.gprs[0] == PsciReturnCodeEncode(
        s,
        PSCI_OFF,
    ));
    let succ_status = status != PSCI_SUCCESS ==> (calling_rec.gprs[0] == PsciReturnCodeEncode(
        s,
        status,
    ));
    let succ_args = calling_rec.gprs[1] == Zeros(64) && calling_rec.gprs[2] == Zeros(64)
        && calling_rec.gprs[3] == Zeros(64);

    fail_alias && fail_calling_align && fail_calling_bound && fail_calling_state
        && fail_target_align && fail_target_bound && fail_target_state && fail_pending && fail_owner
        && fail_target && fail_status && succ_pending && succ_on_already && succ_on_success
        && succ_affinity_on && succ_affinity_off && succ_status && succ_args
}