pub open spec fn sbi_fwft_get_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_NOT_SUPPORTED ==> new_s.fwft_value == 0)
    && (result == SBI_SBI_ERR_DENIED ==> new_s.fwft_value == 0)
    && (result == SBI_SBI_ERR_FAILED ==> new_s.fwft_value == 0)
    && (result == SBI_SBI_SUCCESS ==> new_s.fwft_value == old_s.fwft_value)
}