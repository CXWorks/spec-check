pub open spec fn sbi_remote_hfence_gvma_vmid_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (hart_mask == 0 ==> ResultEqual(result, SBI_ERROR_INVALID_ARGS))
    && (hart_mask_base == 0 ==> ResultEqual(result, SBI_ERROR_INVALID_ARGS))
    && (start_addr == 0 ==> ResultEqual(result, SBI_ERROR_INVALID_ARGS))
    && (size == 0 ==> ResultEqual(result, SBI_ERROR_INVALID_ARGS))
    && (vmid == 0 ==> ResultEqual(result, SBI_ERROR_INVALID_ARGS))
    && (hart_mask > 0 && hart_mask_base > 0 && start_addr > 0 && size > 0 && vmid > 0 ==> result.is_Ok())
}