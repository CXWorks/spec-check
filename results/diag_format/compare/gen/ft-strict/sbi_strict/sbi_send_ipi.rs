pub open spec fn sbi_send_ipi_spec(hart_mask: UInt, hart_mask_base: UInt, error: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (exists h: HartId, IsTargetedHart(hart_mask, hart_mask_base, h) && !(IsHartEnabledByPlatform(h) && IsHartAvailableToSupervisor(h)) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (RequestFailedForUnspecifiedReason(old_s) ==> ResultEqual(error, SBI_ERR_FAILED))
  && (ResultEqual(error, SBI_SUCCESS) ==> SupervisorSoftwareInterruptPending(new_s))
  && (forall h: HartId, IsTargetedHart(hart_mask, hart_mask_base, h) ==> SupervisorSoftwareInterruptPending(new_s))
  && ((!(exists h: HartId, IsTargetedHart(hart_mask, hart_mask_base, h) && !(IsHartEnabledByPlatform(h) && IsHartAvailableToSupervisor(h))) &&
       !RequestFailedForUnspecifiedReason(old_s))
    ==> ResultEqual(error, SBI_SUCCESS))
}