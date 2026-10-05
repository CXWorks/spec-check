pub open spec fn sbi_remote_sfence_vma_asid_spec(hart_mask: unsigned long, hart_mask_base: unsigned long, start_addr: unsigned long, size: unsigned long, asid: unsigned long, error: long, old_s: S, new_s: S) -> bool {
  (!IsValidAddressRange(old_s, start_addr, size) ==> ResultEqual(error, SBI_ERR_INVALID_ADDRESS))
  && (!IsValidAsid(old_s, asid) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (Exists(hartid in HartsFromMask(old_s, hart_mask, hart_mask_base) : !IsHartEnabledByPlatform(old_s, hartid) || !IsHartAvailableToSupervisor(old_s, hartid)) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (RequestFailedForUnspecifiedReason(old_s) ==> ResultEqual(error, SBI_ERR_FAILED))
  && (ResultEqual(error, SBI_SUCCESS) ==> ResultEqual(error, SBI_SUCCESS))
  && (ResultEqual(error, SBI_SUCCESS) ==> Forall(hartid in HartsFromMask(new_s, hart_mask, hart_mask_base) : IpiSent(new_s, hartid)))
  && (ResultEqual(error, SBI_SUCCESS) ==> Forall(hartid in HartsFromMask(new_s, hart_mask, hart_mask_base) : SfenceVmaRequested(new_s, hartid, start_addr, size, asid)))
  && ((IsValidAddressRange(old_s, start_addr, size) &&
       IsValidAsid(old_s, asid) &&
       !(Exists(hartid in HartsFromMask(old_s, hart_mask, hart_mask_base) : !IsHartEnabledByPlatform(old_s, hartid) || !IsHartAvailableToSupervisor(old_s, hartid))) &&
       !RequestFailedForUnspecifiedReason(old_s))
    ==> ResultEqual(error, SBI_SUCCESS))
  && (result != SBI_SUCCESS
    ==> Forall(hartid in HartsFromMask(new_s, hart_mask, hart_mask_base) : !IpiSent(new_s, hartid)))
  && (result != SBI_SUCCESS
    ==> Forall(hartid in HartsFromMask(new_s, hart_mask, hart_mask_base) : !SfenceVmaRequested(new_s, hartid, start_addr, size, asid)))
}