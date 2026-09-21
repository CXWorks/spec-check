pub open spec fn sbi_sse_enable_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_NOT_SUPPORTED ==> true)
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> true)
    && (result == SBI_SBI_ERR_INVALID_STATE ==> true)
    && (result == SBI_SBI_SUCCESS ==> true)
}