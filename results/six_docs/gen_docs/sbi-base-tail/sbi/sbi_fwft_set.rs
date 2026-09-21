pub open spec fn sbi_fwft_set_spec(result: int, old_s: S, new_s: S, feature: UInt32, value: UInt64, flags: UInt64) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (feature == 0 || (flags & 1) != 0))
    && (result == SBI_SBI_ERR_INVALID_STATE ==> (value != old_s.feature_value(feature) || (flags & 1) != 0))
    && (result == SBI_SBI_ERR_DENIED ==> (flags & 1) != 0 && (old_s.feature_locked(feature) || (flags & 1) == 0))
    && (result == SBI_SBI_ERR_FAILED ==> true)
    && (result == SBI_SBI_SUCCESS ==> (new_s.feature_value(feature) == value && (flags & 1) == 0 || new_s.feature_locked(feature)))
    && (result == SBI_SBI_SUCCESS ==> (flags & (1u64 << 63)) == 0)
    && (result != SBI_SBI_SUCCESS && result != SBI_SBI_ERR_INVALID_PARAM && result != SBI_SBI_ERR_INVALID_STATE && result != SBI_SBI_ERR_DENIED && result != SBI_SBI_ERR_FAILED ==> true)
}