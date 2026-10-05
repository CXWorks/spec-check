pub open spec fn sbi_remote_hfence_vvma_spec(result: sbiret.error, hart_mask: UInt, hart_mask_base: UInt, start_addr: UInt, size: UInt, old_s: S, new_s: S) -> bool {
    (!RemoteHfenceVvmaImplemented() ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
    && ((exists|h: HartId| IsTargetHart(h, hart_mask, hart_mask_base) && !HartSupportsHypervisorExtension(h)) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
    && (!IsValidAddress(start_addr) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
    && (!IsValidSize(start_addr, size) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
    && ((exists|h: HartId| IsTargetHart(h, hart_mask, hart_mask_base) && (!HartEnabledByPlatform(h) || !HartAvailableToSupervisor(h))) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (RequestFailedForUnspecifiedReason() ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> (forall|h: HartId| IsTargetHart(h, hart_mask, hart_mask_base) ==> (IpiSent(h) && HfenceVvmaExecuted(h, start_addr, start_addr + size, HgatpVmid(CallingHart())))))
}