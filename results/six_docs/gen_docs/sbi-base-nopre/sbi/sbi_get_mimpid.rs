pub open spec fn sbi_get_mimpid_spec(result: u64, old_s: S, new_s: S) -> bool {
    (result == 0)
    || (result >= 0)
    && (old_s == new_s)
}