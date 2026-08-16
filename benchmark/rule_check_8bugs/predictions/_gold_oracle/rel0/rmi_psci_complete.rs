pub open spec fn rmi_psci_complete_spec(calling_rec: Address, target_rec: Address, status: PsciReturnCode, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
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
  && (result.is_Ok() ==> Rec(new_s, calling_rec).psci_pending == NO_PSCI_REQUEST_PENDING)
  && (result.is_Ok() && (status == PSCI_SUCCESS && Rec(old_s, calling_rec).gprs[0] == FID_PSCI_CPU_ON && Rec(old_s, target_rec).flags.runnable == RUNNABLE) ==> (Rec(new_s, calling_rec).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_ALREADY_ON)))
  && (result.is_Ok() && (status == PSCI_SUCCESS && Rec(old_s, calling_rec).gprs[0] == FID_PSCI_CPU_ON && Rec(old_s, target_rec).flags.runnable != RUNNABLE) ==> (Rec(new_s, target_rec).gprs[0] == Rec(new_s, calling_rec).gprs[3] && Rec(new_s, target_rec).gprs[1] == 0 && Rec(new_s, target_rec).gprs[2] == 0 && Rec(new_s, target_rec).gprs[3] == 0 && Rec(new_s, target_rec).gprs[4] == 0 && Rec(new_s, target_rec).gprs[5] == 0 && Rec(new_s, target_rec).gprs[6] == 0 && Rec(new_s, target_rec).gprs[7] == 0 && Rec(new_s, target_rec).gprs[8] == 0 && Rec(new_s, target_rec).gprs[9] == 0 && Rec(new_s, target_rec).gprs[10] == 0 && Rec(new_s, target_rec).gprs[11] == 0 && Rec(new_s, target_rec).gprs[12] == 0 && Rec(new_s, target_rec).gprs[13] == 0 && Rec(new_s, target_rec).gprs[14] == 0 && Rec(new_s, target_rec).gprs[15] == 0 && Rec(new_s, target_rec).gprs[16] == 0 && Rec(new_s, target_rec).gprs[17] == 0 && Rec(new_s, target_rec).gprs[18] == 0 && Rec(new_s, target_rec).gprs[19] == 0 && Rec(new_s, target_rec).gprs[20] == 0 && Rec(new_s, target_rec).gprs[21] == 0 && Rec(new_s, target_rec).gprs[22] == 0 && Rec(new_s, target_rec).gprs[23] == 0 && Rec(new_s, target_rec).gprs[24] == 0 && Rec(new_s, target_rec).gprs[25] == 0 && Rec(new_s, target_rec).gprs[26] == 0 && Rec(new_s, target_rec).gprs[27] == 0 && Rec(new_s, target_rec).gprs[28] == 0 && Rec(new_s, target_rec).gprs[29] == 0 && Rec(new_s, target_rec).gprs[30] == 0 && Rec(new_s, target_rec).gprs[31] == 0 && Rec(new_s, target_rec).pc == Rec(new_s, calling_rec).gprs[2] && Rec(new_s, target_rec).flags.runnable == RUNNABLE && Rec(new_s, calling_rec).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_SUCCESS)))
  && (result.is_Ok() && (status == PSCI_SUCCESS && Rec(old_s, calling_rec).gprs[0] == FID_PSCI_AFFINITY_INFO && Rec(old_s, target_rec).flags.runnable == RUNNABLE) ==> (Rec(new_s, calling_rec).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_SUCCESS)))
  && (result.is_Ok() && (status == PSCI_SUCCESS && Rec(old_s, calling_rec).gprs[0] == FID_PSCI_AFFINITY_INFO && Rec(old_s, target_rec).flags.runnable != RUNNABLE) ==> (Rec(new_s, calling_rec).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_OFF)))
  && (result.is_Ok() && status != PSCI_SUCCESS ==> (Rec(new_s, calling_rec).gprs[0] == PsciReturnCodeEncode(new_s, status)))
  && (result.is_Ok() ==> (Rec(new_s, calling_rec).gprs[1] == 0 && Rec(new_s, calling_rec).gprs[2] == 0 && Rec(new_s, calling_rec).gprs[3] == 0))
  && ((!(calling_rec == target_rec) &&
       AddrIsGranuleAligned(old_s, calling_rec) &&
       PaIsDelegable(old_s, calling_rec) &&
       !(Granule(old_s, calling_rec).state != REC) &&
       AddrIsGranuleAligned(old_s, target_rec) &&
       PaIsDelegable(old_s, target_rec) &&
       !(Granule(old_s, target_rec).state != REC) &&
       !(Rec(old_s, calling_rec).psci_pending != PSCI_REQUEST_PENDING) &&
       !(Rec(old_s, target_rec).owner != Rec(old_s, calling_rec).owner) &&
       !(Rec(old_s, target_rec).mpidr != Rec(old_s, calling_rec).gprs[1]) &&
       PsciReturnCodePermitted(old_s, Rec(old_s, calling_rec), Rec(old_s, target_rec), status))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Rec(new_s, calling_rec).psci_pending == Rec(old_s, calling_rec).psci_pending)
  && (result.is_Err()
    ==> Rec(new_s, calling_rec).gprs[0] == Rec(old_s, calling_rec).gprs[0])
  && (result.is_Err()
    ==> Rec(new_s, calling_rec).gprs[0] == Rec(old_s, calling_rec).gprs[0])
  && (result.is_Err()
    ==> Rec(new_s, calling_rec).gprs[0] == Rec(old_s, calling_rec).gprs[0])
  && (result.is_Err()
    ==> Rec(new_s, calling_rec).gprs[0] == Rec(old_s, calling_rec).gprs[0])
}
