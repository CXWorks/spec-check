pub open spec fn sbi_nacl_probe_feature_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_SUCCESS)
    && (result == SBI_SBI_SUCCESS ==> (old_s == new_s))
    && (result == SBI_SBI_SUCCESS ==> (result == 0 || result == 1))
}