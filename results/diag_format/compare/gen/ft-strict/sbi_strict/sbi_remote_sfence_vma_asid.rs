pub open spec fn sbi_remote_sfence_vma_asid_spec(hart_mask: unsigned long, hart_mask_base: unsigned long, start_addr: unsigned long, size: unsigned long, asid: unsigned long, result: long, old_s: S, new_s: S) -> bool {
  (!IsValidAddressRange(old_s, start_addr, size) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
  && (!IsValidAsid(old_s, asid) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (exists|h: HartId| IsTargetedHart(old_s, hart_mask, hart_mask_base, h) && (!IsHartEnabledByPlatform(old_s, h) || !IsHartAvailableToSupervisor(old_s, h)) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (RequestFailedForUnspecifiedReason(old_s, hart_mask, hart_mask_base, start_addr, size, asid) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> forall|h: HartId| IsTargetedHart(old_s, hart_mask, hart_mask_base, h) ==> IpiSentToHart(new_s, h))
  && (result == SBI_SUCCESS ==> forall|h: HartId| IsTargetedHart(old_s, hart_mask, hart_mask_base, h) ==> SfenceVmaAsidRequested(new_s, h, start_addr, start_addr + size, asid))
  && ((IsValidAddressRange(old_s, start_addr, size) &&
       IsValidAsid(old_s, asid) &&
       !(exists|h: HartId| IsTargetedHart(old_s, hart_mask, hart_mask_base, h) && (!IsHartEnabledByPlatform(old_s, h) || !IsHartAvailableToSupervisor(old_s, h))) &&
       !RequestFailedForUnspecifiedReason(old_s, hart_mask, hart_mask_base, start_addr, size, asid))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> forall|h: HartId| IsTargetedHart(old_s, hart_mask, hart_mask_base, h) ==> !IpiSentToHart(new_s, h))
  && (result != SBI_SUCCESS
    ==> forall|h: HartId| IsTargetedHart(old_s, hart_mask, hart_mask_base, h) ==> !SfenceVmaAsidRequested(new_s, h, start_addr, start_addr + size, asid))
}