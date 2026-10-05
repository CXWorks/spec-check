pub open spec fn sbi_remote_hfence_gvma_spec(hart_mask: UInt64, hart_mask_base: UInt64, start_addr: UInt64, size: UInt64, result: SbiRet, old_s: S, new_s: S) -> bool {
    ((!IsRemoteHfenceGvmaImplemented(old_s) || !AllTargetHartsSupportHypervisor(old_s, hart_mask, hart_mask_base)) ==> result.error != SBI_SUCCESS)
    && (!IsValidGuestPhysAddrRange(old_s, start_addr, size) ==> result.error != SBI_SUCCESS)
    && (!AllTargetHartIdsValid(old_s, hart_mask, hart_mask_base) ==> result.error != SBI_SUCCESS)
    && (result.error == SBI_ERR_NOT_SUPPORTED ==> ((!IsRemoteHfenceGvmaImplemented(old_s) || !AllTargetHartsSupportHypervisor(old_s, hart_mask, hart_mask_base)) && new_s == old_s))
    && (result.error == SBI_ERR_INVALID_ADDRESS ==> (!IsValidGuestPhysAddrRange(old_s, start_addr, size) && new_s == old_s))
    && (result.error == SBI_ERR_INVALID_PARAM ==> (!AllTargetHartIdsValid(old_s, hart_mask, hart_mask_base) && new_s == old_s))
    && ((IsRemoteHfenceGvmaImplemented(old_s)
        && AllTargetHartsSupportHypervisor(old_s, hart_mask, hart_mask_base)
        && IsValidGuestPhysAddrRange(old_s, start_addr, size)
        && AllTargetHartIdsValid(old_s, hart_mask, hart_mask_base))
        ==> (result.error == SBI_SUCCESS || result.error == SBI_ERR_FAILED))
    && (result.error == SBI_SUCCESS ==> (
        IsRemoteHfenceGvmaImplemented(old_s)
        && AllTargetHartsSupportHypervisor(old_s, hart_mask, hart_mask_base)
        && IsValidGuestPhysAddrRange(old_s, start_addr, size)
        && AllTargetHartIdsValid(old_s, hart_mask, hart_mask_base)
        && HfenceGvmaSentToAllTargetHarts(old_s, new_s, hart_mask, hart_mask_base, start_addr, size)))
}
