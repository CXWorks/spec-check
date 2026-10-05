pub open spec fn sdei_shared_reset_spec(result: i64, old_s: S, new_s: S) -> bool {
    (result == 0i64 || result == -1i64 || result == -3i64)
}
