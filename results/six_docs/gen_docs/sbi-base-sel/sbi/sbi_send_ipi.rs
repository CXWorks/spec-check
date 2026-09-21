pub open spec fn sbi_send_ipi_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> <failure postconditions>)
    && (result == SBI_SBI_ERR_FAILED ==> <failure postconditions>)
    && (result == SBI_SBI_SUCCESS ==> <success postconditions>)
}