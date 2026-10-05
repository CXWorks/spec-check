pub open spec fn sbi_remote_sfence_vma_asid_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_ADDRESS ==> (start_addr as u64 < 0 || size as u64 < 0 || (start_addr as u64 + size as u64) < start_addr as u64))
    && (result.error == SBI_ERR_INVALID_PARAM ==> (asid as u64 == 0 || hart_mask as u64 == 0 || hart_mask_base as u64 == 0))
    && (result.error == SBI_ERR_FAILED ==> true)
    && (result.error == SBI_SUCCESS ==> true)
}