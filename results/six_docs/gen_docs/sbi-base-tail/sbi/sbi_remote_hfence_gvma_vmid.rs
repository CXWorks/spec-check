pub open spec fn sbi_remote_hfence_gvma_vmid_spec(result: int, hart_mask: UInt64, hart_mask_base: UInt64, start_addr: UInt64, size: UInt64, vmid: UInt64, old_s: S, new_s: S) -> bool {
    (hart_mask == 0 ==> result == SBI_SBI_ERR_INVALID_PARAM)
    && (hart_mask_base == 0 ==> result == SBI_SBI_ERR_INVALID_PARAM)
    && (start_addr == 0 ==> result == SBI_SBI_ERR_INVALID_PARAM)
    && (size == 0 ==> result == SBI_SBI_ERR_INVALID_PARAM)
    && (vmid == 0 ==> result == SBI_SBI_ERR_INVALID_PARAM)
    && (hart_mask >= (1u64 << 64) ==> result == SBI_SBI_ERR_INVALID_PARAM)
    && (hart_mask_base >= (1u64 << 64) ==> result == SBI_SBI_ERR_INVALID_PARAM)
    && (start_addr >= (1u64 << 64) ==> result == SBI_SBI_ERR_INVALID_PARAM)
    && (size >= (1u64 << 64) ==> result == SBI_SBI_ERR_INVALID_PARAM)
    && (vmid >= (1u64 << 64) ==> result == SBI_SBI_ERR_INVALID_PARAM)
    && (hart_mask & hart_mask_base != 0 ==> result == SBI_SBI_ERR_INVALID_PARAM)
    && (result == SBI_SBI_SUCCESS ==> (hart_mask == old_s.hart_mask && hart_mask_base == old_s.hart_mask_base && start_addr == old_s.start_addr && size == old_s.size && vmid == old_s.vmid))
}