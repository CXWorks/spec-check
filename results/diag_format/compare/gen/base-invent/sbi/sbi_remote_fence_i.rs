pub open spec fn sbi_remote_fence_i_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> <hart_mask and hart_mask_base imply invalid hartid>)
    && (result.error == SBI_ERR_FAILED ==> true)
    && (result.error == SBI_SUCCESS ==> true)
}