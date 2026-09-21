pub open spec fn sbi_remote_sfence_vma_spec(result: int, old_s: S, new_s: S) -> bool {
    ((start_addr as int) < 0 || (size as int) < 0 ==> result == SBI_SBI_ERR_INVALID_ADDRESS)
    && (result == SBI_SBI_SUCCESS ==> true)
}