pub open spec fn sbi_remote_hfence_gvma_spec(fid: UInt64, hart_mask: UInt64, hart_mask_base: UInt64, start_addr: Address, size: UInt64, result: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (!IsFunctionImplemented(old_s, fid) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
  && ((exists (h: HartId), IsTargetHart(old_s, h, hart_mask, hart_mask_base)) && (!HartSupportsHypervisorExtension(old_s, h)) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
  && (!IsValidAddressRange(old_s, start_addr, size) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
  && ((exists (h: HartId), IsTargetHart(old_s, h, hart_mask, hart_mask_base)) && (!IsHartEnabledByPlatform(old_s, h) || !IsHartAvailableToSupervisor(old_s, h)) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (RequestFailedForOtherReason(old_s, hart_mask, hart_mask_base, start_addr, size) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> forall (h: HartId), IsTargetHart(new_s, h, hart_mask, hart_mask_base) ==> IpiSentToHart(new_s, h))
  && (result == SBI_SUCCESS ==> forall (h: HartId), IsTargetHart(new_s, h, hart_mask, hart_mask_base) ==> HfenceGvmaExecuted(new_s, h, start_addr, start_addr + size))
  && ((!(IsFunctionImplemented(old_s, fid)) &&
       ((!(exists (h: HartId), IsTargetHart(old_s, h, hart_mask, hart_mask_base)) || HartSupportsHypervisorExtension(old_s, h))) &&
       IsValidAddressRange(old_s, start_addr, size) &&
       !((exists (h: HartId), IsTargetHart(old_s, h, hart_mask, hart_mask_base)) && (!IsHartEnabledByPlatform(old_s, h) || !IsHartAvailableToSupervisor(old_s, h))) &&
       !(RequestFailedForOtherReason(old_s, hart_mask, hart_mask_base, start_addr, size)))
    ==> ResultEqual(result, SBI_SUCCESS))
  && (result != SBI_SUCCESS
    ==> forall (h: HartId), IsTargetHart(new_s, h, hart_mask, hart_mask_base) ==> !IpiSentToHart(new_s, h))
  && (result != SBI_SUCCESS
    ==> forall (h: HartId), IsTargetHart(new_s, h, hart_mask, hart_mask_base) ==> !HfenceGvmaExecuted(new_s, h, start_addr, start_addr + size))
}