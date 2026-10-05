pub open spec fn sbi_remote_hfence_vvma_spec(ret: SbiRet, old_s: S, new_s: S, hart_mask: UInt64, hart_mask_base: UInt64, start_addr: UInt64, size: UInt64) -> bool {
    (ret.error == SBI_SUCCESS
        || ret.error == SBI_ERR_NOT_SUPPORTED
        || ret.error == SBI_ERR_INVALID_ADDRESS
        || ret.error == SBI_ERR_INVALID_PARAM
        || ret.error == SBI_ERR_FAILED)
    && ((!SbiRemoteHfenceVvmaImplemented(old_s)
            || !AllTargetHartsSupportHypervisorExtension(old_s, hart_mask, hart_mask_base))
        ==> ret.error != SBI_SUCCESS)
    && (ret.error == SBI_ERR_NOT_SUPPORTED
        ==> (!SbiRemoteHfenceVvmaImplemented(old_s)
                || !AllTargetHartsSupportHypervisorExtension(old_s, hart_mask, hart_mask_base))
            && new_s == old_s)
    && (!IsValidGuestVirtualAddressRange(old_s, start_addr, size)
        ==> ret.error != SBI_SUCCESS)
    && (ret.error == SBI_ERR_INVALID_ADDRESS
        ==> !IsValidGuestVirtualAddressRange(old_s, start_addr, size)
            && new_s == old_s)
    && (!AllTargetHartIdsValid(old_s, hart_mask, hart_mask_base)
        ==> ret.error != SBI_SUCCESS)
    && (ret.error == SBI_ERR_INVALID_PARAM
        ==> !AllTargetHartIdsValid(old_s, hart_mask, hart_mask_base)
            && new_s == old_s)
    && ((SbiRemoteHfenceVvmaImplemented(old_s)
            && AllTargetHartsSupportHypervisorExtension(old_s, hart_mask, hart_mask_base)
            && IsValidGuestVirtualAddressRange(old_s, start_addr, size)
            && AllTargetHartIdsValid(old_s, hart_mask, hart_mask_base))
        ==> (ret.error == SBI_SUCCESS || ret.error == SBI_ERR_FAILED))
    && (ret.error == SBI_SUCCESS
        ==> SbiRemoteHfenceVvmaImplemented(old_s)
            && AllTargetHartsSupportHypervisorExtension(old_s, hart_mask, hart_mask_base)
            && IsValidGuestVirtualAddressRange(old_s, start_addr, size)
            && AllTargetHartIdsValid(old_s, hart_mask, hart_mask_base)
            && HfenceVvmaIpiSentToAllTargetHarts(old_s, new_s, hart_mask, hart_mask_base, start_addr, size, CallingHartHgatpVmid(old_s)))
}
