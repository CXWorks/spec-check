pub open spec fn sbi_remote_sfence_vma_asid_spec(result: int, hart_mask: UInt64, hart_mask_base: UInt64, start_addr: UInt64, size: UInt64, asid: UInt64, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_ADDRESS ==> (start_addr as int < 0 || size as int < 0 || start_addr as int + size as int < start_addr as int))
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> (asid as int < 0 || hart_mask as int < 0 || hart_mask_base as int < 0))
    && (result == SBI_SBI_ERR_FAILED ==> true)
    && (result == SBI_SBI_SUCCESS ==> true)
    && (result != SBI_SBI_SUCCESS && result != SBI_SBI_ERR_INVALID_ADDRESS && result != SBI_SBI_ERR_INVALID_PARAM && result != SBI_SBI_ERR_FAILED ==> false)
}