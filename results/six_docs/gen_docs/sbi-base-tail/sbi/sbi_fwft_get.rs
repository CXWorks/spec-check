pub open spec fn sbi_fwft_get_spec(result: int, feature: UInt32, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_NOT_SUPPORTED ==> feature != 0)
    && (result == SBI_SBI_ERR_DENIED ==> feature != 0)
    && (result == SBI_SBI_ERR_FAILED ==> feature != 0)
    && (result == SBI_SBI_SUCCESS ==> true)
    && (result != SBI_SBI_SUCCESS ==> new_s == old_s)
}