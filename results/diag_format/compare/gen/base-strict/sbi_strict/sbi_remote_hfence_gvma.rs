pub open spec fn sbi_remote_hfence_gvma_spec(result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (!IsFunctionImplemented(old_s, 4) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
    && (forall h: HartId | IsTargetHart(h, old_s.hart_mask, old_s.hart_mask_base) ==> (!HartSupportsHypervisorExtension(old_s, h) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED)))
    && (!IsValidAddressRange(old_s, old_s.start_addr, old_s.size) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
    && (forall h: HartId | IsTargetHart(h, old_s.hart_mask, old_s.hart_mask_base) ==> (!IsHartEnabledByPlatform(old_s, h) || !IsHartAvailableToSupervisor(old_s, h) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM)))
    && (RequestFailedForOtherReason(old_s.hart_mask, old_s.hart_mask_base, old_s.start_addr, old_s.size) ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> IpiSentToHart(old_s, old_s.hart_mask, old_s.hart_mask_base, old_s.start_addr, old_s.size))
    && (ResultEqual(result, SBI_SUCCESS) ==> forall h: HartId | IsTargetHart(h, old_s.hart_mask, old_s.hart_mask_base) ==> HfenceGvmaExecuted(old_s, h, old_s.start_addr, old_s.start_addr + old_s.size))
    && (ResultEqual(result, SBI_SUCCESS) ==> forall h: HartId | IsTargetHart(h, old_s.hart_mask, old_s.hart_mask_base) ==> GuestPhysicalTranslationState(old_s, h, old_s.start_addr, old_s.start_addr + old_s.size))
}