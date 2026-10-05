pub open spec fn sbi_get_mvendorid_spec(error: i64, value: u64, old_s: S, new_s: S) -> bool {
    IsLegalMvendoridValue(old_s, value)
    && new_s == old_s
}
