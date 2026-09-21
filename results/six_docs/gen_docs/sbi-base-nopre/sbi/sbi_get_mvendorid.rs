pub open spec fn sbi_get_mvendorid_spec(result: u64, old_s: S, new_s: S) -> bool {
    (result == 0) || (result >= 0)
}