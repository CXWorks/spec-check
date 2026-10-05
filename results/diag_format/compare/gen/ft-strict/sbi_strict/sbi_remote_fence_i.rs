pub open spec fn sbi_remote_fence_i_spec(hart_mask: UInt64, hart_mask_base: UInt64, error: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (exists h: HartId, IsTargetedHart(hart_mask, hart_mask_base, h) && (!IsHartEnabledByPlatform(h) || !IsHartAvailableToSupervisor(h)) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (RequestFailedForUnspecifiedReason(hart_mask, hart_mask_base) ==> ResultEqual(error, SBI_ERR_FAILED))
  && (ResultEqual(error, SBI_SUCCESS) ==> ResultEqual(error, SBI_SUCCESS))
  && (ResultEqual(error, SBI_SUCCESS) ==> forall h: HartId, IsTargetedHart(hart_mask, hart_mask_base, h) ==> FenceIIpiSent(new_s))
  && ((!((exists h: HartId, IsTargetedHart(hart_mask, hart_mask_base, h) && (!IsHartEnabledByPlatform(h) || !IsHartAvailableToSupervisor(h)))) &&
       !RequestFailedForUnspecifiedReason(hart_mask, hart_mask_base))
    ==> ResultEqual(error, SBI_SUCCESS))
}