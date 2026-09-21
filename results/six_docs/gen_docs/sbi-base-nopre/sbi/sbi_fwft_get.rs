pub open spec fn sbi_fwft_get_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_NOT_SUPPORTED ==> result.value == 0)
    && (result.error == SBI_ERR_DENIED ==> result.value == 0)
    && (result.error == SBI_ERR_FAILED ==> result.value == 0)
    && (result.error == SBI_SUCCESS ==> result.value != 0)
    && (result.error != SBI_SUCCESS ==> result.value == 0)
    && (result.error == SBI_SUCCESS ==> result.value == old_s.value)
    && (result.error != SBI_SUCCESS ==> new_s == old_s)
}