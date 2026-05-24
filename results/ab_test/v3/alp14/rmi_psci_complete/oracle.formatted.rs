pub open spec fn rmi_psci_complete_spec(calling_rec_ptr: Address, target_rec_ptr: Address, status: PsciReturnCode, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (calling_rec_ptr == target_rec_ptr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, calling_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, calling_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, calling_rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, target_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, target_rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, target_rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, calling_rec_ptr).pending != REC_PENDING_PSCI ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, target_rec_ptr).owner != RecAt(old_s, calling_rec_ptr).owner ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, target_rec_ptr).mpidr != RecAt(old_s, calling_rec_ptr).gprs[1] ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PsciReturnCodePermitted(old_s, RecAt(old_s, calling_rec_ptr), RecAt(old_s, target_rec_ptr), status) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> RecAt(new_s, calling_rec_ptr).pending == REC_PENDING_NONE)
  && (result.is_Ok() && (status == PSCI_SUCCESS && RecAt(old_s, calling_rec_ptr).gprs[0] == FID_PSCI_CPU_ON && RecAt(old_s, target_rec_ptr).flags.runnable == RUNNABLE) ==> (RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_ALREADY_ON)))
  && (result.is_Ok() && (status == PSCI_SUCCESS && RecAt(old_s, calling_rec_ptr).gprs[0] == FID_PSCI_CPU_ON && RecAt(old_s, target_rec_ptr).flags.runnable != RUNNABLE) ==> (RecAt(new_s, target_rec_ptr).gprs[0] == RecAt(new_s, calling_rec_ptr).gprs[3] && RecAt(new_s, target_rec_ptr).gprs[1] == 0 && RecAt(new_s, target_rec_ptr).gprs[2] == 0 && RecAt(new_s, target_rec_ptr).gprs[3] == 0 && RecAt(new_s, target_rec_ptr).gprs[4] == 0 && RecAt(new_s, target_rec_ptr).gprs[5] == 0 && RecAt(new_s, target_rec_ptr).gprs[6] == 0 && RecAt(new_s, target_rec_ptr).gprs[7] == 0 && RecAt(new_s, target_rec_ptr).gprs[8] == 0 && RecAt(new_s, target_rec_ptr).gprs[9] == 0 && RecAt(new_s, target_rec_ptr).gprs[10] == 0 && RecAt(new_s, target_rec_ptr).gprs[11] == 0 && RecAt(new_s, target_rec_ptr).gprs[12] == 0 && RecAt(new_s, target_rec_ptr).gprs[13] == 0 && RecAt(new_s, target_rec_ptr).gprs[14] == 0 && RecAt(new_s, target_rec_ptr).gprs[15] == 0 && RecAt(new_s, target_rec_ptr).gprs[16] == 0 && RecAt(new_s, target_rec_ptr).gprs[17] == 0 && RecAt(new_s, target_rec_ptr).gprs[18] == 0 && RecAt(new_s, target_rec_ptr).gprs[19] == 0 && RecAt(new_s, target_rec_ptr).gprs[20] == 0 && RecAt(new_s, target_rec_ptr).gprs[21] == 0 && RecAt(new_s, target_rec_ptr).gprs[22] == 0 && RecAt(new_s, target_rec_ptr).gprs[23] == 0 && RecAt(new_s, target_rec_ptr).gprs[24] == 0 && RecAt(new_s, target_rec_ptr).gprs[25] == 0 && RecAt(new_s, target_rec_ptr).gprs[26] == 0 && RecAt(new_s, target_rec_ptr).gprs[27] == 0 && RecAt(new_s, target_rec_ptr).gprs[28] == 0 && RecAt(new_s, target_rec_ptr).gprs[29] == 0 && RecAt(new_s, target_rec_ptr).gprs[30] == 0 && RecAt(new_s, target_rec_ptr).gprs[31] == 0 && RecAt(new_s, target_rec_ptr).pc == RecAt(new_s, calling_rec_ptr).gprs[2] && RecAt(new_s, target_rec_ptr).flags.runnable == RUNNABLE && RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_SUCCESS)))
  && (result.is_Ok() && (status == PSCI_SUCCESS && RecAt(old_s, calling_rec_ptr).gprs[0] == FID_PSCI_AFFINITY_INFO && RecAt(old_s, target_rec_ptr).flags.runnable == RUNNABLE) ==> (RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_SUCCESS)))
  && (result.is_Ok() && (status == PSCI_SUCCESS && RecAt(old_s, calling_rec_ptr).gprs[0] == FID_PSCI_AFFINITY_INFO && RecAt(old_s, target_rec_ptr).flags.runnable != RUNNABLE) ==> (RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(new_s, PSCI_OFF)))
  && (result.is_Ok() && status != PSCI_SUCCESS ==> (RecAt(new_s, calling_rec_ptr).gprs[0] == PsciReturnCodeEncode(new_s, status)))
  && (result.is_Ok() ==> (RecAt(new_s, calling_rec_ptr).gprs[1] == 0 && RecAt(new_s, calling_rec_ptr).gprs[2] == 0 && RecAt(new_s, calling_rec_ptr).gprs[3] == 0))
  && ((!(calling_rec_ptr == target_rec_ptr) &&
       AddrIsGranuleAligned(old_s, calling_rec_ptr) &&
       PaIsDelegable(old_s, calling_rec_ptr) &&
       !(GranuleAt(old_s, calling_rec_ptr).state != REC) &&
       AddrIsGranuleAligned(old_s, target_rec_ptr) &&
       PaIsDelegable(old_s, target_rec_ptr) &&
       !(GranuleAt(old_s, target_rec_ptr).state != REC) &&
       !(RecAt(old_s, calling_rec_ptr).pending != REC_PENDING_PSCI) &&
       !(RecAt(old_s, target_rec_ptr).owner != RecAt(old_s, calling_rec_ptr).owner) &&
       !(RecAt(old_s, target_rec_ptr).mpidr != RecAt(old_s, calling_rec_ptr).gprs[1]) &&
       PsciReturnCodePermitted(old_s, RecAt(old_s, calling_rec_ptr), RecAt(old_s, target_rec_ptr), status))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, calling_rec_ptr).pending == RecAt(old_s, calling_rec_ptr).pending)
  && (result.is_Err()
    ==> RecAt(new_s, calling_rec_ptr).gprs[0] == RecAt(old_s, calling_rec_ptr).gprs[0])
  && (result.is_Err()
    ==> RecAt(new_s, calling_rec_ptr).gprs[0] == RecAt(old_s, calling_rec_ptr).gprs[0])
  && (result.is_Err()
    ==> RecAt(new_s, calling_rec_ptr).gprs[0] == RecAt(old_s, calling_rec_ptr).gprs[0])
  && (result.is_Err()
    ==> RecAt(new_s, calling_rec_ptr).gprs[0] == RecAt(old_s, calling_rec_ptr).gprs[0])
}