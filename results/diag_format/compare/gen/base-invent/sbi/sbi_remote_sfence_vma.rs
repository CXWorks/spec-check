pub open spec fn sbi_remote_sfence_vma_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_ADDRESS ==> (old_s.hart_mask != 0 || old_s.hart_mask_base != 0 || old_s.start_addr as u64 < 0 || old_s.size as u64 < 0))
    && (result.error == SBI_SUCCESS ==> true)
}