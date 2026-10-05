pub open spec fn sbi_remote_sfence_vma_spec(error: SbiErrorCode, hart_mask: UInt, hart_mask_base: UInt, start_addr: UInt, size: UInt, old_s: S, new_s: S) -> bool {
    (!IsValidAddressRange(start_addr, size) ==> ResultEqual(error, SBI_ERR_INVALID_ADDRESS))
    && (ResultEqual(error, SBI_SUCCESS) ==> (forall|h: HartId| IsTargetedHart(hart_mask, hart_mask_base, h) ==> (IpiSent(h) && SfenceVmaExecuted(h, start_addr, start_addr + size))))
    && (forall|h: HartId| IsTargetedHart(hart_mask, hart_mask_base, h) ==> AddressTranslationCache(h))
}