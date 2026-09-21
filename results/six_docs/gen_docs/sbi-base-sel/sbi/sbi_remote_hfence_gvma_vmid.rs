pub open spec fn sbi_remote_hfence_gvma_vmid_spec(result: int, old_s: S, new_s: S) -> bool {
    (hart_mask == 0 ==> ResultEqual(result, SBI_SBI_ERR_INVALID_PARAM))
    && (hart_mask_base == 0 ==> ResultEqual(result, SBI_SBI_ERR_INVALID_PARAM))
    && (start_addr == 0 ==> ResultEqual(result, SBI_SBI_ERR_INVALID_PARAM))
    && (size == 0 ==> ResultEqual(result, SBI_SBI_ERR_INVALID_PARAM))
    && (vmid == 0 ==> ResultEqual(result, SBI_SBI_ERR_INVALID_PARAM))
    && (hart_mask_base > hart_mask ==> ResultEqual(result, SBI_SBI_ERR_INVALID_PARAM))
    && (hart_mask_base + hart_mask > 0x1_0000_0000_0000 ==> ResultEqual(result, SBI_SBI_ERR_INVALID_PARAM))
    && (start_addr + size > 0x1_0000_0000_0000 ==> ResultEqual(result, SBI_SBI_ERR_INVALID_PARAM))
    && (result == SBI_SBI_SUCCESS ==> (hart_mask > 0 && hart_mask_base > 0 && start_addr > 0 && size > 0 && vmid > 0 && hart_mask_base <= hart_mask && hart_mask_base + hart_mask <= 0x1_0000_0000_0000 && start_addr + size <= 0x1_0000_0000_0000))
}