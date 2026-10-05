pub open spec fn sbi_remote_fence_i_spec(hart_mask: UInt64, hart_mask_base: UInt64, result: SbiRet, old_s: S, new_s: S) -> bool {
    (!SbiHartMaskAllValid(old_s, hart_mask, hart_mask_base) ==> (result.error == SBI_ERR_INVALID_PARAM && new_s == old_s))
    && (SbiHartMaskAllValid(old_s, hart_mask, hart_mask_base) ==> (result.error == SBI_SUCCESS || result.error == SBI_ERR_FAILED))
    && (result.error == SBI_SUCCESS ==> (SbiHartMaskAllValid(old_s, hart_mask, hart_mask_base) && SbiRemoteFenceIIpiSentToAll(old_s, new_s, hart_mask, hart_mask_base)))
    && ((result.error == SBI_SUCCESS || result.error == SBI_ERR_INVALID_PARAM || result.error == SBI_ERR_FAILED))
}
