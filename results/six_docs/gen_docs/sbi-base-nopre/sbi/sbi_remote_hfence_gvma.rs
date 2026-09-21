pub open spec fn sbi_remote_hfence_gvma_spec(result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == SBI_ERR_NOT_SUPPORTED ==> (hart_mask == 0 || hart_mask_base >= (1u64 << 64) || hart_mask_base >= old_s.hart_count))
    && (result == SBI_ERR_INVALID_ADDRESS ==> (start_addr >= (1u64 << 64) || size >= (1u64 << 64) || start_addr + size < start_addr))
    && (result == SBI_ERR_INVALID_PARAM ==> (hart_mask == 0 || hart_mask_base >= (1u64 << 64) || hart_mask_base >= old_s.hart_count))
    && (result == SBI_ERR_FAILED ==> true)
    && (result == SBI_SUCCESS ==> (hart_mask != 0 && hart_mask_base < old_s.hart_count && start_addr < (1u64 << 64) && size < (1u64 << 64) && start_addr + size <= (1u64 << 64)))
}