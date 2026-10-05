pub open spec fn sbi_send_ipi_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> <hart_mask and hart_mask_base imply invalid hartid>)
    && (result.error == SBI_ERR_FAILED ==> <request failed for unspecified reasons>)
    && (result.error == SBI_SUCCESS ==> <hart_mask and hart_mask_base imply valid harts and IPI sent>)
}