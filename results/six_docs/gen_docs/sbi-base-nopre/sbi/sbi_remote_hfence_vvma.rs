pub open spec fn sbi_remote_hfence_vvma_spec(result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == SBI_ERR_NOT_SUPPORTED ==> (old_s.hypervisor_extension_supported == false || !old_s.all_target_harts_support_hypervisor_extension))
    && (result == SBI_ERR_INVALID_ADDRESS ==> (old_s.start_addr as int < 0 || (old_s.start_addr as int + old_s.size as int) < old_s.start_addr as int))
    && (result == SBI_ERR_INVALID_PARAM ==> !old_s.all_constructed_hartids_valid)
    && (result == SBI_ERR_FAILED ==> true)
    && (result == SBI_SUCCESS ==> (old_s.start_addr == new_s.start_addr && old_s.size == new_s.size && old_s.hart_mask == new_s.hart_mask && old_s.hart_mask_base == new_s.hart_mask_base))
}