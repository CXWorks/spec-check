pub open spec fn sbi_remote_hfence_gvma_spec(result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == SBI_ERR_NOT_SUPPORTED ==> (old_s.hypervisor_extension_supported == false || !old_s.all_harts_in_mask_valid(old_s.hart_mask, old_s.hart_mask_base)))
    && (result == SBI_ERR_INVALID_ADDRESS ==> (old_s.start_addr < 0 || old_s.size < 0 || old_s.start_addr + old_s.size < old_s.start_addr))
    && (result == SBI_ERR_INVALID_PARAM ==> !old_s.all_harts_in_mask_valid(old_s.hart_mask, old_s.hart_mask_base))
    && (result == SBI_ERR_FAILED ==> true)
    && (result == SBI_SUCCESS ==> (old_s.hypervisor_extension_supported == true && old_s.all_harts_in_mask_valid(old_s.hart_mask, old_s.hart_mask_base) && old_s.start_addr >= 0 && old_s.size >= 0 && old_s.start_addr + old_s.size >= old_s.start_addr))
}