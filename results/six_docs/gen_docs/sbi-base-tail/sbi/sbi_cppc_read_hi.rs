pub open spec fn sbi_cppc_read_hi_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (old_s as int) == 0)
    && (result == SBI_SBI_ERR_NOT_SUPPORTED ==> (old_s as int) == 0)
    && (result == SBI_SBI_ERR_DENIED ==> (old_s as int) == 0)
    && (result == SBI_SBI_ERR_FAILED ==> (old_s as int) == 0)
    && (result == SBI_SBI_SUCCESS ==> (result as int) == 0)
    && (result != SBI_SBI_SUCCESS ==> (result as int) == 0)
}