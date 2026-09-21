pub open spec fn sbi_remote_sfence_vma_spec(result: int, old_s: S, new_s: S) -> bool {
    ((result == SBI_SBI_ERR_INVALID_ADDRESS) ==> ((old_s.hart_mask == 0) || (old_s.hart_mask_base == 0) || (old_s.start_addr as int) < 0 || (old_s.size as int) < 0 || (old_s.start_addr as int) + (old_s.size as int) < (old_s.start_addr as int)))
    && (result == SBI_SBI_SUCCESS ==> true)
}