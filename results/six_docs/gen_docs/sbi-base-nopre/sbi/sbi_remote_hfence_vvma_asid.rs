pub open spec fn sbi_remote_hfence_vvma_asid_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (hart_mask == 0 ==> ResultEqual(result, SBI_ERROR_INVALID_ARGS))
    && (hart_mask_base == 0 ==> ResultEqual(result, SBI_ERROR_INVALID_ARGS))
    && (start_addr == 0 ==> ResultEqual(result, SBI_ERROR_INVALID_ARGS))
    && (size == 0 ==> ResultEqual(result, SBI_ERROR_INVALID_ARGS))
    && (asid == 0 ==> ResultEqual(result, SBI_ERROR_INVALID_ARGS))
    && (hart_mask_base > hart_mask ==> ResultEqual(result, SBI_ERROR_INVALID_ARGS))
    && (hart_mask_base + hart_mask > 0x1_0000_0000_0000 ==> ResultEqual(result, SBI_ERROR_INVALID_ARGS))
    && (start_addr + size > 0x1_0000_0000_0000 ==> ResultEqual(result, SBI_ERROR_INVALID_ARGS))
    && (result.is_Ok() ==> (hart_mask > 0) && (hart_mask_base > 0) && (start_addr > 0) && (size > 0) && (asid > 0))
}