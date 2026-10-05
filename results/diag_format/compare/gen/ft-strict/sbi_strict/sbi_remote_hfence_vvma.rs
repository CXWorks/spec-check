pub open spec fn sbi_remote_hfence_vvma_spec(hart_mask: unsigned long, hart_mask_base: unsigned long, start_addr: unsigned long, size: unsigned long, result: sbiret.error, old_s: S, new_s: S) -> bool {
  (!RemoteHfenceVvmaImplemented() || (exists|h: HartId| IsTargetHart(h, hart_mask, hart_mask_base) && !HartSupportsHypervisorExtension(h)) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
  && (!IsValidAddress(start_addr) || !IsValidSize(start_addr, size) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
  && (exists|h: HartId| IsTargetHart(h, hart_mask, hart_mask_base) && (!HartEnabledByPlatform(h) || !HartAvailableToSupervisor(h)) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (RequestFailedForUnspecifiedReason() ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> forall|h: HartId| IsTargetHart(h, hart_mask, hart_mask_base) ==> IpiSent(h))
  && (result == SBI_SUCCESS ==> forall|h: HartId| IsTargetHart(h, hart_mask, hart_mask_base) ==> HfenceVvmaExecuted(h, start_addr, start_addr + size, HgatpVmid(CallingHart())))
  && ((!(RemoteHfenceVvmaImplemented() || (exists|h: HartId| IsTargetHart(h, hart_mask, hart_mask_base) && !HartSupportsHypervisorExtension(h))) &&
       IsValidAddress(start_addr) &&
       IsValidSize(start_addr, size) &&
       !(exists|h: HartId| IsTargetHart(h, hart_mask, hart_mask_base) && (!HartEnabledByPlatform(h) || !HartAvailableToSupervisor(h))) &&
       !RequestFailedForUnspecifiedReason())
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> forall|h: HartId| IsTargetHart(h, hart_mask, hart_mask_base) ==> !(IpiSent(h)))
  && (result != SBI_SUCCESS
    ==> forall|h: HartId| IsTargetHart(h, hart_mask, hart_mask_base) ==> !(HfenceVvmaExecuted(h, start_addr, start_addr + size, HgatpVmid(CallingHart()))))
}