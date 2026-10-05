pub open spec fn sbi_remote_hfence_vvma_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_NOT_SUPPORTED ==> (hart_mask == 0 || hart_mask_base >= (1u64 << 64) || start_addr >= (1u64 << 64) || size >= (1u64 << 64)))
    && (result.error == SBI_ERR_INVALID_ADDRESS ==> (start_addr >= (1u64 << 64) || size >= (1u64 << 64)))
    && (result.error == SBI_ERR_INVALID_PARAM ==> (hart_mask == 0 || hart_mask_base >= (1u64 << 64)))
    && (result.error == SBI_ERR_FAILED ==> true)
    && (result.error == SBI_SUCCESS ==> (hart_mask != 0 && hart_mask_base < (1u64 << 64) && start_addr < (1u64 << 64) && size < (1u64 << 64)))
}