pub open spec fn sbi_remote_sfence_vma_asid_spec(result: long, hart_mask: unsigned long, hart_mask_base: unsigned long, start_addr: unsigned long, size: unsigned long, asid: unsigned long, old_s: S, new_s: S) -> bool {
    (!IsValidAddressRange(start_addr, size) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
    && (!IsValidAsid(asid) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (exists|h: HartId| IsTargetedHart(hart_mask, hart_mask_base, h) && (!IsHartEnabledByPlatform(h) || !IsHartAvailableToSupervisor(h)) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (RequestFailedForUnspecifiedReason(hart_mask, hart_mask_base, start_addr, size, asid) ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> (forall|h: HartId| IsTargetedHart(hart_mask, hart_mask_base, h) ==> (IpiSentToHart(h) && SfenceVmaAsidRequested(h, start_addr, start_addr + size, asid))))
    && (forall|h: HartId| IsTargetedHart(hart_mask, hart_mask_base, h) ==> AddressTranslationCache(h, start_addr, start_addr + size, asid))
}