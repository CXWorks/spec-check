pub open spec fn sbi_send_ipi_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> <failure postconditions>)
    && (result.error == SBI_ERR_FAILED ==> <failure postconditions>)
    && (result.error == SBI_SUCCESS ==> <success postconditions>)
}