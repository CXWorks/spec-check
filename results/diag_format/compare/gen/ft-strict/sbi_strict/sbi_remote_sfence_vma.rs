pub open spec fn sbi_remote_sfence_vma_spec(hart_mask: unsigned long, hart_mask_base: unsigned long, start_addr: unsigned long, size: unsigned long, error: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (!IsValidAddressRange(old_s, start_addr, size) ==> ResultEqual(error, SBI_ERR_INVALID_ADDRESS))
  && (result ==> ResultEqual(error, SBI_SUCCESS))
  && (result ==> forall h: HartId| IsTargetedHart(hart_mask, hart_mask_base, h) ==> IpiSent(h))
  && (result ==> forall h: HartId| IsTargetedHart(hart_mask, hart_mask_base, h) ==> SfenceVmaExecuted(h, start_addr, start_addr + size))
  && ((IsValidAddressRange(old_s, start_addr, size))
    ==> ResultEqual(error, SBI_SUCCESS))
  && (result
    ==> forall h: HartId| IsTargetedHart(hart_mask, hart_mask_base, h) ==> AddressTranslationCache(new_s))
  && ((!(IsValidAddressRange(old_s, start_addr, size)))
    ==> ResultEqual(error, SBI_ERR_INVALID_ADDRESS))
}