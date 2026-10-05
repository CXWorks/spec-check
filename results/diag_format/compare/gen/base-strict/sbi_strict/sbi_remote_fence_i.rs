pub open spec fn sbi_remote_fence_i_spec(error: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (exists h: HartId | IsTargetedHart(hart_mask(old_s), hart_mask_base(old_s), h) && (!IsHartEnabledByPlatform(h) || !IsHartAvailableToSupervisor(h)) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (RequestFailedForUnspecifiedReason(hart_mask(old_s), hart_mask_base(old_s)) ==> ResultEqual(error, SBI_ERR_FAILED))
    && (ResultEqual(error, SBI_SUCCESS) ==> forall h: HartId | IsTargetedHart(hart_mask(old_s), hart_mask_base(old_s), h) ==> FenceIIpiSent(h))
}