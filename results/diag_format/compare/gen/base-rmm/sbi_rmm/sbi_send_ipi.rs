pub open spec fn sbi_send_ipi_spec(result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (Exists(h in HartIdsFromMask(old_s.hart_mask_base, old_s.hart_mask)) :
        (!IsHartEnabledByPlatform(h) || !IsHartAvailableToSupervisor(h))
        ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (RequestFailedForUnspecifiedReason() ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> ForAll(h in HartIdsFromMask(old_s.hart_mask_base, old_s.hart_mask) :
        SupervisorSoftwareInterruptPending(h)))
}