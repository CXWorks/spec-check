pub open spec fn rmi_psci_complete_spec(result: Result<(), RmiStatusCode>, calling_rec: Address, target_rec: Address, status: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (calling_rec == target_rec ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, calling_rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, calling_rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, calling_rec).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, target_rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, target_rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, target_rec).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Rec(old_s, calling_rec).psci_pending != PSCI_REQUEST_PENDING ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Rec(old_s, target_rec).owner != Rec(old_s, calling_rec).owner ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Rec(old_s, target_rec).mpidr != Rec(old_s, calling_rec).gprs[1] ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PsciReturnCodePermitted(old_s, Rec(old_s, calling_rec), Rec(old_s, target_rec), status) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((calling_rec != target_rec
            && AddrIsGranuleAligned(old_s, calling_rec)
            && PaIsDelegable(old_s, calling_rec)
            && Granule(old_s, calling_rec).state == REC
            && AddrIsGranuleAligned(old_s, target_rec)
            && PaIsDelegable(old_s, target_rec)
            && Granule(old_s, target_rec).state == REC
            && Rec(old_s, calling_rec).psci_pending == PSCI_REQUEST_PENDING
            && Rec(old_s, target_rec).owner == Rec(old_s, calling_rec).owner
            && Rec(old_s, target_rec).mpidr == Rec(old_s, calling_rec).gprs[1]
            && PsciReturnCodePermitted(old_s, Rec(old_s, calling_rec), Rec(old_s, target_rec), status))
        ==> (result.is_Ok()
            && Rec(new_s, calling_rec).psci_pending == NO_PSCI_REQUEST_PENDING
            && ((status == PSCI_SUCCESS
                    && Rec(old_s, calling_rec).gprs[0] == FID_PSCI_CPU_ON
                    && Rec(old_s, target_rec).flags.runnable == RUNNABLE)
                ==> Rec(new_s, calling_rec).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_ALREADY_ON))
            && ((status == PSCI_SUCCESS
                    && Rec(old_s, calling_rec).gprs[0] == FID_PSCI_CPU_ON
                    && Rec(old_s, target_rec).flags.runnable != RUNNABLE)
                ==> (Rec(new_s, target_rec).gprs[0] == Rec(old_s, calling_rec).gprs[3]
                    && (forall|i: int| 1 <= i <= 31 ==> Rec(new_s, target_rec).gprs[i] == 0)
                    && Rec(new_s, target_rec).pc == Rec(old_s, calling_rec).gprs[2]
                    && Rec(new_s, target_rec).flags.runnable == RUNNABLE
                    && Rec(new_s, calling_rec).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_SUCCESS)))
            && ((status == PSCI_SUCCESS
                    && Rec(old_s, calling_rec).gprs[0] == FID_PSCI_AFFINITY_INFO
                    && Rec(old_s, target_rec).flags.runnable == RUNNABLE)
                ==> Rec(new_s, calling_rec).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_SUCCESS))
            && ((status == PSCI_SUCCESS
                    && Rec(old_s, calling_rec).gprs[0] == FID_PSCI_AFFINITY_INFO
                    && Rec(old_s, target_rec).flags.runnable != RUNNABLE)
                ==> Rec(new_s, calling_rec).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_OFF))
            && (status != PSCI_SUCCESS
                ==> Rec(new_s, calling_rec).gprs[0] == PsciReturnCodeEncode(new_s, status))
            && Rec(new_s, calling_rec).gprs[1] == 0
            && Rec(new_s, calling_rec).gprs[2] == 0
            && Rec(new_s, calling_rec).gprs[3] == 0))
}