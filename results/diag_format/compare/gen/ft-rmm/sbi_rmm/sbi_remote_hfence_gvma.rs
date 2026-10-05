pub open spec fn sbi_remote_hfence_gvma_spec(hart_mask: unsigned long, hart_mask_base: unsigned long, start_addr: unsigned long, size: unsigned long, result: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (!IsFunctionImplemented(old_s, SBI_REMOTE_HFENCE_GVMA) || AnyTargetHartLacksHypervisorExtension(old_s, hart_mask, hart_mask_base) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
  && (!IsValidAddressRange(old_s, start_addr, size) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
  && (AnyTargetHartIdInvalid(old_s, hart_mask, hart_mask_base) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (RequestFailedForUnspecifiedReason(old_s) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> IpiSentToAllTargetHarts(new_s, hart_mask, hart_mask_base))
  && (result == SBI_SUCCESS ==> TargetHartsInstructedToExecuteHfenceGvma(new_s, hart_mask, hart_mask_base, start_addr, start_addr + size, ALL_GUESTS))
  && ((!(IsFunctionImplemented(old_s, SBI_REMOTE_HFENCE_GVMA)) &&
       IsValidAddressRange(old_s, start_addr, size) &&
       !(AnyTargetHartIdInvalid(old_s, hart_mask, hart_mask_base)) &&
       !(RequestFailedForUnspecifiedReason(old_s)))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> IpiSentToAllTargetHarts(new_s, hart_mask, hart_mask_base))
  && (result != SBI_SUCCESS
    ==> TargetHartsInstructedToExecuteHfenceGvma(new_s, hart_mask, hart_mask_base, start_addr, start_addr + size, ALL_GUESTS))
}