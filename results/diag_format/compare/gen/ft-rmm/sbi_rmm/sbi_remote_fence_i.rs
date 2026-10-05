pub open spec fn sbi_remote_fence_i_spec(hart_mask: unsigned long, hart_mask_base: unsigned long, result: sbiret.error, old_s: S, new_s: S) -> bool {
  (Exists(hartid in HartsFromMask(old_s, hart_mask, hart_mask_base) : (!IsHartEnabledByPlatform(old_s, hartid) || !IsHartAvailableToSupervisor(old_s, hartid))) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (RequestFailedForUnspecifiedReason(old_s) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> ForAll(hartid in HartsFromMask(old_s, hart_mask, hart_mask_base) : IpiSentTo(new_s, hartid)))
  && ((!Exists(hartid in HartsFromMask(old_s, hart_mask, hart_mask_base) : (!IsHartEnabledByPlatform(old_s, hartid) || !IsHartAvailableToSupervisor(old_s, hartid))) &&
       !RequestFailedForUnspecifiedReason(old_s))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> ForAll(hartid in HartsFromMask(old_s, hart_mask, hart_mask_base) : !(IpiSentTo(new_s, hartid))))
}