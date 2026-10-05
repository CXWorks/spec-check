pub open spec fn sbi_remote_hfence_vvma_spec(error: sbiret, hart_mask: u64, hart_mask_base: u64, start_addr: u64, size: u64, old_s: S, new_s: S) -> bool {
    (!IsFunctionImplemented(RemoteHfenceVvma) ==> ResultEqual(error, SBI_ERR_NOT_SUPPORTED))
    && (!AllTargetHartsImplementHypervisorExt(hart_mask, hart_mask_base) ==> ResultEqual(error, SBI_ERR_NOT_SUPPORTED))
    && (!IsValidAddressRange(start_addr, size) ==> ResultEqual(error, SBI_ERR_INVALID_ADDRESS))
    && (!AllHartIdsValid(hart_mask, hart_mask_base) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (RequestFailedForUnspecifiedReason() ==> ResultEqual(error, SBI_ERR_FAILED))
    && (ResultEqual(error, SBI_SUCCESS) ==> IpiSentToAllTargetHarts(hart_mask, hart_mask_base))
    && (ResultEqual(error, SBI_SUCCESS) ==> TargetHartsExecutedHfenceVvma(hart_mask, hart_mask_base, start_addr, start_addr + size, CurrentVmid(hgatp)))
}