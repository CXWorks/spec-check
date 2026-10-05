pub open spec fn sbi_remote_hfence_gvma_spec(result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (!IsFunctionImplemented(old_s, SBI_REMOTE_HFENCE_GVMA) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
    && (AnyTargetHartLacksHypervisorExtension(old_s, hart_mask, hart_mask_base) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
    && (!IsValidAddressRange(start_addr, size) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
    && (AnyTargetHartIdInvalid(old_s, hart_mask, hart_mask_base) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (RequestFailedForUnspecifiedReason() ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> IpiSentToAllTargetHarts(hart_mask, hart_mask_base))
    && (ResultEqual(result, SBI_SUCCESS) ==> TargetHartsInstructedToExecuteHfenceGvma(hart_mask, hart_mask_base, start_addr, start_addr + size, ALL_GUESTS))
    && (ResultEqual(result, SBI_SUCCESS) ==> GuestPhysicalTranslations(new_s, TargetHarts(hart_mask, hart_mask_base), start_addr, start_addr + size, ALL_GUESTS))
}