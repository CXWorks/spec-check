pub open spec fn sbi_remote_sfence_vma_asid_spec(error: long, hart_mask: UInt, hart_mask_base: UInt, start_addr: UInt, size: UInt, asid: UInt, old_s: S, new_s: S) -> bool {
    (!IsValidAddressRange(start_addr, size) ==> ResultEqual(error, SBI_ERR_INVALID_ADDRESS))
    && (!IsValidAsid(asid) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (Exists(hartid in HartsFromMask(hart_mask, hart_mask_base) : !IsHartEnabledByPlatform(hartid) || !IsHartAvailableToSupervisor(hartid)) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (RequestFailedForUnspecifiedReason() ==> ResultEqual(error, SBI_ERR_FAILED))
    && (ResultEqual(error, SBI_SUCCESS) ==> (IpiSent(hartid) && SfenceVmaRequested(hartid, start_addr, size, asid) for all hartid in HartsFromMask(hart_mask, hart_mask_base)))
}