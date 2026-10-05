pub open spec fn sbi_send_ipi_spec(hart_mask: unsigned long, hart_mask_base: unsigned long, result: struct sbiret, old_s: S, new_s: S) -> bool {
  (Exists(h in HartIdsFromMask(hart_mask_base, hart_mask)) : !IsHartEnabledByPlatform(old_s, h) || !IsHartAvailableToSupervisor(old_s, h) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (RequestFailedForUnspecifiedReason(old_s) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> ForAll(h in HartIdsFromMask(hart_mask_base, hart_mask)) : SupervisorSoftwareInterruptPending(new_s, h))
  && ((!Exists(h in HartIdsFromMask(hart_mask_base, hart_mask)) : !IsHartEnabledByPlatform(old_s, h) || !IsHartAvailableToSupervisor(old_s, h)) &&
       !RequestFailedForUnspecifiedReason(old_s))
  ==> ResultEqual(result, SBI_SUCCESS)
}