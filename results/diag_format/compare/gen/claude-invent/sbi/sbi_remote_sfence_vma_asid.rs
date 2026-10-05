pub open spec fn sbi_remote_sfence_vma_asid_spec(hart_mask: UInt64, hart_mask_base: UInt64, start_addr: UInt64, size: UInt64, asid: UInt64, result: SbiRet, old_s: S, new_s: S) -> bool {
    (result.error == SBI_SUCCESS
        || result.error == SBI_ERR_INVALID_ADDRESS
        || result.error == SBI_ERR_INVALID_PARAM
        || result.error == SBI_ERR_FAILED)
    && (!SfenceAddrRangeValid(old_s, start_addr, size) ==> result.error != SBI_SUCCESS)
    && (!SfenceAsidValid(old_s, asid) ==> result.error != SBI_SUCCESS)
    && (!HartMaskAllValid(old_s, hart_mask, hart_mask_base) ==> result.error != SBI_SUCCESS)
    && (result.error == SBI_ERR_INVALID_ADDRESS ==>
        !SfenceAddrRangeValid(old_s, start_addr, size)
        && new_s == old_s)
    && (result.error == SBI_ERR_INVALID_PARAM ==>
        (!SfenceAsidValid(old_s, asid) || !HartMaskAllValid(old_s, hart_mask, hart_mask_base))
        && new_s == old_s)
    && (result.error == SBI_SUCCESS ==>
        SfenceAddrRangeValid(old_s, start_addr, size)
        && SfenceAsidValid(old_s, asid)
        && HartMaskAllValid(old_s, hart_mask, hart_mask_base)
        && RemoteSfenceVmaAsidIssued(old_s, new_s, hart_mask, hart_mask_base, start_addr, size, asid))
}
