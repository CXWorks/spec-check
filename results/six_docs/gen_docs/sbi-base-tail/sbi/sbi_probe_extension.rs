pub open spec fn sbi_probe_extension_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == 0 ==> true)
    && (result == 1 ==> true)
    && (result != 0 && result != 1 ==> true)
    && (old_s == new_s)
}