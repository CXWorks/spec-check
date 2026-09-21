pub open spec fn sbi_cppc_write_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (old_s == new_s))
    && (result == SBI_SBI_ERR_NOT_SUPPORTED ==> (old_s == new_s))
    && (result == SBI_SBI_SUCCESS ==> (old_s != new_s))
}