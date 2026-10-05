pub open spec fn sbi_remote_sfence_vma_spec(result: SbiRet, hart_mask: u64, hart_mask_base: u64, start_addr: u64, size: u64, old_s: S, new_s: S) -> bool {
    (!IsValidSfenceVmaAddressRange(old_s, start_addr, size) ==> result.error == SBI_ERR_INVALID_ADDRESS)
    && (IsValidSfenceVmaAddressRange(old_s, start_addr, size) ==> (
        result.error == SBI_SUCCESS
        && RemoteSfenceVmaSentToTargetHarts(old_s, new_s, hart_mask, hart_mask_base, start_addr, size)
    ))
}
