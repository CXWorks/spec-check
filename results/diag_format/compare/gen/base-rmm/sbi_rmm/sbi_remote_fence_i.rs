pub open spec fn sbi_remote_fence_i_spec(result: sbiret, hart_mask: UInt64, hart_mask_base: UInt64, old_s: S, new_s: S) -> bool {
    (Exists(hartid in HartsFromMask(hart_mask, hart_mask_base) : !IsHartEnabledByPlatform(hartid) || !IsHartAvailableToSupervisor(hartid)) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (RequestFailedForUnspecifiedReason() ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> (ForAll(hartid in HartsFromMask(hart_mask, hart_mask_base) : IpiSentTo(hartid))))
}