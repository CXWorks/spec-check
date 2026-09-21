pub open spec fn sbi_remote_hfence_gvma_spec(result: int, hart_mask: UInt64, hart_mask_base: UInt64, start_addr: UInt64, size: UInt64, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_NOT_SUPPORTED ==> true)
    && (result == SBI_SBI_ERR_INVALID_ADDRESS ==> (start_addr as int < 0 || size as int < 0))
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> true)
    && (result == SBI_SBI_ERR_FAILED ==> true)
    && (result == SBI_SBI_SUCCESS ==> true)
}