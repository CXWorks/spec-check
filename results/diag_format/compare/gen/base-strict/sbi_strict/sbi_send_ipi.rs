pub open spec fn sbi_send_ipi_spec(error: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (exists h: HartId | IsTargetedHart(hart_mask, hart_mask_base, h) && !(IsHartEnabledByPlatform(h) && IsHartAvailableToSupervisor(h)) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (RequestFailedForUnspecifiedReason() ==> ResultEqual(error, SBI_ERR_FAILED))
    && (ResultEqual(error, SBI_SUCCESS) ==> forall h: HartId | IsTargetedHart(hart_mask, hart_mask_base, h) ==> SupervisorSoftwareInterruptPending(h))
}