pub open spec fn sbi_remote_hfence_vvma_spec(hart_mask: unsigned long, hart_mask_base: unsigned long, start_addr: unsigned long, size: unsigned long, error: sbiret.error, old_s: S, new_s: S) -> bool {
  (!IsFunctionImplemented(old_s, RemoteHfenceVvma) ==> ResultEqual(error, SBI_ERR_NOT_SUPPORTED))
  && (!AllTargetHartsImplementHypervisorExt(old_s, hart_mask, hart_mask_base) ==> ResultEqual(error, SBI_ERR_NOT_SUPPORTED))
  && (!IsValidAddressRange(old_s, start_addr, size) ==> ResultEqual(error, SBI_ERR_INVALID_ADDRESS))
  && (!AllHartIdsValid(old_s, hart_mask, hart_mask_base) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (RequestFailedForUnspecifiedReason(old_s) ==> ResultEqual(error, SBI_ERR_FAILED))
  && (ResultEqual(error, SBI_SUCCESS) ==> ResultEqual(error, SBI_SUCCESS))
  && (ResultEqual(error, SBI_SUCCESS) ==> IpiSentToAllTargetHarts(new_s, hart_mask, hart_mask_base))
  && (ResultEqual(error, SBI_SUCCESS) ==> TargetHartsExecutedHfenceVvma(new_s, hart_mask, hart_mask_base, start_addr, start_addr + size, CurrentVmid(hgatp(new_s))))
  && ((IsFunctionImplemented(old_s, RemoteHfenceVvma) &&
       AllTargetHartsImplementHypervisorExt(old_s, hart_mask, hart_mask_base) &&
       IsValidAddressRange(old_s, start_addr, size) &&
       AllHartIdsValid(old_s, hart_mask, hart_mask_base) &&
       !RequestFailedForUnspecifiedReason(old_s))
    ==> ResultEqual(error, SBI_SUCCESS))
  && (result != SBI_SUCCESS
    ==> !IpiSentToAllTargetHarts(new_s, hart_mask, hart_mask_base))
  && (result != SBI_SUCCESS
    ==> !TargetHartsExecutedHfenceVvma(new_s, hart_mask, hart_mask_base, start_addr, start_addr + size, CurrentVmid(hgatp(new_s))))
}